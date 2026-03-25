use std::{
    collections::VecDeque,
    hash::Hash,
    sync::{
        Arc, Mutex,
        atomic::{AtomicU64, Ordering},
    },
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

#[derive(Clone, Debug)]
struct TraceQueue {
    entries: VecDeque<TraceEntry>,
    last_touched: u64,
}

impl TraceQueue {
    fn new(last_touched: u64) -> Self {
        Self {
            entries: VecDeque::new(),
            last_touched,
        }
    }

    fn push(
        &mut self,
        entry: TraceEntry,
        max_entries: usize,
        last_touched: u64,
    ) -> TraceEntry {
        if self.entries.len() >= max_entries {
            self.entries.pop_front();
        }
        self.last_touched = last_touched;
        self.entries.push_back(entry);
        self.entries.back().cloned().expect("trace entry")
    }
}

#[derive(Clone)]
pub struct Store {
    requests: Arc<DashMap<RequestId, TraceQueue>>,
    sessions: Arc<DashMap<SessionId, TraceQueue>>,
    flow_filters: Arc<DashMap<sse::StreamKey, FlowFilterTerms>>,
    global: Arc<Mutex<VecDeque<TraceEntry>>>,
    max_entries: usize,
    next_touch: Arc<AtomicU64>,
    sse: sse::Registry,
    emit_sse: bool,
}

impl Store {
    fn new(sse: sse::Registry, max_entries: usize, emit_sse: bool) -> Self {
        Self {
            requests: Arc::new(DashMap::new()),
            sessions: Arc::new(DashMap::new()),
            flow_filters: Arc::new(DashMap::new()),
            global: Arc::new(Mutex::new(VecDeque::new())),
            max_entries,
            next_touch: Arc::new(AtomicU64::new(0)),
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
        let touch = self.next_touch();
        let entry = self.record_scoped_entry(&self.requests, request_id, entry, touch);
        self.prune_scoped_keys(&self.requests);

        if let Some(session_id) = session_id {
            self.record_scoped_entry(&self.sessions, session_id, entry.clone(), touch);
            self.prune_scoped_keys(&self.sessions);
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
            let touch = self.next_touch();
            self.record_scoped_entry(&self.sessions, session_id, entry.clone(), touch);
            self.prune_scoped_keys(&self.sessions);
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
        let stream_key = match tab_id.cloned() {
            Some(tab_id) => sse::StreamKey::with_tab(session_id.clone(), tab_id),
            None => sse::StreamKey::new(session_id.clone()),
        };
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
            .map(|queue| queue.entries.iter().cloned().collect())
            .unwrap_or_default()
    }

    pub fn snapshot_session(&self, session_id: &SessionId) -> Vec<TraceEntry> {
        self.sessions
            .get(session_id)
            .map(|queue| queue.entries.iter().cloned().collect())
            .unwrap_or_default()
    }

    pub fn snapshot_global(&self) -> Vec<TraceEntry> {
        self.global
            .lock()
            .map(|value| value.iter().cloned().collect())
            .unwrap_or_default()
    }

    #[cfg(test)]
    pub(crate) fn has_stream_flow_filter(&self, stream_key: &sse::StreamKey) -> bool {
        self.flow_filters.contains_key(stream_key)
    }

    fn filter_terms_for_stream(&self, stream_key: &sse::StreamKey) -> FlowFilterTerms {
        let Some(query) = self.flow_filters.get(stream_key) else {
            return FlowFilterTerms::default();
        };
        query.clone()
    }

    fn next_touch(&self) -> u64 {
        self.next_touch.fetch_add(1, Ordering::Relaxed) + 1
    }

    fn record_scoped_entry<K>(
        &self,
        map: &DashMap<K, TraceQueue>,
        key: &K,
        entry: TraceEntry,
        touch: u64,
    ) -> TraceEntry
    where
        K: Clone + Eq + Hash,
    {
        let mut queue = map
            .entry(key.clone())
            .or_insert_with(|| TraceQueue::new(touch));
        queue.push(entry, self.max_entries, touch)
    }

    fn prune_scoped_keys<K>(&self, map: &DashMap<K, TraceQueue>)
    where
        K: Clone + Eq + Hash,
    {
        let len = map.len();
        if len <= self.max_entries {
            return;
        }

        let mut keys: Vec<(K, u64)> = map
            .iter()
            .map(|entry| (entry.key().clone(), entry.value().last_touched))
            .collect();
        keys.sort_by_key(|(_, touch)| *touch);

        for (key, _) in keys.into_iter().take(len - self.max_entries) {
            map.remove(&key);
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    fn trace_entry(message: &str) -> TraceEntry {
        TraceEntry::builder()
            .timestamp(TimestampText::new("2026-03-25 00:00:00"))
            .level(LogLevelText::new("INFO"))
            .target(LogTargetText::new("demo.request"))
            .message(LogMessageText::new(message))
            .fields(Vec::new())
            .build()
    }

    #[test]
    fn prunes_oldest_request_and_session_keys_at_store_bound() {
        let store = Store::builder()
            .with_sse(crate::sse::Registry::new())
            .with_max_entries(2)
            .with_emit_sse(false)
            .build();
        let request_a = RequestId::new("req-a");
        let request_b = RequestId::new("req-b");
        let request_c = RequestId::new("req-c");
        let session_a = SessionId::new("session-a");
        let session_b = SessionId::new("session-b");
        let session_c = SessionId::new("session-c");

        store.record_with_session(&request_a, Some(&session_a), trace_entry("request-a"));
        store.record_with_session(&request_b, Some(&session_b), trace_entry("request-b"));
        store.record_with_session(&request_c, Some(&session_c), trace_entry("request-c"));

        assert!(store.snapshot_request(&request_a).is_empty());
        assert_eq!(store.snapshot_request(&request_b).len(), 1);
        assert_eq!(store.snapshot_request(&request_c).len(), 1);

        assert!(store.snapshot_session(&session_a).is_empty());
        assert_eq!(store.snapshot_session(&session_b).len(), 1);
        assert_eq!(store.snapshot_session(&session_c).len(), 1);

        let global = store.snapshot_global();
        assert_eq!(global.len(), 2);
        assert_eq!(global[0].message.to_string(), "request-b");
        assert_eq!(global[1].message.to_string(), "request-c");
    }

    #[test]
    fn record_sse_event_prunes_oldest_session_keys() {
        let store = Store::builder()
            .with_sse(crate::sse::Registry::new())
            .with_max_entries(2)
            .with_emit_sse(false)
            .build();
        let session_a = SessionId::new("session-a");
        let session_b = SessionId::new("session-b");
        let session_c = SessionId::new("session-c");

        store.record_sse_event(Some(&session_a), trace_entry("sse-a"));
        store.record_sse_event(Some(&session_b), trace_entry("sse-b"));
        store.record_sse_event(Some(&session_c), trace_entry("sse-c"));

        assert!(store.snapshot_session(&session_a).is_empty());
        assert_eq!(store.snapshot_session(&session_b).len(), 1);
        assert_eq!(store.snapshot_session(&session_c).len(), 1);
    }
}
