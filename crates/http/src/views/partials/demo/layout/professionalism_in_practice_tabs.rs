use bon::Builder;
use maud::Render;

use crate::types::Text;

use super::{TabbedShowcase, TabbedShowcaseTab};

#[derive(Clone, Debug, Builder)]
pub struct ProfessionalismInPracticeTabs {}

impl Render for ProfessionalismInPracticeTabs {
    fn render(&self) -> maud::Markup {
        TabbedShowcase::builder()
            .id(Text::from("professionalism-practice"))
            .title(Text::from("Professionalism In Practice (Detailed Breakdown)"))
            .subtitle(
                Text::from(
                    "Concrete patterns from this codebase, with real snippets and why each choice is maintainable.",
                ),
            )
            .tabs(vec![
                TabbedShowcaseTab::builder()
                    .tab_label(Text::from("Boundary-first modeling"))
                    .title(Text::from("Boundary-first modeling"))
                    .subtitle(Text::from("App defines contracts and infra implements mechanisms, so policy stays stable as storage evolves."))
                    .bullets(vec![
                        Text::from("Core policy flows are insulated from transport and SQL details."),
                        Text::from("Repository traits in app enforce dependency direction."),
                        Text::from("Domain entities avoid HTTP/database concerns."),
                    ])
                    .maybe_mock_panel(None)
                    .chips_label(Text::from("Scope"))
                    .chips(vec![Text::from("domain"), Text::from("app"), Text::from("infra"), Text::from("http")])
                    .code_path(Text::from("crates/app/src/chat/mod.rs"))
                    .code(Text::from(
                        "pub trait ChatRepository: Send + Sync {\n    async fn create_room(\n        &self,\n        name: room::RoomName,\n        created_by: room::UserId,\n    ) -> Result<room::Room>;\n}",
                    ))
                    .build(),
                TabbedShowcaseTab::builder()
                    .tab_label(Text::from("Typed invariants"))
                    .title(Text::from("Typed invariants over stringly logic"))
                    .subtitle(Text::from("Enums and newtypes move correctness checks to compile time and reduce typo-prone runtime branching."))
                    .bullets(vec![
                        Text::from("Serialization values are mapped via enum variants."),
                        Text::from("State transitions stay explicit and searchable."),
                        Text::from("Invalid states are unrepresentable in regular paths."),
                    ])
                    .maybe_mock_panel(None)
                    .chips_label(Text::from("Pattern"))
                    .chips(vec![Text::from("enum"), Text::from("newtype"), Text::from("strum")])
                    .code_path(Text::from("crates/domain/src/chat/message.rs"))
                    .code(Text::from(
                        "#[derive(Debug, Clone, Copy, PartialEq, Eq, strum_macros::Display, strum_macros::EnumString)]\npub enum MessageStatus {\n    #[strum(serialize = \"visible\")]\n    Visible,\n    #[strum(serialize = \"pending\")]\n    Pending,\n    #[strum(serialize = \"removed\")]\n    Removed,\n}",
                    ))
                    .build(),
                TabbedShowcaseTab::builder()
                    .tab_label(Text::from("Error contracts"))
                    .title(Text::from("Centralized error contracts"))
                    .subtitle(Text::from("Handlers stay focused on workflow while response behavior remains consistent across pages and partials."))
                    .bullets(vec![
                        Text::from("A single HTTP error type maps app/domain failures."),
                        Text::from("Datastar vs full-page behavior stays consistent."),
                        Text::from("Cross-cutting response behavior is easier to review."),
                    ])
                    .maybe_mock_panel(None)
                    .chips_label(Text::from("Outcome"))
                    .chips(vec![Text::from("consistency"), Text::from("clarity"), Text::from("safety")])
                    .code_path(Text::from("crates/http/src/error.rs"))
                    .code(Text::from(
                        "pub enum Error {\n    Internal,\n    Unauthorized,\n    Validation(Text),\n    Chat(app::chat::Error),\n    User(app::user::Error),\n    Auth(app::auth::Error),\n}",
                    ))
                    .build(),
                TabbedShowcaseTab::builder()
                    .tab_label(Text::from("Observability"))
                    .title(Text::from("Observability designed as architecture"))
                    .subtitle(Text::from("Live and diagnostic streams are intentionally separated so users see signal instead of noise."))
                    .bullets(vec![
                        Text::from("Known target enums enforce naming discipline."),
                        Text::from("Request context fields are added consistently."),
                        Text::from("Live panel remains concise while diagnostic depth stays available."),
                    ])
                    .maybe_mock_panel(None)
                    .chips_label(Text::from("Built on"))
                    .chips(vec![Text::from("tracing"), Text::from("SSE"), Text::from("typed targets")])
                    .code_path(Text::from("crates/http/src/trace_log.rs"))
                    .code(Text::from(
                        "#[derive(Clone, Copy, Debug, PartialEq, Eq, Display, EnumString)]\npub enum LogTargetKnown {\n    #[strum(serialize = \"demo.request\")]\n    DemoRequest,\n    #[strum(serialize = \"demo.db\")]\n    DemoDb,\n    #[strum(serialize = \"demo.sse\")]\n    DemoSse,\n    #[strum(serialize = \"http::router::layers\")]\n    RouterLayers,\n}",
                    ))
                    .build(),
                TabbedShowcaseTab::builder()
                    .tab_label(Text::from("Reusable views"))
                    .title(Text::from("Reusable view components with typed inputs"))
                    .subtitle(Text::from("Shared typed components reduce duplication and make broad UI changes local and safe."))
                    .bullets(vec![
                        Text::from("View models are explicit and composable."),
                        Text::from("Large pages stay readable by composing `Render` partials."),
                        Text::from("UI iteration happens without touching transport logic."),
                    ])
                    .maybe_mock_panel(None)
                    .chips_label(Text::from("Technique"))
                    .chips(vec![Text::from("maud::Render"), Text::from("builder"), Text::from("typed props")])
                    .code_path(Text::from("crates/http/src/views/partials/demo/log/log_row.rs"))
                    .code(Text::from(
                        "#[derive(Clone, Debug, Builder)]\npub struct LogRow {\n    pub timestamp: Text,\n    pub level: maud::Markup,\n    pub status: Option<maud::Markup>,\n    pub method: Option<maud::Markup>,\n    pub path: Option<maud::Markup>,\n    pub message: Text,\n    pub extras: Vec<maud::Markup>,\n}",
                    ))
                    .build(),
                TabbedShowcaseTab::builder()
                    .tab_label(Text::from("Readable wiring"))
                    .title(Text::from("Builders as readable wiring"))
                    .subtitle(Text::from("Builder pipelines make dependency wiring self-documenting and less error-prone over time."))
                    .bullets(vec![
                        Text::from("Composition roots read as intent rather than boilerplate."),
                        Text::from("Constructor growth stays manageable."),
                        Text::from("Naming at the callsite clarifies ownership boundaries."),
                    ])
                    .maybe_mock_panel(None)
                    .chips_label(Text::from("Outcome"))
                    .chips(vec![Text::from("readability"), Text::from("maintainability"), Text::from("explicit wiring")])
                    .code_path(Text::from("crates/http/src/state.rs"))
                    .code(Text::from(
                        "#[builder]\npub fn from_parts(\n    #[builder(setters(name = with_user))] user: app::user::Service,\n    #[builder(setters(name = with_auth))] auth: app::auth::ProviderImpl,\n    #[builder(setters(name = with_chat))] chat: app::chat::Service,\n    #[builder(setters(name = with_sse))] sse: crate::sse::Registry,\n) -> Self",
                    ))
                    .build(),
            ])
            .build()
            .render()
    }
}
