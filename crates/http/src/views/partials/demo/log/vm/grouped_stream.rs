mod build;
mod support;
#[cfg(test)]
mod tests;

use crate::trace_log::store;
use crate::views::partials::components;

use build::collect_groups;
use support::GroupDraft;

pub fn build_grouped_feed<'a, I>(entries: I) -> components::logs::composed::GroupedFeed
where
    I: IntoIterator<Item = &'a store::TraceEntry>,
{
    let groups = collect_groups(entries)
        .into_iter()
        .map(GroupDraft::into_group)
        .collect();

    components::logs::composed::GroupedFeed::builder()
        .children(groups)
        .build()
}
