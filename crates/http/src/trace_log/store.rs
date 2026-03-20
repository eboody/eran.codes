use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

use bon::{Builder, bon};
use dashmap::DashMap;
use maud::Render;

use crate::types::{
    LogFieldKey, LogFieldName, LogFieldValue, LogLevelText, LogMessageText, LogTargetText,
    RequestId, SessionId, SseTabId, Text, TimestampText,
};
use crate::{sse, views};

use super::log::{self, message, target};
use super::{FlowFilterTerms, db_bind};

#[derive(Clone, Debug, Builder)]
pub struct TraceEntry {
    pub timestamp: TimestampText,
    pub level: LogLevelText,
    pub target: LogTargetText,
    pub message: LogMessageText,
    pub fields: Vec<(LogFieldName, LogFieldValue)>,
}

impl TraceEntry {
    pub fn field_value(&self, key: LogFieldKey) -> Option<&LogFieldValue> {
        let name = LogFieldName::from(key);
        self.fields
            .iter()
            .find(|(field, _)| field == &name)
            .map(|(_, value)| value)
    }

    pub fn field_text(&self, key: LogFieldKey) -> Option<&Text> {
        self.field_value(key).and_then(LogFieldValue::as_text)
    }

    pub fn target_kind(&self) -> target::Kind {
        let (target_kind, _) = self.kinds();
        target_kind
    }

    pub fn message_kind(&self) -> message::Kind {
        let (_, message_kind) = self.kinds();
        message_kind
    }

    pub fn db_bind_values(&self) -> Vec<(usize, Text)> {
        let mut values: Vec<(usize, Text)> = self
            .fields
            .iter()
            .filter_map(|(name, value)| {
                let index = db_bind::Index::from_field_name(&name.to_string())?.get();
                let value = match value {
                    LogFieldValue::Text(text) => text.clone(),
                    LogFieldValue::Missing => return None,
                };
                Some((index, value))
            })
            .collect();
        values.sort_by_key(|(index, _)| *index);
        values
    }

    pub fn kinds(&self) -> (target::Kind, message::Kind) {
        log::classify(&self.target.to_string(), &self.message.to_string())
    }
}

#[derive(Clone)]
pub struct Store {
    requests: Arc<DashMap<RequestId, VecDeque<TraceEntry>>>,
    sessions: Arc<DashMap<SessionId, VecDeque<TraceEntry>>>,
    flow_filters: Arc<DashMap<sse::StreamKey, FlowFilterTerms>>,
    global: Arc<Mutex<VecDeque<TraceEntry>>>,
    max_entries: usize,
    sse: sse::Registry,
    emit_sse: bool,
}

impl Store {
    pub fn new(sse: sse::Registry, max_entries: usize, emit_sse: bool) -> Self {
        Self {
            requests: Arc::new(DashMap::new()),
            sessions: Arc::new(DashMap::new()),
            flow_filters: Arc::new(DashMap::new()),
            global: Arc::new(Mutex::new(VecDeque::new())),
            max_entries,
            sse,
            emit_sse,
        }
    }

    pub fn record_with_session(
        &self,
        request_id: &RequestId,
        session_id: Option<&SessionId>,
        entry: TraceEntry,
    ) {
        let mut queue = self.requests.entry(request_id.clone()).or_default();
        if queue.len() >= self.max_entries {
            queue.pop_front();
        }
        queue.push_back(entry);

        let entry = queue.back().cloned().expect("entry");

        if let Some(session_id) = session_id {
            let mut session_queue = self.sessions.entry(session_id.clone()).or_default();
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

        if self.emit_sse
            && let Some(session_id) = session_id
        {
            let entries = self.snapshot_session(session_id);
            self.emit_session_log_panels(session_id, &entries, None);
        }
    }

    pub fn record_sse_event(&self, session_id: Option<&SessionId>, entry: TraceEntry) {
        if let Some(session_id) = session_id {
            let mut session_queue = self.sessions.entry(session_id.clone()).or_default();
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

        if self.emit_sse
            && let Some(session_id) = session_id
        {
            let entries = self.snapshot_session(session_id);
            self.emit_session_log_panels(session_id, &entries, None);
        }
    }

    fn emit_session_log_panels(
        &self,
        session_id: &SessionId,
        entries: &[TraceEntry],
        target_tab: Option<&SseTabId>,
    ) {
        for stream_key in self.sse.stream_keys_for_session(session_id) {
            if let Some(target_tab) = target_tab
                && stream_key.tab_id() != Some(target_tab)
            {
                continue;
            }
            let active_tab_id = stream_key.tab_id().cloned();
            let excluded_terms = self.filter_terms_for_stream(&stream_key);
            let network_log = views::partials::TransportLogSet::builder()
                .entries(entries)
                .maybe_active_tab_id(active_tab_id.clone())
                .excluded_terms(excluded_terms)
                .build()
                .render()
                .into_string();
            let handle = sse::Handle::with_tab(session_id.clone(), active_tab_id);
            if let Err(error) = self
                .sse
                .send(&handle, sse::Event::patch_elements(network_log))
            {
                tracing::debug!(?error, session_id = %session_id, "trace log SSE fanout failed");
            }
        }
    }

    pub fn set_stream_flow_filter(
        &self,
        session_id: &SessionId,
        tab_id: Option<&SseTabId>,
        filter_query: Option<&str>,
    ) {
        let stream_key = sse::StreamKey::new(session_id.clone(), tab_id.cloned());
        let terms = filter_query.map(FlowFilterTerms::from).unwrap_or_default();
        if terms.is_empty() {
            self.flow_filters.remove(&stream_key);
        } else {
            self.flow_filters.insert(stream_key, terms);
        }
    }

    pub fn refresh_stream_log_panels(
        &self,
        session_id: &SessionId,
        tab_id: Option<&SseTabId>,
    ) {
        let entries = self.snapshot_session(session_id);
        self.emit_session_log_panels(session_id, &entries, tab_id);
    }

    pub fn clear_stream_flow_filter(&self, stream_key: &sse::StreamKey) {
        self.flow_filters.remove(stream_key);
    }

    pub fn snapshot_request(&self, request_id: &RequestId) -> Vec<TraceEntry> {
        self.requests
            .get(request_id)
            .map(|queue| queue.iter().cloned().collect())
            .unwrap_or_default()
    }

    pub fn snapshot_session(&self, session_id: &SessionId) -> Vec<TraceEntry> {
        self.sessions
            .get(session_id)
            .map(|queue| queue.iter().cloned().collect())
            .unwrap_or_default()
    }

    pub fn snapshot_global(&self) -> Vec<TraceEntry> {
        self.global
            .lock()
            .map(|value| value.iter().cloned().collect())
            .unwrap_or_default()
    }

    fn filter_terms_for_stream(&self, stream_key: &sse::StreamKey) -> FlowFilterTerms {
        let Some(query) = self.flow_filters.get(stream_key) else {
            return FlowFilterTerms::default();
        };
        query.clone()
    }
}

#[bon]
impl Store {
    #[builder]
    pub fn builder(
        #[builder(setters(name = with_sse))] sse: sse::Registry,
        #[builder(default = 50, setters(name = with_max_entries))] max_entries: usize,
        #[builder(default = true, setters(name = with_emit_sse))] emit_sse: bool,
    ) -> Self {
        Self::new(sse, max_entries, emit_sse)
    }
}
