#[derive(Clone, Copy, Debug)]
pub enum Route {
    Home,
    Login,
    Register,
    Logout,
    Protected,
    Chat,
    ChatMessages,
    ChatMessagesDemo,
    ChatModeration,
    Events,
    ErrorTest,
    Health,
    PartialAuthStatus,
    PartialSessionStatus,
    PartialRequestMeta,
    PartialBoundaryCheck,
    PartialDbCheck,
    PartialPing,
    PartialSurrealGuarded,
    PartialSurrealCancel,
}

impl Route {
    pub const fn as_str(self) -> &'static str {
        match self {
            Route::Home => "/",
            Route::Login => "/login",
            Route::Register => "/register",
            Route::Logout => "/logout",
            Route::Protected => "/protected",
            Route::Chat => "/demo/chat",
            Route::ChatMessages => "/demo/chat/messages",
            Route::ChatMessagesDemo => "/demo/chat/messages/demo",
            Route::ChatModeration => "/demo/chat/moderation",
            Route::Events => "/events",
            Route::ErrorTest => "/error-test",
            Route::Health => "/health",
            Route::PartialAuthStatus => "/partials/auth-status",
            Route::PartialSessionStatus => "/partials/session-status",
            Route::PartialRequestMeta => "/partials/request-meta",
            Route::PartialBoundaryCheck => "/partials/boundary-check",
            Route::PartialDbCheck => "/partials/db-check",
            Route::PartialPing => "/partials/ping",
            Route::PartialSurrealGuarded => "/partials/surreal-message-guarded",
            Route::PartialSurrealCancel => "/partials/surreal-message-cancel",
        }
    }

    pub fn with_query(self, query: &str) -> String {
        format!("{}?{}", self.as_str(), query)
    }
}
