use bon::Builder;
use maud::Render;

use crate::types::Text;
use crate::views::partials::components::{Tab, TabInteraction};
use crate::views::proper_theme::THEME;

use super::tabbed_showcase;

#[derive(Clone, Debug, Builder)]
pub struct ProfessionalismInPracticeTabs {}

impl Render for ProfessionalismInPracticeTabs {
    fn render(&self) -> maud::Markup {
        maud::html! {
            (tabbed_showcase::builder()
                .id(Text::from("professionalism-practice"))
                .theme(tabbed_showcase::Theme::netbird_detail())
                .title(Text::from("Professionalism In Practice (Detailed Breakdown)"))
                .subtitle(
                    Text::from(
                        "Concrete patterns from this codebase, with real snippets and why each choice is maintainable.",
                    ),
                )
                .tabs(vec![
                    Tab {
                        id: Text::from("professionalism-practice-tab-0"),
                        controls: Text::from("professionalism-practice-panel-0"),
                        palette: &THEME.gray,
                        is_selected: true,
                        icon: None,
                        text: Text::from("Boundary-first modeling"),
                        interaction: TabInteraction::PanelJs,
                    },
                    Tab {
                        id: Text::from("professionalism-practice-tab-1"),
                        controls: Text::from("professionalism-practice-panel-1"),
                        palette: &THEME.gray,
                        is_selected: false,
                        icon: None,
                        text: Text::from("Typed invariants"),
                        interaction: TabInteraction::PanelJs,
                    },
                    Tab {
                        id: Text::from("professionalism-practice-tab-2"),
                        controls: Text::from("professionalism-practice-panel-2"),
                        palette: &THEME.gray,
                        is_selected: false,
                        icon: None,
                        text: Text::from("Error contracts"),
                        interaction: TabInteraction::PanelJs,
                    },
                    Tab {
                        id: Text::from("professionalism-practice-tab-3"),
                        controls: Text::from("professionalism-practice-panel-3"),
                        palette: &THEME.gray,
                        is_selected: false,
                        icon: None,
                        text: Text::from("Observability"),
                        interaction: TabInteraction::PanelJs,
                    },
                    Tab {
                        id: Text::from("professionalism-practice-tab-4"),
                        controls: Text::from("professionalism-practice-panel-4"),
                        palette: &THEME.gray,
                        is_selected: false,
                        icon: None,
                        text: Text::from("Reusable views"),
                        interaction: TabInteraction::PanelJs,
                    },
                    Tab {
                        id: Text::from("professionalism-practice-tab-5"),
                        controls: Text::from("professionalism-practice-panel-5"),
                        palette: &THEME.gray,
                        is_selected: false,
                        icon: None,
                        text: Text::from("Readable wiring"),
                        interaction: TabInteraction::PanelJs,
                    },
                ])
                .panels(vec![
                    tabbed_showcase::Panel::builder()
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
                    tabbed_showcase::Panel::builder()
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
                    tabbed_showcase::Panel::builder()
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
                    tabbed_showcase::Panel::builder()
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
                    tabbed_showcase::Panel::builder()
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
                        .code_path(Text::from("crates/http/src/views/partials/demo/log/row.rs"))
                        .code(Text::from(
                            "#[derive(Clone, Debug, Builder)]\npub struct Row {\n    pub timestamp: Text,\n    pub message: Text,\n    #[builder(default)]\n    pub pills: Vec<Pill>,\n}",
                        ))
                        .build(),
                    tabbed_showcase::Panel::builder()
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
                .build())
        }
    }
}
