use bon::Builder;
use maud::Render;

use crate::types::Text;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProfessionalismTopic {
    BoundaryModeling,
    TypedInvariants,
    ErrorContracts,
    Observability,
    ReusableViews,
    ReadableWiring,
}

impl ProfessionalismTopic {
    fn title(self) -> Text {
        match self {
            Self::BoundaryModeling => Text::from("Boundary-first modeling"),
            Self::TypedInvariants => Text::from("Typed invariants over stringly logic"),
            Self::ErrorContracts => Text::from("Centralized error contracts"),
            Self::Observability => Text::from("Observability designed as architecture"),
            Self::ReusableViews => Text::from("Reusable view components with typed inputs"),
            Self::ReadableWiring => Text::from("Builders as readable wiring"),
        }
    }

    fn why(self) -> Text {
        match self {
            Self::BoundaryModeling => Text::from(
                "App defines contracts and infra implements mechanisms, so policy stays stable as storage evolves.",
            ),
            Self::TypedInvariants => Text::from(
                "Enums/newtypes move correctness checks to compile time and remove typo-prone runtime comparisons.",
            ),
            Self::ErrorContracts => Text::from(
                "Handlers stay focused on workflow while response behavior remains consistent across pages and partials.",
            ),
            Self::Observability => Text::from(
                "Live and diagnostic streams are intentionally separated so users see signal instead of noise.",
            ),
            Self::ReusableViews => Text::from(
                "Shared typed components reduce duplication and make broad UI changes local and safe.",
            ),
            Self::ReadableWiring => Text::from(
                "Builder pipelines make dependency wiring self-documenting and less error-prone over time.",
            ),
        }
    }

    fn path(self) -> Text {
        match self {
            Self::BoundaryModeling => Text::from("crates/app/src/chat/mod.rs"),
            Self::TypedInvariants => Text::from("crates/domain/src/chat/message.rs"),
            Self::ErrorContracts => Text::from("crates/http/src/error.rs"),
            Self::Observability => Text::from("crates/http/src/trace_log.rs"),
            Self::ReusableViews => Text::from(
                "crates/http/src/views/partials/demo/log/log_row.rs",
            ),
            Self::ReadableWiring => Text::from("crates/http/src/state.rs"),
        }
    }

    fn code(self) -> Text {
        match self {
            Self::BoundaryModeling => Text::from(
                "pub trait ChatRepository: Send + Sync {\n    async fn create_room(\n        &self,\n        name: room::RoomName,\n        created_by: room::UserId,\n    ) -> Result<room::Room>;\n}",
            ),
            Self::TypedInvariants => Text::from(
                "#[derive(Debug, Clone, Copy, PartialEq, Eq, strum_macros::Display, strum_macros::EnumString)]\npub enum MessageStatus {\n    #[strum(serialize = \"visible\")]\n    Visible,\n    #[strum(serialize = \"pending\")]\n    Pending,\n    #[strum(serialize = \"removed\")]\n    Removed,\n}",
            ),
            Self::ErrorContracts => Text::from(
                "pub enum Error {\n    Internal,\n    Unauthorized,\n    Validation(Text),\n    Chat(app::chat::Error),\n    User(app::user::Error),\n    Auth(app::auth::Error),\n}",
            ),
            Self::Observability => Text::from(
                "#[derive(Clone, Copy, Debug, PartialEq, Eq, Display, EnumString)]\npub enum LogTargetKnown {\n    #[strum(serialize = \"demo.request\")]\n    DemoRequest,\n    #[strum(serialize = \"demo.db\")]\n    DemoDb,\n    #[strum(serialize = \"demo.sse\")]\n    DemoSse,\n    #[strum(serialize = \"http::router::layers\")]\n    RouterLayers,\n}",
            ),
            Self::ReusableViews => Text::from(
                "#[derive(Clone, Debug, Builder)]\npub struct LogRow {\n    pub timestamp: Text,\n    pub level: maud::Markup,\n    pub status: Option<maud::Markup>,\n    pub method: Option<maud::Markup>,\n    pub path: Option<maud::Markup>,\n    pub message: Text,\n    pub extras: Vec<maud::Markup>,\n}",
            ),
            Self::ReadableWiring => Text::from(
                "#[builder]\npub fn from_parts(\n    #[builder(setters(name = with_user))] user: app::user::Service,\n    #[builder(setters(name = with_auth))] auth: app::auth::ProviderImpl,\n    #[builder(setters(name = with_chat))] chat: app::chat::Service,\n    #[builder(setters(name = with_sse))] sse: crate::sse::Registry,\n) -> Self",
            ),
        }
    }
}

#[derive(Clone, Debug, Builder)]
pub struct ProfessionalismBreakdown {
    pub topics: Vec<ProfessionalismTopic>,
}

impl Render for ProfessionalismBreakdown {
    fn render(&self) -> maud::Markup {
        maud::html! {
            div class="professionalism-accordion-list" {
                @for topic in &self.topics {
                    details class="professionalism-accordion" {
                        summary {
                            h3 { (topic.title()) }
                        }
                        p class="muted" { (topic.why()) }
                        p class="professionalism-path" {
                            "Example: "
                            code { (topic.path()) }
                        }
                        pre class="professionalism-code" {
                            code { (topic.code()) }
                        }
                    }
                }
            }
        }
    }
}
