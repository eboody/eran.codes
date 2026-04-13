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
        self.entries.push_back(entry.clone());
        entry
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
        self.record_session_and_global(session_id, entry, touch);
    }

    pub fn record_sse_event(&self, session_id: Option<&SessionId>, entry: TraceEntry) {
        let touch = self.next_touch();
        self.record_session_and_global(session_id, entry, touch);
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

    fn record_session_and_global(
        &self,
        session_id: Option<&SessionId>,
        entry: TraceEntry,
        touch: u64,
    ) {
        if let Some(session_id) = session_id {
            self.record_scoped_entry(&self.sessions, session_id, entry.clone(), touch);
            self.prune_scoped_keys(&self.sessions);
        }

        self.push_global_entry(entry);
        self.refresh_session_panels(session_id);
    }

    fn push_global_entry(&self, entry: TraceEntry) {
        if let Ok(mut global) = self.global.lock() {
            if global.len() >= self.max_entries {
                global.pop_front();
            }
            global.push_back(entry);
        }
    }

    fn refresh_session_panels(&self, session_id: Option<&SessionId>) {
        if !self.emit_sse {
            return;
        }

        let Some(session_id) = session_id else {
            return;
        };
        let entries = self.snapshot_session(session_id);
        self.emit_session_log_panels(session_id, &entries, None);
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
mod tests;
