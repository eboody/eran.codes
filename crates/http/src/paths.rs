use maud::Render;
use strum_macros::{AsRefStr, Display, EnumString};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Display, EnumString, AsRefStr)]
pub enum Route {
    #[strum(serialize = "/")]
    Home,
    #[strum(serialize = "/lab")]
    Lab,
    #[strum(serialize = "/work")]
    Work,
    #[strum(serialize = "/open-source")]
    OpenSource,
    #[strum(serialize = "/resume.txt")]
    ResumeText,
    #[strum(serialize = "/work/chat-realtime")]
    WorkChatRealtime,
    #[strum(serialize = "/work/command-sse")]
    WorkCommandSse,
    #[strum(serialize = "/work/operational-visibility")]
    WorkOperationalVisibility,
    #[strum(serialize = "/work/sensitive-sync")]
    WorkSensitiveSync,
    #[strum(serialize = "/login")]
    Login,
    #[strum(serialize = "/register")]
    Register,
    #[strum(serialize = "/logout")]
    Logout,
    #[strum(serialize = "/protected")]
    Protected,
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
    #[strum(serialize = "/partials/request-burst-probe")]
    PartialRequestBurstProbe,
    #[strum(serialize = "/partials/sensitive-proof")]
    PartialSensitiveProof,
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
            Route::Lab => "/lab",
            Route::Work => "/work",
            Route::OpenSource => "/open-source",
            Route::ResumeText => "/resume.txt",
            Route::WorkChatRealtime => "/work/chat-realtime",
            Route::WorkCommandSse => "/work/command-sse",
            Route::WorkOperationalVisibility => "/work/operational-visibility",
            Route::WorkSensitiveSync => "/work/sensitive-sync",
            Route::Login => "/login",
            Route::Register => "/register",
            Route::Logout => "/logout",
            Route::Protected => "/protected",
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
            Route::PartialRequestBurstProbe => "/partials/request-burst-probe",
            Route::PartialSensitiveProof => "/partials/sensitive-proof",
            Route::PartialPing => "/partials/ping",
            Route::PartialSurrealGuarded => "/partials/surreal-message-guarded",
            Route::PartialSurrealCancel => "/partials/surreal-message-cancel",
        }
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
