use std::collections::HashMap;

use crate::trace_log::store;

use super::kind;
use super::support::{FlowDraft, FlowIdentity};

pub(super) fn collect_flows(entries: &[store::TraceEntry]) -> Vec<FlowDraft> {
    let mut order: Vec<String> = Vec::new();
    let mut flow_map: HashMap<String, FlowDraft> = HashMap::new();

    for (index, entry) in entries.iter().enumerate() {
        let Some(kind) = kind::flow_event(entry) else {
            continue;
        };

        let identity = FlowIdentity::from_entry(entry, index);
        let key = identity.id.to_string();

        match flow_map.entry(key.clone()) {
            std::collections::hash_map::Entry::Vacant(vacant) => {
                order.push(key);
                vacant.insert(FlowDraft::from_entry(identity, kind, entry, index));
            }
            std::collections::hash_map::Entry::Occupied(mut occupied) => {
                occupied.get_mut().record(kind, entry, index);
            }
        }
    }

    order
        .into_iter()
        .filter_map(|key| flow_map.remove(&key))
        .collect()
}
