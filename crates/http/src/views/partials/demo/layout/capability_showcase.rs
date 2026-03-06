use bon::Builder;
use maud::Render;

use crate::paths::Route;
use crate::types::Text;
use crate::views::partials::components::{Tab, TabInteraction, primitives::Icon};
use crate::views::proper_theme::THEME;

use super::tabbed_showcase;

#[derive(Clone, Debug, Builder)]
pub struct CapabilityShowcase {}

impl Render for CapabilityShowcase {
    fn render(&self) -> maud::Markup {
        maud::html! {
            (tabbed_showcase::builder()
                .id(Text::from("portfolio-showcase"))
                .theme(tabbed_showcase::Theme::netbird())
                .title(Text::from("Capability Showcase"))
                .subtitle(
                    Text::from(
                        "Each tab maps to a concrete capability in this workspace: auth durability, boundary-safe flows, observability, and live chat delivery.",
                    ),
                )
                .tabs(vec![
                    Tab {
                        id: Text::from("portfolio-showcase-tab-0"),
                        controls: Text::from("portfolio-showcase-panel-0"),
                        palette: &THEME.yellow,
                        is_selected: true,
                        icon: Some(Icon::from_token("heart")),
                        text: Text::from("Identity + Sessions"),
                        interaction: TabInteraction::PanelJs,
                    },
                    Tab {
                        id: Text::from("portfolio-showcase-tab-1"),
                        controls: Text::from("portfolio-showcase-panel-1"),
                        palette: &THEME.teal,
                        is_selected: false,
                        icon: Some(Icon::from_token("refresh-double")),
                        text: Text::from("Boundary-Safe Flows"),
                        interaction: TabInteraction::PanelJs,
                    },
                    Tab {
                        id: Text::from("portfolio-showcase-tab-2"),
                        controls: Text::from("portfolio-showcase-panel-2"),
                        palette: &THEME.green,
                        is_selected: false,
                        icon: Some(Icon::from_token("activity")),
                        text: Text::from("Observability + SSE"),
                        interaction: TabInteraction::PanelJs,
                    },
                    Tab {
                        id: Text::from("portfolio-showcase-tab-3"),
                        controls: Text::from("portfolio-showcase-panel-3"),
                        palette: &THEME.purple,
                        is_selected: false,
                        icon: Some(Icon::from_token("chat-lines")),
                        text: Text::from("Live Chat Capstone"),
                        interaction: TabInteraction::PanelJs,
                    },
                ])
                .panels(vec![
                    tabbed_showcase::Panel::builder()
                        .title(Text::from("Identity and Session Durability"))
                        .subtitle(Text::from("Encrypted cookies and durable Postgres-backed sessions keep identity consistent across requests."))
                        .bullets(vec![
                            Text::from("axum-login provider + tower-sessions for auth state"),
                            Text::from("Session cookie is signed/encrypted and HTTP-only"),
                            Text::from("Credential hashing is isolated behind app traits"),
                            Text::from("Tracing attaches user/session context to requests"),
                        ])
                        .mock_panel(
                            tabbed_showcase::MockPanel::builder()
                                .title(Text::from("Auth Session Snapshot"))
                                .subtitle(Text::from("Example runtime facts from the active auth/session stack."))
                                .rows(vec![
                                    tabbed_showcase::Row::builder().label(Text::from("Cookie")).value(Text::from("session_id · http-only")).build(),
                                    tabbed_showcase::Row::builder().label(Text::from("Session store")).value(Text::from("postgres + SQLx")).build(),
                                    tabbed_showcase::Row::builder().label(Text::from("Auth hash")).value(Text::from("dedicated session hash")).build(),
                                    tabbed_showcase::Row::builder().label(Text::from("Lifecycle")).value(Text::from("expiry + cleanup task")).build(),
                                ])
                                .build(),
                        )
                        .chips_label(Text::from("Built with"))
                        .chips(vec![Text::from("axum-login"), Text::from("tower-sessions"), Text::from("postgres")])
                        .action(
                            tabbed_showcase::Action::builder()
                                .label(Text::from("Open auth flow"))
                                .href(Text::from(Route::Register.as_str()))
                                .build(),
                        )
                        .build(),
                    tabbed_showcase::Panel::builder()
                        .title(Text::from("Boundary-Safe Request Flow"))
                        .subtitle(Text::from("Transport, policy, domain invariants, and persistence stay separated by typed handoffs."))
                        .bullets(vec![
                            Text::from("HTTP DTOs parse untrusted input"),
                            Text::from("App commands orchestrate use-case policy"),
                            Text::from("Domain newtypes enforce invariants"),
                            Text::from("Infra owns SQL rows and concrete mechanisms"),
                        ])
                        .mock_panel(
                            tabbed_showcase::MockPanel::builder()
                                .title(Text::from("Request Handoff Map"))
                                .subtitle(Text::from("A single chat request moving through each layer."))
                                .rows(vec![
                                    tabbed_showcase::Row::builder().label(Text::from("HTTP")).value(Text::from("signals -> request DTO")).build(),
                                    tabbed_showcase::Row::builder().label(Text::from("App")).value(Text::from("PostMessage command")).build(),
                                    tabbed_showcase::Row::builder().label(Text::from("Domain")).value(Text::from("RoomId, MessageBody, UserId")).build(),
                                    tabbed_showcase::Row::builder().label(Text::from("Infra")).value(Text::from("repo + SQL + audit")).build(),
                                ])
                                .build(),
                        )
                        .chips_label(Text::from("Crates"))
                        .chips(vec![Text::from("domain"), Text::from("app"), Text::from("infra"), Text::from("http")])
                        .action(
                            tabbed_showcase::Action::builder()
                                .label(Text::from("See code-level breakdown"))
                                .href(Text::from("#professionalism-practice"))
                                .build(),
                        )
                        .build(),
                    tabbed_showcase::Panel::builder()
                        .title(Text::from("Observability and Real-Time Delivery"))
                        .subtitle(Text::from("Request spans, live logs, and SSE patches make behavior inspectable while the app runs."))
                        .bullets(vec![
                            Text::from("Live and diagnostic streams are intentionally separated"),
                            Text::from("Network log shows status/method/path/latency"),
                            Text::from("SSE events track selector/mode/payload bytes"),
                            Text::from("Trace fields include request/session/user context"),
                        ])
                        .mock_panel(
                            tabbed_showcase::MockPanel::builder()
                                .title(Text::from("Live Event Snapshot"))
                                .subtitle(Text::from("What is emitted while a chat message is posted and broadcast."))
                                .rows(vec![
                                    tabbed_showcase::Row::builder().label(Text::from("request end")).value(Text::from("status + latency + route")).build(),
                                    tabbed_showcase::Row::builder().label(Text::from("chat incoming")).value(Text::from("sender + payload_bytes")).build(),
                                    tabbed_showcase::Row::builder().label(Text::from("chat broadcast")).value(Text::from("selector=[data-chat-messages]")).build(),
                                    tabbed_showcase::Row::builder().label(Text::from("network panel")).value(Text::from("SSE + HTTP tables")).build(),
                                ])
                                .build(),
                        )
                        .chips_label(Text::from("Signals"))
                        .chips(vec![Text::from("tracing"), Text::from("sse"), Text::from("datastar"), Text::from("log panels")])
                        .action(
                            tabbed_showcase::Action::builder()
                                .label(Text::from("Open network log"))
                                .href(Text::from("#network-log-target"))
                                .build(),
                        )
                        .build(),
                    tabbed_showcase::Panel::builder()
                        .title(Text::from("Live Chat Capstone"))
                        .subtitle(Text::from("A complete request -> validate -> persist -> broadcast path with moderation and rate limiting."))
                        .bullets(vec![
                            Text::from("Messages persist to Postgres and reload on entry"),
                            Text::from("Rate limits and moderation are enforced in app services"),
                            Text::from("SSE pushes append updates to all connected tabs"),
                            Text::from("Moderation queue surfaces pending decisions"),
                        ])
                        .mock_panel(
                            tabbed_showcase::MockPanel::builder()
                                .title(Text::from("Chat Flow Runtime"))
                                .subtitle(Text::from("Core controls in the chat path."))
                                .rows(vec![
                                    tabbed_showcase::Row::builder().label(Text::from("Request")).value(Text::from("POST /demo/chat/messages")).build(),
                                    tabbed_showcase::Row::builder().label(Text::from("Validation")).value(Text::from("typed ids + message body")).build(),
                                    tabbed_showcase::Row::builder().label(Text::from("Persistence")).value(Text::from("chat_messages + audit")).build(),
                                    tabbed_showcase::Row::builder().label(Text::from("Fanout")).value(Text::from("prepend [data-chat-messages]")).build(),
                                ])
                                .build(),
                        )
                        .chips_label(Text::from("Includes"))
                        .chips(vec![Text::from("persistence"), Text::from("rate limit"), Text::from("moderation"), Text::from("sse fanout")])
                        .action(
                            tabbed_showcase::Action::builder()
                                .label(Text::from("Jump to live chat"))
                                .href(Text::from("#chat-demo"))
                                .build(),
                        )
                        .build(),
                ])
                .build())
        }
    }
}
