use crate::types::SseTabId;

use super::support::FlowDraft;

pub(super) fn retain_renderable(flows: &mut Vec<FlowDraft>) {
    flows.retain(FlowDraft::is_renderable);
}

pub(super) fn sort_newest_first(flows: &mut [FlowDraft]) {
    flows.sort_by_key(|flow| std::cmp::Reverse(flow.latest_index()));
}

pub(super) fn retain_active_tab(flows: &mut Vec<FlowDraft>, active_tab_id: Option<&SseTabId>) {
    if let Some(active_tab_id) = active_tab_id.map(ToString::to_string) {
        flows.retain(|flow| flow.matches_active_tab(&active_tab_id));
    }
}
