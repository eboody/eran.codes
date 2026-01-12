use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use dashmap::DashMap;
use tracing::{Event, Level};
use tracing_subscriber::{Layer, registry::LookupSpan};

use crate::{request, sse, views};
use maud::Render;

#[derive(Clone, Debug)]
pub struct Entry {
    pub timestamp: String,
    pub level: String,
    pub target: String,
    pub message: String,
    pub fields: Vec<(String, String)>,
}

#[derive(Clone)]
pub struct Store {
    inner: Arc<DashMap<String, VecDeque<Entry>>>,
    global: Arc<Mutex<VecDeque<Entry>>>,
    max_entries: usize,
    sse: sse::Registry,
}

impl Store {
    pub fn new(sse: sse::Registry) -> Self {
        Self {
            inner: Arc::new(DashMap::new()),
            global: Arc::new(Mutex::new(VecDeque::new())),
            max_entries: 50,
            sse,
        }
    }

    pub fn record(
        &self,
        request_id: &str,
        entry: Entry,
    ) {
        self.record_with_session(request_id, None, entry);
    }

    pub fn record_with_session(
        &self,
        request_id: &str,
        session_id: Option<&str>,
        entry: Entry,
    ) {
        let mut queue = self
            .inner
            .entry(request_id.to_string())
            .or_insert_with(VecDeque::new);
        if queue.len() >= self.max_entries {
            queue.pop_front();
        }
        queue.push_back(entry);

        if let Ok(mut global) = self.global.lock() {
            if global.len() >= self.max_entries {
                global.pop_front();
            }
            global.push_back(queue.back().cloned().expect("entry"));
        }

        if let Some(session_id) = session_id {
            let markup = views::partials::LiveLog {
                entries: &self.snapshot_global(),
            }
            .render()
            .into_string();
            let _ = self
                .sse
                .send_by_id(session_id, sse::Event::patch_elements(markup));
        }
    }

    pub fn snapshot(
        &self,
        request_id: &str,
    ) -> Vec<Entry> {
        self.inner
            .get(request_id)
            .map(|queue| queue.iter().cloned().collect())
            .unwrap_or_default()
    }

    pub fn snapshot_session(
        &self,
        session_id: &str,
    ) -> Vec<Entry> {
        self.inner
            .get(session_id)
            .map(|queue| queue.iter().cloned().collect())
            .unwrap_or_default()
    }

    pub fn snapshot_global(&self) -> Vec<Entry> {
        self.global
            .lock()
            .map(|value| value.iter().cloned().collect())
            .unwrap_or_default()
    }
}

pub struct TraceLogLayer {
    store: Store,
}

impl TraceLogLayer {
    pub fn new(store: Store) -> Self {
        Self { store }
    }
}

impl<S> Layer<S> for TraceLogLayer
where
    S: tracing::Subscriber + for<'a> LookupSpan<'a>,
{
    fn on_event(
        &self,
        event: &Event<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        let context = request::current_context();
        let Some(request_id) = context
            .as_ref()
            .and_then(|value| value.request_id.as_deref())
        else {
            return;
        };

        let mut visitor = FieldVisitor::default();
        event.record(&mut visitor);

        let level = *event.metadata().level();
        let has_db = visitor
            .fields
            .iter()
            .any(|(name, _)| name == "db_statement");
        let is_demo = event.metadata().target().starts_with("demo");
        let is_info = matches!(level, Level::INFO | Level::WARN | Level::ERROR);

        if !(is_info || has_db || is_demo) {
            return;
        }

        let message = visitor
            .message
            .unwrap_or_else(|| event.metadata().name().to_string());

        let entry = Entry {
            timestamp: jiff::Timestamp::now().to_string(),
            level: level.to_string(),
            target: event.metadata().target().to_string(),
            message,
            fields: visitor.fields,
        };

        let session_id = context
            .as_ref()
            .and_then(|value| value.session_id.as_deref());
        self.store
            .record_with_session(request_id, session_id, entry);
    }
}

#[derive(Default)]
struct FieldVisitor {
    fields: Vec<(String, String)>,
    message: Option<String>,
}

impl tracing::field::Visit for FieldVisitor {
    fn record_debug(
        &mut self,
        field: &tracing::field::Field,
        value: &dyn core::fmt::Debug,
    ) {
        let value = format!("{value:?}");
        if field.name() == "message" {
            self.message = Some(value);
        } else {
            self.fields
                .push((field.name().to_string(), value));
        }
    }
}

pub async fn audit_middleware(
    State(state): State<crate::State>,
    req: Request<Body>,
    next: Next,
) -> Response {
    let request_id = request::current_context()
        .and_then(|value| value.request_id)
        .unwrap_or_else(|| "unknown".to_string());
    let session_id = request::current_context()
        .and_then(|value| value.session_id);

    state.trace_log.record_with_session(
        &request_id,
        session_id.as_deref(),
        Entry {
            timestamp: jiff::Timestamp::now().to_string(),
            level: "INFO".to_string(),
            target: "demo.request".to_string(),
            message: "request.start".to_string(),
            fields: vec![
                ("method".to_string(), req.method().to_string()),
                ("path".to_string(), req.uri().path().to_string()),
                ("request_id".to_string(), request_id.clone()),
            ],
        },
    );

    let response = next.run(req).await;

    state.trace_log.record_with_session(
        &request_id,
        session_id.as_deref(),
        Entry {
            timestamp: jiff::Timestamp::now().to_string(),
            level: "INFO".to_string(),
            target: "demo.request".to_string(),
            message: "request.end".to_string(),
            fields: vec![
                ("status".to_string(), response.status().as_u16().to_string()),
                ("request_id".to_string(), request_id.clone()),
            ],
        },
    );

    if response.status() == StatusCode::INTERNAL_SERVER_ERROR {
        tracing::error!(target: "demo.request", "response error");
    }

    response
}
