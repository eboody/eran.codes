use std::collections::HashMap;

use crate::trace_log::store;

use super::support::{GroupDraft, GroupKey};

pub(super) fn collect_groups<'a, I>(entries: I) -> Vec<GroupDraft<'a>>
where
    I: IntoIterator<Item = &'a store::TraceEntry>,
{
    let mut order: Vec<GroupKey> = Vec::new();
    let mut groups: HashMap<GroupKey, GroupDraft<'a>> = HashMap::new();

    for entry in entries {
        let key = GroupKey::from_entry(entry);

        match groups.entry(key.clone()) {
            std::collections::hash_map::Entry::Vacant(vacant) => {
                order.push(key);
                vacant.insert(GroupDraft::new(entry));
            }
            std::collections::hash_map::Entry::Occupied(mut occupied) => {
                occupied.get_mut().push(entry);
            }
        }
    }

    order
        .into_iter()
        .filter_map(|key| groups.remove(&key))
        .collect()
}
