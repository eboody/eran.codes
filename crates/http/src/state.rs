use std::sync::atomic::AtomicU64;

use bon::bon;
use tower_cookies::Key;

#[derive(Clone)]
pub struct State {
    pub user: app::user::Service,
    pub auth: app::auth::Service,
    pub chat: app::chat::Service,
    pub sse: crate::sse::Registry,
    pub cookie_key: Key,
    pub trace_log: crate::trace_log::TraceLogStore,
    pub demo: DemoState,
}

#[derive(Clone)]
pub struct DemoState {
    pub surreal: SurrealState,
}

impl DemoState {
    pub fn new() -> Self {
        Self {
            surreal: SurrealState::new(),
        }
    }
}

#[derive(Clone)]
pub struct SurrealState {
    pub guard:
        std::sync::Arc<dashmap::DashMap<crate::types::SessionId, std::sync::Arc<tokio::sync::Mutex<()>>>>,
    pub cancel:
        std::sync::Arc<dashmap::DashMap<crate::types::SessionId, tokio_util::sync::CancellationToken>>,
    pub seq: std::sync::Arc<AtomicU64>,
}

impl SurrealState {
    pub fn new() -> Self {
        Self {
            guard: std::sync::Arc::new(dashmap::DashMap::new()),
            cancel: std::sync::Arc::new(dashmap::DashMap::new()),
            seq: std::sync::Arc::new(AtomicU64::new(0)),
        }
    }
}

impl State {
    pub fn new(
        user: app::user::Service,
        auth: app::auth::Service,
        chat: app::chat::Service,
        sse: crate::sse::Registry,
        cookie_key: Key,
        trace_log: crate::trace_log::TraceLogStore,
    ) -> Self {
        Self {
            user,
            auth,
            chat,
            sse,
            cookie_key,
            trace_log,
            demo: DemoState::new(),
        }
    }
}

#[bon]
impl State {
    #[builder]
    pub fn builder(
        #[builder(setters(name = with_user))] user: app::user::Service,
        #[builder(setters(name = with_auth))] auth: app::auth::Service,
        #[builder(setters(name = with_chat))] chat: app::chat::Service,
        #[builder(setters(name = with_sse))] sse: crate::sse::Registry,
        #[builder(setters(name = with_cookie_key))] cookie_key: Key,
        #[builder(setters(name = with_trace_log))] trace_log: crate::trace_log::TraceLogStore,
    ) -> Self {
        Self::new(user, auth, chat, sse, cookie_key, trace_log)
    }
}
