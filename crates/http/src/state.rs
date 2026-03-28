use std::sync::atomic::AtomicI64;

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
