use std::collections::HashMap;

use crate::trace_log::TraceEntry;
use crate::types::{LogFieldKey, Text};
use crate::views::partials::components::logs;

use super::{field_text, short_request_id};

mod event_builder;
mod kind;
mod pills;
#[cfg(test)]
mod tests;

use event_builder::build_flow_event;
use kind::{flow_event_kind, FlowEventKind};

pub fn request_flows(entries: &[TraceEntry], max_flows: usize) -> Vec<logs::composed::Flow> {
    let mut order: Vec<String> = Vec::new();
    let mut flow_map: HashMap<String, FlowAggregate> = HashMap::new();

    for (index, entry) in entries.iter().enumerate() {
        let Some(kind) = flow_event_kind(entry) else {
            continue;
        };

        let flow_id = flow_id(entry, index);
        let key = flow_id.to_string();
        let display_id = field_text(entry, LogFieldKey::RequestId)
            .map(|value| short_request_id(&value))
            .unwrap_or_else(|| Text::from("orphan"));

        if !flow_map.contains_key(&key) {
            order.push(key.clone());
            flow_map.insert(
                key.clone(),
                FlowAggregate {
                    id: flow_id.clone(),
                    detail_id: Text::from(format!("network-flow-{}", slugify(&key))),
                    display_id,
                    latest_timestamp: Text::from(entry.timestamp.clone()),
                    latest_index: index,
                    method: None,
                    path: None,
                    status: None,
                    events: Vec::new(),
                },
            );
        }

        if let Some(aggregate) = flow_map.get_mut(&key) {
            aggregate.latest_timestamp = Text::from(entry.timestamp.clone());
            aggregate.latest_index = index;
            aggregate.events.push(build_flow_event(kind, entry));
            hydrate_request_fields(aggregate, entry, kind);
        }
    }

    let mut flows: Vec<FlowAggregate> = order
        .into_iter()
        .filter_map(|key| flow_map.remove(&key))
        .collect();
    flows.sort_by(|left, right| right.latest_index.cmp(&left.latest_index));

    flows
        .into_iter()
        .take(max_flows)
        .map(|flow| {
            let title = flow_title(&flow);
            logs::composed::Flow {
                id: flow.id,
                detail_id: flow.detail_id,
                display_id: flow.display_id,
                title,
                latest_timestamp: flow.latest_timestamp,
                status: flow.status,
                events: flow.events,
            }
        })
        .collect()
}

struct FlowAggregate {
    id: Text,
    detail_id: Text,
    display_id: Text,
    latest_timestamp: Text,
    latest_index: usize,
    method: Option<Text>,
    path: Option<Text>,
    status: Option<Text>,
    events: Vec<logs::composed::FlowEvent>,
}

fn hydrate_request_fields(
    aggregate: &mut FlowAggregate,
    entry: &TraceEntry,
    kind: FlowEventKind,
) {
    if matches!(kind, FlowEventKind::RequestEnd | FlowEventKind::RequestStart) {
        if aggregate.method.is_none() {
            aggregate.method = field_text(entry, LogFieldKey::Method);
        }
        if aggregate.path.is_none() {
            aggregate.path = field_text(entry, LogFieldKey::Path);
        }
    }

    if aggregate.status.is_none() {
        aggregate.status = field_text(entry, LogFieldKey::Status);
    }
}

fn flow_title(flow: &FlowAggregate) -> Text {
    match (&flow.method, &flow.path) {
        (Some(method), Some(path)) => Text::from(format!("{method} {path}")),
        (Some(method), None) => Text::from(format!("{method} request")),
        _ => Text::from(format!("Flow {}", flow.display_id)),
    }
}

fn flow_id(entry: &TraceEntry, index: usize) -> Text {
    field_text(entry, LogFieldKey::RequestId).unwrap_or_else(|| {
        Text::from(format!(
            "orphan-{}-{index}",
            entry.timestamp.to_string().replace(':', "")
        ))
    })
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
