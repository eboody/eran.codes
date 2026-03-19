use std::sync::Arc;

use dashmap::DashMap;

#[derive(Clone, Default)]
pub struct RoomBindings {
    bindings: Arc<DashMap<crate::sse::StreamKey, domain::chat::room::Id>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Match {
    Bound,
    Missing,
    Mismatch,
}

impl RoomBindings {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn bind(&self, handle: &crate::sse::Handle, room_id: domain::chat::room::Id) {
        self.bindings.insert(handle.stream_key().clone(), room_id);
    }

    pub fn matches(
        &self,
        handle: &crate::sse::Handle,
        room_id: &domain::chat::room::Id,
    ) -> Match {
        match self.bindings.get(handle.stream_key()) {
            Some(bound_room) if *bound_room == *room_id => Match::Bound,
            Some(_) => Match::Mismatch,
            None => Match::Missing,
        }
    }

    pub fn room_id_for(
        &self,
        handle: &crate::sse::Handle,
    ) -> Option<domain::chat::room::Id> {
        self.bindings
            .get(handle.stream_key())
            .map(|entry| *entry.value())
    }

    pub fn stream_keys_for_room(
        &self,
        room_id: &domain::chat::room::Id,
    ) -> Vec<crate::sse::StreamKey> {
        self.bindings
            .iter()
            .filter(|entry| *entry.value() == *room_id)
            .map(|entry| entry.key().clone())
            .collect()
    }

    pub fn remove(&self, key: &crate::sse::StreamKey) {
        self.bindings.remove(key);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{SessionId, SseTabId};

    #[test]
    fn reports_missing_until_handle_is_bound() {
        let bindings = RoomBindings::new();
        let handle = crate::sse::Handle::with_tab(
            SessionId::new("session-1"),
            Some(SseTabId::new("tab-1")),
        );
        let room_id = domain::chat::room::Id::new_v4();

        assert_eq!(bindings.matches(&handle, &room_id), Match::Missing);
    }

    #[test]
    fn distinguishes_matching_and_mismatched_rooms() {
        let bindings = RoomBindings::new();
        let handle = crate::sse::Handle::with_tab(
            SessionId::new("session-1"),
            Some(SseTabId::new("tab-1")),
        );
        let room_a = domain::chat::room::Id::new_v4();
        let room_b = domain::chat::room::Id::new_v4();
        bindings.bind(&handle, room_a);

        assert_eq!(bindings.matches(&handle, &room_a), Match::Bound);
        assert_eq!(bindings.matches(&handle, &room_b), Match::Mismatch);
    }

    #[test]
    fn filters_stream_keys_by_room() {
        let bindings = RoomBindings::new();
        let room_a = domain::chat::room::Id::new_v4();
        let room_b = domain::chat::room::Id::new_v4();
        let handle_a = crate::sse::Handle::with_tab(
            SessionId::new("session-1"),
            Some(SseTabId::new("tab-a")),
        );
        let handle_b = crate::sse::Handle::with_tab(
            SessionId::new("session-2"),
            Some(SseTabId::new("tab-b")),
        );
        bindings.bind(&handle_a, room_a);
        bindings.bind(&handle_b, room_b);

        let room_a_keys = bindings.stream_keys_for_room(&room_a);

        assert_eq!(room_a_keys, vec![handle_a.stream_key().clone()]);
    }

    #[test]
    fn returns_bound_room_for_handle() {
        let bindings = RoomBindings::new();
        let room_id = domain::chat::room::Id::new_v4();
        let handle = crate::sse::Handle::with_tab(
            SessionId::new("session-1"),
            Some(SseTabId::new("tab-a")),
        );
        bindings.bind(&handle, room_id);

        assert_eq!(bindings.room_id_for(&handle), Some(room_id));
    }
}
