use std::str::FromStr;
use maud::Render;
use strum_macros::{AsRefStr, Display, EnumString};

#[derive(Clone, Copy, Debug, Display, EnumString, AsRefStr)]
pub enum Route {
    #[strum(serialize = "/")]
    Home,
    #[strum(serialize = "/login")]
    Login,
    #[strum(serialize = "/register")]
    Register,
    #[strum(serialize = "/logout")]
    Logout,
    #[strum(serialize = "/protected")]
    Protected,
    #[strum(serialize = "/demo/chat")]
    Chat,
    #[strum(serialize = "/demo/chat/messages")]
    ChatMessages,
    #[strum(serialize = "/demo/chat/messages/demo")]
    ChatMessagesDemo,
    #[strum(serialize = "/demo/chat/moderation")]
    ChatModeration,
    #[strum(serialize = "/events")]
    Events,
    #[strum(serialize = "/error-test")]
    ErrorTest,
    #[strum(serialize = "/health")]
    Health,
    #[strum(serialize = "/partials/auth-status")]
    PartialAuthStatus,
    #[strum(serialize = "/partials/session-status")]
    PartialSessionStatus,
    #[strum(serialize = "/partials/request-meta")]
    PartialRequestMeta,
    #[strum(serialize = "/partials/boundary-check")]
    PartialBoundaryCheck,
    #[strum(serialize = "/partials/db-check")]
    PartialDbCheck,
    #[strum(serialize = "/partials/ping")]
    PartialPing,
    #[strum(serialize = "/partials/surreal-message-guarded")]
    PartialSurrealGuarded,
    #[strum(serialize = "/partials/surreal-message-cancel")]
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

    pub fn from_path(path: &str) -> Option<Self> {
        Self::from_str(path).ok()
    }

    pub fn with_query(self, query: &str) -> String {
        format!("{}?{}", self.as_str(), query)
    }
}

impl Render for Route {
    fn render(&self) -> maud::Markup {
        maud::html! { (self.as_str()) }
    }
}
