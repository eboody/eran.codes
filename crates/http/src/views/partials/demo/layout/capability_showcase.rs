use bon::Builder;
use maud::Render;

use crate::paths::Route;
use crate::types::Text;

use super::{
    TabbedShowcase, TabbedShowcaseAction, TabbedShowcaseMockPanel, TabbedShowcaseRow,
    TabbedShowcaseTab,
};

#[derive(Clone, Debug, Builder)]
pub struct CapabilityShowcase {}

impl Render for CapabilityShowcase {
    fn render(&self) -> maud::Markup {
        TabbedShowcase::builder()
            .id(Text::from("portfolio-showcase"))
            .title(Text::from("Capability Showcase"))
            .subtitle(
                Text::from(
                    "Each tab maps to a concrete capability in this workspace: auth durability, boundary-safe flows, observability, and live chat delivery.",
                ),
            )
            .tabs(vec![
                TabbedShowcaseTab::builder()
                    .tab_icon(Text::from("ID"))
                    .tab_label(Text::from("Identity + Sessions"))
                    .title(Text::from("Identity and Session Durability"))
                    .subtitle(Text::from("Encrypted cookies and durable Postgres-backed sessions keep identity consistent across requests."))
                    .bullets(vec![
                        Text::from("axum-login provider + tower-sessions for auth state"),
                        Text::from("Session cookie is signed/encrypted and HTTP-only"),
                        Text::from("Credential hashing is isolated behind app traits"),
                        Text::from("Tracing attaches user/session context to requests"),
                    ])
                    .mock_panel(
                        TabbedShowcaseMockPanel::builder()
                            .title(Text::from("Auth Session Snapshot"))
                            .subtitle(Text::from("Example runtime facts from the active auth/session stack."))
                            .rows(vec![
                                TabbedShowcaseRow::builder().label(Text::from("Cookie")).value(Text::from("session_id · http-only")).build(),
                                TabbedShowcaseRow::builder().label(Text::from("Session store")).value(Text::from("postgres + SQLx")).build(),
                                TabbedShowcaseRow::builder().label(Text::from("Auth hash")).value(Text::from("dedicated session hash")).build(),
                                TabbedShowcaseRow::builder().label(Text::from("Lifecycle")).value(Text::from("expiry + cleanup task")).build(),
                            ])
                            .build(),
                    )
                    .chips_label(Text::from("Built with"))
                    .chips(vec![Text::from("axum-login"), Text::from("tower-sessions"), Text::from("postgres")])
                    .action(
                        TabbedShowcaseAction::builder()
                            .label(Text::from("Open auth flow"))
                            .href(Text::from(Route::Register.as_str()))
                            .build(),
                    )
                    .build(),
                TabbedShowcaseTab::builder()
                    .tab_icon(Text::from("BD"))
                    .tab_label(Text::from("Boundary-Safe Flows"))
                    .title(Text::from("Boundary-Safe Request Flow"))
                    .subtitle(Text::from("Transport, policy, domain invariants, and persistence stay separated by typed handoffs."))
                    .bullets(vec![
                        Text::from("HTTP DTOs parse untrusted input"),
                        Text::from("App commands orchestrate use-case policy"),
                        Text::from("Domain newtypes enforce invariants"),
                        Text::from("Infra owns SQL rows and concrete mechanisms"),
                    ])
                    .mock_panel(
                        TabbedShowcaseMockPanel::builder()
                            .title(Text::from("Request Handoff Map"))
                            .subtitle(Text::from("A single chat request moving through each layer."))
                            .rows(vec![
                                TabbedShowcaseRow::builder().label(Text::from("HTTP")).value(Text::from("signals -> request DTO")).build(),
                                TabbedShowcaseRow::builder().label(Text::from("App")).value(Text::from("PostMessage command")).build(),
                                TabbedShowcaseRow::builder().label(Text::from("Domain")).value(Text::from("RoomId, MessageBody, UserId")).build(),
                                TabbedShowcaseRow::builder().label(Text::from("Infra")).value(Text::from("repo + SQL + audit")).build(),
                            ])
                            .build(),
                    )
                    .chips_label(Text::from("Crates"))
                    .chips(vec![Text::from("domain"), Text::from("app"), Text::from("infra"), Text::from("http")])
                    .action(
                        TabbedShowcaseAction::builder()
                            .label(Text::from("See code-level breakdown"))
                            .href(Text::from("#professionalism-practice"))
                            .build(),
                    )
                    .build(),
                TabbedShowcaseTab::builder()
                    .tab_icon(Text::from("OBS"))
                    .tab_label(Text::from("Observability + SSE"))
                    .title(Text::from("Observability and Real-Time Delivery"))
                    .subtitle(Text::from("Request spans, live logs, and SSE patches make behavior inspectable while the app runs."))
                    .bullets(vec![
                        Text::from("Live and diagnostic streams are intentionally separated"),
                        Text::from("Network log shows status/method/path/latency"),
                        Text::from("SSE events track selector/mode/payload bytes"),
                        Text::from("Trace fields include request/session/user context"),
                    ])
                    .mock_panel(
                        TabbedShowcaseMockPanel::builder()
                            .title(Text::from("Live Event Snapshot"))
                            .subtitle(Text::from("What is emitted while a chat message is posted and broadcast."))
                            .rows(vec![
                                TabbedShowcaseRow::builder().label(Text::from("request end")).value(Text::from("status + latency + route")).build(),
                                TabbedShowcaseRow::builder().label(Text::from("chat incoming")).value(Text::from("sender + payload_bytes")).build(),
                                TabbedShowcaseRow::builder().label(Text::from("chat broadcast")).value(Text::from("selector=[data-chat-messages]")).build(),
                                TabbedShowcaseRow::builder().label(Text::from("network panel")).value(Text::from("SSE + HTTP tables")).build(),
                            ])
                            .build(),
                    )
                    .chips_label(Text::from("Signals"))
                    .chips(vec![Text::from("tracing"), Text::from("sse"), Text::from("datastar"), Text::from("log panels")])
                    .action(
                        TabbedShowcaseAction::builder()
                            .label(Text::from("Open network log"))
                            .href(Text::from("#network-log-target"))
                            .build(),
                    )
                    .build(),
                TabbedShowcaseTab::builder()
                    .tab_icon(Text::from("CHAT"))
                    .tab_label(Text::from("Live Chat Capstone"))
                    .title(Text::from("Live Chat Capstone"))
                    .subtitle(Text::from("A complete request -> validate -> persist -> broadcast path with moderation and rate limiting."))
                    .bullets(vec![
                        Text::from("Messages persist to Postgres and reload on entry"),
                        Text::from("Rate limits and moderation are enforced in app services"),
                        Text::from("SSE pushes append updates to all connected tabs"),
                        Text::from("Moderation queue surfaces pending decisions"),
                    ])
                    .mock_panel(
                        TabbedShowcaseMockPanel::builder()
                            .title(Text::from("Chat Flow Runtime"))
                            .subtitle(Text::from("Core controls in the chat path."))
                            .rows(vec![
                                TabbedShowcaseRow::builder().label(Text::from("Request")).value(Text::from("POST /demo/chat/messages")).build(),
                                TabbedShowcaseRow::builder().label(Text::from("Validation")).value(Text::from("typed ids + message body")).build(),
                                TabbedShowcaseRow::builder().label(Text::from("Persistence")).value(Text::from("chat_messages + audit")).build(),
                                TabbedShowcaseRow::builder().label(Text::from("Fanout")).value(Text::from("prepend [data-chat-messages]")).build(),
                            ])
                            .build(),
                    )
                    .chips_label(Text::from("Includes"))
                    .chips(vec![Text::from("persistence"), Text::from("rate limit"), Text::from("moderation"), Text::from("sse fanout")])
                    .action(
                        TabbedShowcaseAction::builder()
                            .label(Text::from("Jump to live chat"))
                            .href(Text::from("#chat-demo"))
                            .build(),
                    )
                    .build(),
            ])
            .build()
            .render()
    }
}
