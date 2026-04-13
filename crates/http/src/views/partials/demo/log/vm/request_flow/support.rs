use std::collections::HashSet;

use crate::trace_log::store;
use crate::types::{LogFieldKey, Text};
use crate::views::partials::components;
use crate::views::partials::demo::log::vm::entry_fields::{
    non_empty_field_text, short_request_id,
};

use super::event_builder::build_flow_event;
use super::kind::FlowEvent;

pub(super) struct FlowIdentity {
    pub(super) id: Text,
    detail_id: Text,
    display_id: Text,
    has_request_id: bool,
}

impl FlowIdentity {
    pub(super) fn from_entry(entry: &store::TraceEntry, index: usize) -> Self {
        match non_empty_field(entry, LogFieldKey::RequestId) {
            Some(request_id) => {
                let id = request_id.clone();

                Self {
                    detail_id: Text::from(format!("network-flow-{}", slugify(&id.to_string()))),
                    display_id: short_request_id(request_id),
                    has_request_id: true,
                    id,
                }
            }
            None => {
                let id = Text::from(format!(
                    "orphan-{}-{index}",
                    entry.timestamp.to_string().replace(':', "")
                ));

                Self {
                    detail_id: Text::from(format!("network-flow-{}", slugify(&id.to_string()))),
                    display_id: Text::from("orphan"),
                    has_request_id: false,
                    id,
                }
            }
        }
    }
}

pub(super) struct FlowDraft {
    identity: FlowIdentity,
    latest_timestamp: Text,
    latest_index: usize,
    has_request_envelope: bool,
    method: Option<Text>,
    path: Option<Text>,
    status: Option<Text>,
    tab_ids: HashSet<String>,
    events: Vec<components::logs::composed::FlowEvent>,
}

impl FlowDraft {
    pub(super) fn from_entry(
        identity: FlowIdentity,
        kind: FlowEvent,
        entry: &store::TraceEntry,
        index: usize,
    ) -> Self {
        let mut draft = Self {
            identity,
            latest_timestamp: Text::from(entry.timestamp.clone()),
            latest_index: index,
            has_request_envelope: false,
            method: None,
            path: None,
            status: None,
            tab_ids: HashSet::new(),
            events: Vec::new(),
        };
        draft.record(kind, entry, index);
        draft
    }

    pub(super) fn record(&mut self, kind: FlowEvent, entry: &store::TraceEntry, index: usize) {
        self.latest_timestamp = Text::from(entry.timestamp.clone());
        self.latest_index = index;
        self.events.push(build_flow_event(kind, entry));
        self.hydrate_request_fields(entry, kind);
        if let Some(tab_id) = non_empty_field(entry, LogFieldKey::SseTabId) {
            self.tab_ids.insert(tab_id.to_string());
        }
    }

    pub(super) fn latest_index(&self) -> usize {
        self.latest_index
    }

    pub(super) fn is_renderable(&self) -> bool {
        !self.identity.has_request_id || self.has_request_envelope
    }

    pub(super) fn matches_active_tab(&self, active_tab_id: &str) -> bool {
        self.tab_ids.is_empty() || self.tab_ids.contains(active_tab_id)
    }

    pub(super) fn into_flow(self) -> components::logs::composed::Flow {
        let title = self.title();

        components::logs::composed::Flow {
            id: self.identity.id,
            detail_id: self.identity.detail_id,
            display_id: self.identity.display_id,
            title,
            latest_timestamp: self.latest_timestamp,
            status: self.status,
            events: self.events,
        }
    }

    fn hydrate_request_fields(&mut self, entry: &store::TraceEntry, kind: FlowEvent) {
        if matches!(kind, FlowEvent::RequestEnd | FlowEvent::RequestStart) {
            self.has_request_envelope = true;
            if self.method.is_none() {
                self.method = non_empty_field(entry, LogFieldKey::Method).cloned();
            }
            if self.path.is_none() {
                self.path = non_empty_field(entry, LogFieldKey::Path).cloned();
            }
        }

        if self.status.is_none() {
            self.status = non_empty_field(entry, LogFieldKey::Status).cloned();
        }
    }

    fn title(&self) -> Text {
        match (&self.method, &self.path) {
            (Some(method), Some(path)) => Text::from(format!("{method} {path}")),
            (Some(method), None) => Text::from(format!("{method} request")),
            _ if self.identity.has_request_id => {
                Text::from(format!("Request {}", self.identity.display_id))
            }
            _ => Text::from("Request (orphan)"),
        }
    }
}

fn non_empty_field(entry: &store::TraceEntry, key: LogFieldKey) -> Option<&Text> {
    non_empty_field_text(entry, key)
}

fn slugify(value: &str) -> String {
    let mut slug = String::with_capacity(value.len());
    let mut last_dash = false;

    for ch in value.chars() {
        if ch.is_ascii_alphanumeric() {
            slug.push(ch.to_ascii_lowercase());
            last_dash = false;
        } else if !last_dash {
            slug.push('-');
            last_dash = true;
        }
    }

    let trimmed = slug.trim_matches('-');
    if trimmed.is_empty() {
        "flow".to_string()
    } else {
        trimmed.to_string()
    }
}
