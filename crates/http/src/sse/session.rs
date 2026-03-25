use std::sync::atomic::{AtomicUsize, Ordering};
use tower_cookies::cookie::SameSite;
use tower_cookies::{Cookie, Cookies, Key};
use uuid::Uuid;

use super::{Event, SESSION_COOKIE, StreamKey};
use crate::types::{SessionId, SseTabId};

const SESSION_CHANNEL_SIZE: usize = 32;

#[derive(Clone)]
pub struct Handle {
    key: StreamKey,
}

impl Handle {
    pub fn from_cookies(cookies: &Cookies, key: &Key) -> Self {
        let session_id = ensure_session(cookies, key);
        Self::with_tab(session_id, None)
    }

    pub fn from_cookies_with_tab(
        cookies: &Cookies,
        key: &Key,
        tab_id: Option<SseTabId>,
    ) -> Self {
        let session_id = ensure_session(cookies, key);
        Self::with_tab(session_id, tab_id)
    }

    pub fn with_tab(session_id: SessionId, tab_id: Option<SseTabId>) -> Self {
        Self {
            key: match tab_id {
                Some(tab_id) => StreamKey::with_tab(session_id, tab_id),
                None => StreamKey::new(session_id),
            },
        }
    }

    pub fn id(&self) -> SessionId {
        self.key.session_id().clone()
    }

    pub fn tab_id(&self) -> Option<&SseTabId> {
        self.key.tab_id()
    }

    pub fn stream_key(&self) -> &StreamKey {
        &self.key
    }
}

pub struct Session {
    sender: tokio::sync::broadcast::Sender<Event>,
    active: AtomicUsize,
}

impl Session {
    pub fn new() -> Self {
        let (sender, _receiver) = tokio::sync::broadcast::channel(SESSION_CHANNEL_SIZE);
        Self {
            sender,
            active: AtomicUsize::new(0),
        }
    }

    pub fn subscribe(&self) -> tokio::sync::broadcast::Receiver<Event> {
        self.active.fetch_add(1, Ordering::Relaxed);
        self.sender.subscribe()
    }

    pub fn send(
        &self,
        event: Event,
    ) -> Result<usize, tokio::sync::broadcast::error::SendError<Event>> {
        self.sender.send(event)
    }

    pub fn release(&self) -> usize {
        let prev = self.active.fetch_sub(1, Ordering::Relaxed);
        prev.saturating_sub(1)
    }
}

impl Default for Session {
    fn default() -> Self {
        Self::new()
    }
}

fn ensure_session(cookies: &Cookies, key: &Key) -> SessionId {
    if let Some(cookie) = cookies.signed(key).get(SESSION_COOKIE) {
        return SessionId::new(cookie.value());
    }

    let session_id = SessionId::new(Uuid::new_v4().to_string());
    let cookie = Cookie::build((SESSION_COOKIE, session_id.to_string()))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .secure(!cfg!(debug_assertions))
        .build();

    cookies.signed(key).add(cookie);
    session_id
}
