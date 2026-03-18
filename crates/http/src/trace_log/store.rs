use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

use bon::{Builder, bon};
use dashmap::DashMap;
use maud::Render;

use crate::types::{
    LogFieldName, LogFieldValue, LogLevelText, LogMessageText, LogTargetText, RequestId,
    SessionId, SseTabId, Text, TimestampText,
};
use crate::{sse, views};

#[derive(Clone, Debug, Builder)]
pub struct TraceEntry {
    pub timestamp: TimestampText,
    pub level: LogLevelText,
    pub target: LogTargetText,
    pub message: LogMessageText,
    pub fields: Vec<(LogFieldName, LogFieldValue)>,
}

#[derive(Clone)]
pub struct Store {
    requests: Arc<DashMap<RequestId, VecDeque<TraceEntry>>>,
    sessions: Arc<DashMap<SessionId, VecDeque<TraceEntry>>>,
    flow_filters: Arc<DashMap<sse::StreamKey, Text>>,
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
        let normalized = filter_query
            .map(str::trim)
            .filter(|value| !value.is_empty());
        if let Some(value) = normalized {
            self.flow_filters
                .insert(stream_key, Text::from(value.to_string()));
        } else {
            self.flow_filters.remove(&stream_key);
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

    fn filter_terms_for_stream(&self, stream_key: &sse::StreamKey) -> Vec<Text> {
        let Some(query) = self.flow_filters.get(stream_key) else {
            return Vec::new();
        };
        parse_filter_terms(&query.to_string())
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

fn parse_filter_terms(query: &str) -> Vec<Text> {
    query
        .split(',')
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(|value| Text::from(value.to_lowercase()))
        .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse_filter_terms_trims_commas_and_normalizes_case() {
        let terms = super::parse_filter_terms(" /events,  POST , , /HEALTH ");
        let values: Vec<String> =
            terms.into_iter().map(|value| value.to_string()).collect();
        assert_eq!(values, vec!["/events", "post", "/health"]);
    }
}
