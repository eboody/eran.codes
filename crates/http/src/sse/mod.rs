use dashmap::DashMap;
use datastar::prelude::{DatastarEvent, ExecuteScript, PatchElements, PatchSignals};
use std::sync::Arc;
use tokio::sync::broadcast;

pub const SESSION_COOKIE: &str = "session_id";

mod session;

pub use session::{Handle, Session};

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
            inner: Arc::new(
                PatchSignals::new(signals.to_string()).into_datastar_event(),
            ),
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
    sessions: Arc<DashMap<String, Session>>,
}

impl Registry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn subscribe(
        &self,
        handle: &Handle,
    ) -> (broadcast::Receiver<Event>, SessionGuard) {
        let session_id = handle.id().to_string();
        let receiver = self
            .sessions
            .entry(session_id.clone())
            .or_insert_with(Session::new)
            .subscribe();
        let guard = SessionGuard::new(self.clone(), session_id);

        (receiver, guard)
    }

    pub fn send(
        &self,
        handle: &Handle,
        event: Event,
    ) -> SendResult<()> {
        let session_id = handle.id();
        let session = self
            .sessions
            .get(session_id)
            .ok_or(SendError::SessionMissing)?;

        let result = session.send(event).map(|_| ());
        if result.is_err() {
            drop(session);
            self.sessions.remove(session_id);
            return Err(SendError::SendFailed);
        }
        Ok(())
    }

    pub fn send_by_id(
        &self,
        session_id: &str,
        event: Event,
    ) -> SendResult<()> {
        let session = self
            .sessions
            .get(session_id)
            .ok_or(SendError::SessionMissing)?;

        let result = session.send(event).map(|_| ());
        if result.is_err() {
            drop(session);
            self.sessions.remove(session_id);
            return Err(SendError::SendFailed);
        }
        Ok(())
    }

    pub fn broadcast(
        &self,
        event: Event,
    ) -> SendResult<usize> {
        let mut sent = 0;
        let mut failed = Vec::new();
        for entry in self.sessions.iter() {
            let result = entry.value().send(event.clone());
            if result.is_err() {
                failed.push(entry.key().clone());
            } else {
                sent += 1;
            }
        }

        for session_id in failed {
            self.sessions.remove(&session_id);
        }

        if sent == 0 && !self.sessions.is_empty() {
            return Err(SendError::SendFailed);
        }

        Ok(sent)
    }

    pub fn remove(
        &self,
        session_id: &str,
    ) {
        self.sessions.remove(session_id);
    }

    pub fn release(
        &self,
        session_id: &str,
    ) {
        if let Some(entry) = self.sessions.get(session_id) {
            let remaining = entry.release();
            if remaining == 0 {
                drop(entry);
                self.sessions.remove(session_id);
            }
        }
    }
}

pub struct SessionGuard {
    registry: Registry,
    session_id: String,
}

impl SessionGuard {
    pub fn new(
        registry: Registry,
        session_id: impl Into<String>,
    ) -> Self {
        Self {
            registry,
            session_id: session_id.into(),
        }
    }
}

impl Drop for SessionGuard {
    fn drop(&mut self) {
        self.registry.release(&self.session_id);
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
        let cookies = Cookies::new();
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
}
