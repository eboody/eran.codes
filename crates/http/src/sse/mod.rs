use dashmap::DashMap;
use datastar::prelude::{DatastarEvent, ExecuteScript, PatchElements, PatchSignals};
use std::sync::Arc;
use tokio::sync::broadcast;

pub const SESSION_COOKIE: &str = "session_id";

mod session {
    use tower_cookies::cookie::SameSite;
    use tower_cookies::{Cookie, Cookies, Key};
    use uuid::Uuid;

    use super::{Event, SESSION_COOKIE};

    const SESSION_CHANNEL_SIZE: usize = 32;

    pub struct Handle {
        id: String,
    }

    impl Handle {
        pub fn from_cookies(
            cookies: &Cookies,
            key: &Key,
        ) -> Self {
            let id = ensure_session(cookies, key);
            Self { id }
        }

        pub fn id(&self) -> &str {
            &self.id
        }
    }

    pub struct Session {
        sender: tokio::sync::broadcast::Sender<Event>,
    }

    impl Session {
        pub fn new() -> Self {
            let (sender, _receiver) = tokio::sync::broadcast::channel(SESSION_CHANNEL_SIZE);
            Self { sender }
        }

        pub fn subscribe(&self) -> tokio::sync::broadcast::Receiver<Event> {
            self.sender.subscribe()
        }

        pub fn send(
            &self,
            event: Event,
        ) -> Result<usize, tokio::sync::broadcast::error::SendError<Event>> {
            self.sender.send(event)
        }
    }

    fn ensure_session(
        cookies: &Cookies,
        key: &Key,
    ) -> String {
        if let Some(cookie) = cookies.signed(key).get(SESSION_COOKIE) {
            return cookie.value().to_string();
        }

        let session_id = Uuid::new_v4().to_string();
        let cookie = Cookie::build((SESSION_COOKIE, session_id.clone()))
            .path("/")
            .http_only(true)
            .same_site(SameSite::Lax)
            .secure(!cfg!(debug_assertions))
            .build();

        cookies.signed(key).add(cookie);
        session_id
    }
}

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
    ) -> broadcast::Receiver<Event> {
        self.sessions
            .entry(handle.id().to_string())
            .or_insert_with(Session::new)
            .subscribe()
    }

    pub fn send(
        &self,
        handle: &Handle,
        event: Event,
    ) -> SendResult<()> {
        let session = self
            .sessions
            .get(handle.id())
            .ok_or(SendError::SessionMissing)?;

        session
            .send(event)
            .map(|_| ())
            .map_err(|_| SendError::SendFailed)
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

        session
            .send(event)
            .map(|_| ())
            .map_err(|_| SendError::SendFailed)
    }
}
