use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
    time::Instant,
};

use axum::{
    body::Body,
    extract::Extension,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use dashmap::DashMap;
use tracing::{Event, Level};
use tracing_subscriber::{Layer, registry::LookupSpan};

use crate::{request, sse, views};
use bon::{bon, Builder};
use maud::Render;

#[derive(Clone, Debug, Builder)]
pub struct Entry {
    pub timestamp: String,
    pub level: String,
    pub target: String,
    pub message: String,
    pub fields: Vec<(String, String)>,
}

#[derive(Clone)]
pub struct Store {
    requests: Arc<DashMap<String, VecDeque<Entry>>>,
    sessions: Arc<DashMap<String, VecDeque<Entry>>>,
    global: Arc<Mutex<VecDeque<Entry>>>,
    max_entries: usize,
    sse: sse::Registry,
}

impl Store {
    pub fn new(
        sse: sse::Registry,
        max_entries: usize,
    ) -> Self {
        Self {
            requests: Arc::new(DashMap::new()),
            sessions: Arc::new(DashMap::new()),
            global: Arc::new(Mutex::new(VecDeque::new())),
            max_entries,
            sse,
        }
    }

    pub fn record_with_session(
        &self,
        request_id: &str,
        session_id: Option<&str>,
        entry: Entry,
    ) {
        let mut queue = self
            .requests
            .entry(request_id.to_string())
            .or_insert_with(VecDeque::new);
        if queue.len() >= self.max_entries {
            queue.pop_front();
        }
        queue.push_back(entry);

        let entry = queue.back().cloned().expect("entry");

        if let Some(session_id) = session_id {
            let mut session_queue = self
                .sessions
                .entry(session_id.to_string())
                .or_insert_with(VecDeque::new);
            if session_queue.len() >= self.max_entries {
                session_queue.pop_front();
            }
            session_queue.push_back(entry.clone());
        }

        if let Ok(mut global) = self.global.lock() {
            if global.len() >= self.max_entries {
                global.pop_front();
            }
            global.push_back(entry);
        }

        if let Some(session_id) = session_id {
            let entries = self.snapshot_session(session_id);
            let live_log = views::partials::LiveLog::builder()
                .entries(&entries)
                .build()
                .render()
                .into_string();
            let network_log = views::partials::NetworkLog::builder()
                .entries(&entries)
                .build()
                .render()
                .into_string();
            let _ = self
                .sse
                .send_by_id(session_id, sse::Event::patch_elements(live_log));
            let _ = self
                .sse
                .send_by_id(session_id, sse::Event::patch_elements(network_log));
        }
    }

    pub fn snapshot_request(
        &self,
        request_id: &str,
    ) -> Vec<Entry> {
        self.requests
            .get(request_id)
            .map(|queue| queue.iter().cloned().collect())
            .unwrap_or_default()
    }

    pub fn snapshot_session(
        &self,
        session_id: &str,
    ) -> Vec<Entry> {
        self.sessions
            .get(session_id)
            .map(|queue| queue.iter().cloned().collect())
            .unwrap_or_default()
    }

    pub fn clear_session(
        &self,
        session_id: &str,
    ) {
        self.sessions.remove(session_id);
    }

    pub fn snapshot_global(&self) -> Vec<Entry> {
        self.global
            .lock()
            .map(|value| value.iter().cloned().collect())
            .unwrap_or_default()
    }
}

#[bon]
impl Store {
    #[builder]
    pub fn builder(
        #[builder(setters(name = with_sse))] sse: sse::Registry,
        #[builder(default = 50, setters(name = with_max_entries))] max_entries: usize,
    ) -> Self {
        Self::new(sse, max_entries)
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

        let entry = Entry::builder()
            .timestamp(jiff::Timestamp::now().to_string())
            .level(level.to_string())
            .target(event.metadata().target().to_string())
            .message(message)
            .fields(visitor.fields)
            .build();

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
    Extension(state): Extension<crate::State>,
    req: Request<Body>,
    next: Next,
) -> Response {
    let started_at = Instant::now();
    let method = req.method().to_string();
    let path = req.uri().path().to_string();
    let request_id = request::current_context()
        .and_then(|value| value.request_id)
        .unwrap_or_else(|| "unknown".to_string());
    let session_id = request::current_context()
        .and_then(|value| value.session_id);

    state.trace_log.record_with_session(
        &request_id,
        session_id.as_deref(),
        Entry::builder()
            .timestamp(jiff::Timestamp::now().to_string())
            .level("INFO".to_string())
            .target("demo.request".to_string())
            .message("request.start".to_string())
            .fields(vec![
                ("method".to_string(), method.clone()),
                ("path".to_string(), path.clone()),
                ("request_id".to_string(), request_id.clone()),
            ])
            .build(),
    );

    let response = next.run(req).await;
    let latency_ms = started_at.elapsed().as_millis().to_string();

    state.trace_log.record_with_session(
        &request_id,
        session_id.as_deref(),
        Entry::builder()
            .timestamp(jiff::Timestamp::now().to_string())
            .level("INFO".to_string())
            .target("demo.request".to_string())
            .message("request.end".to_string())
            .fields(vec![
                ("method".to_string(), method),
                ("path".to_string(), path),
                ("status".to_string(), response.status().as_u16().to_string()),
                ("latency_ms".to_string(), latency_ms),
                ("request_id".to_string(), request_id.clone()),
            ])
            .build(),
    );

    if response.status() == StatusCode::INTERNAL_SERVER_ERROR {
        tracing::error!(target: "demo.request", "response error");
    }

    response
}
