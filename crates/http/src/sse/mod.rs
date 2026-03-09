use dashmap::DashMap;
use datastar::prelude::{DatastarEvent, ExecuteScript, PatchElements, PatchSignals};
use std::sync::Arc;
use tokio::sync::broadcast;

use crate::types::{SessionId, SseTabId};

pub const SESSION_COOKIE: &str = "session_id";

mod session;

pub use session::{Handle, Session};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct StreamKey {
    session_id: SessionId,
    tab_id: Option<SseTabId>,
}

impl StreamKey {
    pub fn new(session_id: SessionId, tab_id: Option<SseTabId>) -> Self {
        Self { session_id, tab_id }
    }

    pub fn session_id(&self) -> &SessionId {
        &self.session_id
    }

    pub fn tab_id(&self) -> Option<&SseTabId> {
        self.tab_id.as_ref()
    }
}

#[derive(Clone, Debug)]
pub struct Event {
    inner: Arc<DatastarEvent>,
}

impl Event {
    pub fn patch_elements(elements: impl Into<String>) -> Self {
        Self {
            inner: Arc::new(PatchElements::new(elements).into_datastar_event()),
        }
    }

    pub fn patch_signals(signals: serde_json::Value) -> Self {
        Self {
            inner: Arc::new(PatchSignals::new(signals.to_string()).into_datastar_event()),
        }
    }

    pub fn execute_script(script: impl Into<String>) -> Self {
        Self {
            inner: Arc::new(ExecuteScript::new(script).into_datastar_event()),
        }
    }

    pub fn from_event(event: DatastarEvent) -> Self {
        Self {
            inner: Arc::new(event),
        }
    }

    pub fn as_datastar_event(&self) -> &DatastarEvent {
        &self.inner
    }
}

#[derive(Debug)]
pub enum SendError {
    SessionMissing,
    SendFailed,
}

pub type SendResult<T> = Result<T, SendError>;

#[derive(Clone, Default)]
pub struct Registry {
    sessions: Arc<DashMap<StreamKey, Session>>,
}

impl Registry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn subscribe(&self, handle: &Handle) -> (broadcast::Receiver<Event>, SessionGuard) {
        let key = handle.stream_key().clone();
        let receiver = self.sessions.entry(key.clone()).or_default().subscribe();
        let guard = SessionGuard::new(self.clone(), key);

        (receiver, guard)
    }

    pub fn send(&self, handle: &Handle, event: Event) -> SendResult<()> {
        let event_type = format!("{:?}", event.as_datastar_event().event);
        let session_id = handle.id();
        let tab_id = handle
            .tab_id()
            .map(ToString::to_string)
            .unwrap_or_else(|| "-".to_string());
        tracing::debug!(
            target: "demo.sse",
            message = "sse send",
            session_id = %session_id,
            tab_id = %tab_id,
            event_type = event_type
        );
        let session = self.sessions.get(handle.stream_key());
        let Some(session) = session else {
            if handle.tab_id().is_none() {
                return self.send_by_id(&session_id, event);
            }
            return Err(SendError::SessionMissing);
        };

        let result = session.send(event).map(|_| ());
        if result.is_err() {
            drop(session);
            self.sessions.remove(handle.stream_key());
            return Err(SendError::SendFailed);
        }
        Ok(())
    }

    pub fn send_by_id(&self, session_id: &SessionId, event: Event) -> SendResult<()> {
        let event_type = format!("{:?}", event.as_datastar_event().event);
        tracing::debug!(
            target: "demo.sse",
            message = "sse send",
            session_id = %session_id,
            event_type = event_type
        );
        let keys = self.stream_keys_for_session(session_id);
        if keys.is_empty() {
            return Err(SendError::SessionMissing);
        }

        let mut sent = 0usize;
        let mut failed = Vec::new();
        for key in keys {
            if let Some(session) = self.sessions.get(&key) {
                if session.send(event.clone()).is_ok() {
                    sent += 1;
                } else {
                    failed.push(key.clone());
                }
            }
        }

        for key in failed {
            self.sessions.remove(&key);
        }

        if sent == 0 {
            return Err(SendError::SendFailed);
        }

        Ok(())
    }

    pub fn broadcast(&self, event: Event) -> SendResult<usize> {
        let event_type = format!("{:?}", event.as_datastar_event().event);
        let mut sent = 0;
        let mut failed = Vec::new();
        let total = self.sessions.len();
        tracing::debug!(
            target: "demo.sse",
            message = "sse broadcast",
            sessions = total,
            event_type = event_type
        );
        for entry in self.sessions.iter() {
            let result = entry.value().send(event.clone());
            if result.is_err() {
                failed.push(entry.key().clone());
            } else {
                sent += 1;
            }
        }

        for key in failed {
            self.sessions.remove(&key);
        }

        if sent == 0 && !self.sessions.is_empty() {
            return Err(SendError::SendFailed);
        }

        Ok(sent)
    }

    pub fn remove(&self, key: &StreamKey) {
        self.sessions.remove(key);
    }

    pub fn has_streams_for_session(&self, session_id: &SessionId) -> bool {
        self.sessions
            .iter()
            .any(|entry| entry.key().session_id() == session_id)
    }

    pub fn release(&self, key: &StreamKey) {
        if let Some(entry) = self.sessions.get(key) {
            let remaining = entry.release();
            if remaining == 0 {
                drop(entry);
                self.sessions.remove(key);
            }
        }
    }

    pub fn stream_keys_for_session(&self, session_id: &SessionId) -> Vec<StreamKey> {
        self.sessions
            .iter()
            .filter(|entry| entry.key().session_id() == session_id)
            .map(|entry| entry.key().clone())
            .collect()
    }
}

pub struct SessionGuard {
    registry: Registry,
    stream_key: StreamKey,
}

impl SessionGuard {
    pub fn new(registry: Registry, stream_key: StreamKey) -> Self {
        Self {
            registry,
            stream_key,
        }
    }
}

impl Drop for SessionGuard {
    fn drop(&mut self) {
        self.registry.release(&self.stream_key);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tower_cookies::{Cookies, Key};

    #[test]
    fn keeps_session_until_last_guard_drops() {
        let registry = Registry::new();
        let key = Key::generate();
        let cookies = Cookies::default();
        let handle = Handle::from_cookies(&cookies, &key);

        let (_rx1, guard1) = registry.subscribe(&handle);
        let (_rx2, guard2) = registry.subscribe(&handle);

        drop(guard1);
        let send_result = registry.send(&handle, Event::patch_elements("ok"));
        assert!(send_result.is_ok());

        drop(guard2);
        let send_result = registry.send(&handle, Event::patch_elements("ok"));
        assert!(matches!(send_result, Err(SendError::SessionMissing)));
    }

    #[test]
    fn send_by_id_fans_out_to_all_tabs_for_same_session() {
        let registry = Registry::new();
        let key = Key::generate();
        let cookies = Cookies::default();
        let base = Handle::from_cookies(&cookies, &key);
        let session_id = base.id();
        let tab_a = Handle::with_tab(session_id.clone(), Some(SseTabId::new("tab-a")));
        let tab_b = Handle::with_tab(session_id.clone(), Some(SseTabId::new("tab-b")));

        let (mut rx_a, _guard_a) = registry.subscribe(&tab_a);
        let (mut rx_b, _guard_b) = registry.subscribe(&tab_b);

        let result = registry.send_by_id(&session_id, Event::patch_elements("ok"));
        assert!(result.is_ok());
        assert!(rx_a.try_recv().is_ok());
        assert!(rx_b.try_recv().is_ok());
    }
}
