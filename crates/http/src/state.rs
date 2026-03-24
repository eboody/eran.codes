use std::sync::atomic::{AtomicI64, AtomicU64};

use bon::bon;
use tower_cookies::Key;

#[derive(Clone)]
pub struct State {
    pub user: app::user::Service,
    pub auth: app::auth::Service,
    pub chat: app::chat::Service,
    pub sensitive: app::sensitive::Service,
    pub sse: crate::sse::Registry,
    pub cookie_key: Key,
    pub trace_log: crate::trace_log::Store,
    pub demo: DemoState,
}

#[derive(Clone)]
pub struct DemoState {
    pub surreal: SurrealState,
    pub counter: CounterState,
    pub chat_room_bindings: crate::chat_demo::room::Bindings,
}

impl DemoState {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for DemoState {
    fn default() -> Self {
        Self {
            surreal: SurrealState::new(),
            counter: CounterState::new(),
            chat_room_bindings: crate::chat_demo::room::Bindings::new(),
        }
    }
}

#[derive(Clone)]
pub struct CounterState {
    pub server_count: std::sync::Arc<AtomicI64>,
}

impl CounterState {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for CounterState {
    fn default() -> Self {
        Self {
            server_count: std::sync::Arc::new(AtomicI64::new(0)),
        }
    }
}

#[derive(Clone)]
pub struct SurrealState {
    pub guard: std::sync::Arc<
        dashmap::DashMap<crate::sse::StreamKey, std::sync::Arc<tokio::sync::Mutex<()>>>,
    >,
    pub cancel: std::sync::Arc<
        dashmap::DashMap<crate::sse::StreamKey, tokio_util::sync::CancellationToken>,
    >,
    pub seq: std::sync::Arc<AtomicU64>,
}

impl SurrealState {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for SurrealState {
    fn default() -> Self {
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
        sensitive: app::sensitive::Service,
        sse: crate::sse::Registry,
        cookie_key: Key,
        trace_log: crate::trace_log::Store,
    ) -> Self {
        Self {
            user,
            auth,
            chat,
            sensitive,
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
        #[builder(setters(name = with_sensitive))] sensitive: app::sensitive::Service,
        #[builder(setters(name = with_sse))] sse: crate::sse::Registry,
        #[builder(setters(name = with_cookie_key))] cookie_key: Key,
        #[builder(setters(name = with_trace_log))] trace_log: crate::trace_log::Store,
    ) -> Self {
        Self::new(user, auth, chat, sensitive, sse, cookie_key, trace_log)
    }
}
