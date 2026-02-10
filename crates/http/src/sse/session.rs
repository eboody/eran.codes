use std::sync::atomic::{AtomicUsize, Ordering};
use tower_cookies::cookie::SameSite;
use tower_cookies::{Cookie, Cookies, Key};
use uuid::Uuid;

use super::{Event, SESSION_COOKIE};
use crate::types::SessionId;

const SESSION_CHANNEL_SIZE: usize = 32;

pub struct Handle {
    id: SessionId,
}

impl Handle {
    pub fn from_cookies(
        cookies: &Cookies,
        key: &Key,
    ) -> Self {
        let id = ensure_session(cookies, key);
        Self { id }
    }

    pub fn id(&self) -> SessionId {
        self.id.clone()
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

fn ensure_session(
    cookies: &Cookies,
    key: &Key,
) -> SessionId {
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
