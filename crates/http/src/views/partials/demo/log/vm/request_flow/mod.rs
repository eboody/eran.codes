use crate::trace_log::store;
use crate::types::SseTabId;
use crate::views::partials::components;

mod build;
mod event_builder;
mod filtering;
mod kind;
mod pills;
mod support;
#[cfg(test)]
mod tests;

pub fn request_flows(
    entries: &[store::TraceEntry],
    max_flows: usize,
    active_tab_id: Option<&SseTabId>,
) -> Vec<components::logs::composed::Flow> {
    let mut flows = build::collect_flows(entries);
    filtering::retain_renderable(&mut flows);
    filtering::sort_newest_first(&mut flows);
    filtering::retain_active_tab(&mut flows, active_tab_id);

    flows
        .into_iter()
        .take(max_flows)
        .map(support::FlowDraft::into_flow)
        .collect()
}
