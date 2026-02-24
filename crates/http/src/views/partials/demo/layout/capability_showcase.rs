use bon::Builder;
use maud::Render;

use crate::types::Text;

use super::{
    TabbedShowcase, TabbedShowcaseAction, TabbedShowcaseMockPanel,
    TabbedShowcaseRow, TabbedShowcaseTab,
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
                    "Each tab is a focused demo surface you can use to present design decisions, architecture, and delivery approach.",
                ),
            )
            .tabs(vec![
                TabbedShowcaseTab::builder()
                    .tab_icon(Text::from("[]"))
                    .tab_label(Text::from("Secure Remote Access"))
                    .title(Text::from("Secure Remote Access"))
                    .subtitle(Text::from("Enable least-privilege network access in a few clicks."))
                    .bullets(vec![
                        Text::from("Provision users and groups from your identity provider"),
                        Text::from("Segment your network by grouping teams and infra"),
                        Text::from("Define granular policies to limit network access"),
                        Text::from("Enforce MFA and device security posture checks"),
                    ])
                    .mock_panel(
                        TabbedShowcaseMockPanel::builder()
                            .title(Text::from("Create New Access Policy"))
                            .subtitle(Text::from("Use this policy to control access groups and resources."))
                            .rows(vec![
                                TabbedShowcaseRow::builder().label(Text::from("Protocol")).value(Text::from("TCP")).build(),
                                TabbedShowcaseRow::builder().label(Text::from("Source")).value(Text::from("IT Department")).build(),
                                TabbedShowcaseRow::builder().label(Text::from("Destination")).value(Text::from("AWS - Servers")).build(),
                                TabbedShowcaseRow::builder().label(Text::from("Ports")).value(Text::from("443, 22")).build(),
                            ])
                            .build(),
                    )
                    .chips_label(Text::from("Integrates with"))
                    .chips(vec![Text::from("Azure"), Text::from("Google"), Text::from("Okta")])
                    .action(
                        TabbedShowcaseAction::builder()
                            .label(Text::from("Explore"))
                            .href(Text::from("#live-chat-demo"))
                            .build(),
                    )
                    .build(),
                TabbedShowcaseTab::builder()
                    .tab_icon(Text::from("::"))
                    .tab_label(Text::from("Zero-Config Deployment"))
                    .title(Text::from("Zero-Config Deployment"))
                    .subtitle(Text::from("Bootstrap environments fast with repeatable defaults."))
                    .bullets(vec![
                        Text::from("Apply baseline templates for new environments"),
                        Text::from("Keep rollout behavior consistent across regions"),
                        Text::from("Ship updates with predictable rollback paths"),
                        Text::from("Move from local demo to hosted deployment quickly"),
                    ])
                    .mock_panel(
                        TabbedShowcaseMockPanel::builder()
                            .title(Text::from("Create New Deployment Profile"))
                            .subtitle(Text::from("Use standardized setup templates per environment."))
                            .rows(vec![
                                TabbedShowcaseRow::builder().label(Text::from("Template")).value(Text::from("Production Baseline")).build(),
                                TabbedShowcaseRow::builder().label(Text::from("Environment")).value(Text::from("Edge Cluster A")).build(),
                                TabbedShowcaseRow::builder().label(Text::from("Verification")).value(Text::from("Policy + Health Checks")).build(),
                                TabbedShowcaseRow::builder().label(Text::from("Rollback")).value(Text::from("Automatic on failed checks")).build(),
                            ])
                            .build(),
                    )
                    .chips_label(Text::from("Built with"))
                    .chips(vec![Text::from("Docker"), Text::from("Coolify"), Text::from("Postgres")])
                    .action(
                        TabbedShowcaseAction::builder()
                            .label(Text::from("Explore"))
                            .href(Text::from("#live-chat-demo"))
                            .build(),
                    )
                    .build(),
                TabbedShowcaseTab::builder()
                    .tab_icon(Text::from("()"))
                    .tab_label(Text::from("Seamless SSO with MFA"))
                    .title(Text::from("Seamless SSO with MFA"))
                    .subtitle(Text::from("Connect identity providers and enforce strong auth policies."))
                    .bullets(vec![
                        Text::from("Use SSO for primary identity and role mapping"),
                        Text::from("Require MFA on privileged operations"),
                        Text::from("Capture auth traces for incident review"),
                        Text::from("Keep session handling explicit and durable"),
                    ])
                    .mock_panel(
                        TabbedShowcaseMockPanel::builder()
                            .title(Text::from("Create New Auth Policy"))
                            .subtitle(Text::from("Use this policy to configure sign-in and security steps."))
                            .rows(vec![
                                TabbedShowcaseRow::builder().label(Text::from("Identity Provider")).value(Text::from("Google Workspace")).build(),
                                TabbedShowcaseRow::builder().label(Text::from("MFA")).value(Text::from("Required for admin actions")).build(),
                                TabbedShowcaseRow::builder().label(Text::from("Session TTL")).value(Text::from("12h with re-auth")).build(),
                                TabbedShowcaseRow::builder().label(Text::from("Audit")).value(Text::from("Enabled")).build(),
                            ])
                            .build(),
                    )
                    .chips_label(Text::from("Connected to"))
                    .chips(vec![Text::from("Google Workspace"), Text::from("GitHub"), Text::from("OIDC")])
                    .action(
                        TabbedShowcaseAction::builder()
                            .label(Text::from("Explore"))
                            .href(Text::from("#live-chat-demo"))
                            .build(),
                    )
                    .build(),
                TabbedShowcaseTab::builder()
                    .tab_icon(Text::from("<>"))
                    .tab_label(Text::from("Dynamic Posture Checks"))
                    .title(Text::from("Dynamic Posture Checks"))
                    .subtitle(Text::from("Gate access using device state and runtime verification."))
                    .bullets(vec![
                        Text::from("Evaluate context before allowing access"),
                        Text::from("Block stale, risky, or unknown devices"),
                        Text::from("Re-check posture as session state changes"),
                        Text::from("Keep policy and enforcement boundaries clear"),
                    ])
                    .mock_panel(
                        TabbedShowcaseMockPanel::builder()
                            .title(Text::from("Create New Posture Rule"))
                            .subtitle(Text::from("Define required device checks and conditions."))
                            .rows(vec![
                                TabbedShowcaseRow::builder().label(Text::from("Device Status")).value(Text::from("Healthy")).build(),
                                TabbedShowcaseRow::builder().label(Text::from("Disk Encryption")).value(Text::from("Verified")).build(),
                                TabbedShowcaseRow::builder().label(Text::from("OS Patch Level")).value(Text::from("Current")).build(),
                                TabbedShowcaseRow::builder().label(Text::from("Policy Decision")).value(Text::from("Allow")).build(),
                            ])
                            .build(),
                    )
                    .chips_label(Text::from("Evaluates"))
                    .chips(vec![Text::from("MDM"), Text::from("Endpoint"), Text::from("Policy Engine")])
                    .action(
                        TabbedShowcaseAction::builder()
                            .label(Text::from("Explore"))
                            .href(Text::from("#live-chat-demo"))
                            .build(),
                    )
                    .build(),
                TabbedShowcaseTab::builder()
                    .tab_icon(Text::from("{}"))
                    .tab_label(Text::from("Centralized Network Management"))
                    .title(Text::from("Centralized Network Management"))
                    .subtitle(Text::from("Model teams, services, and routes from one control plane."))
                    .bullets(vec![
                        Text::from("Represent routes and ownership in one place"),
                        Text::from("Separate policy orchestration from transport"),
                        Text::from("Track resource relationships with typed models"),
                        Text::from("Reduce config drift between environments"),
                    ])
                    .mock_panel(
                        TabbedShowcaseMockPanel::builder()
                            .title(Text::from("Create Network Control Model"))
                            .subtitle(Text::from("Capture ownership and route intent for each segment."))
                            .rows(vec![
                                TabbedShowcaseRow::builder().label(Text::from("Teams")).value(Text::from("Platform, Product, Security")).build(),
                                TabbedShowcaseRow::builder().label(Text::from("Networks")).value(Text::from("Shared + Isolated Segments")).build(),
                                TabbedShowcaseRow::builder().label(Text::from("Gateway")).value(Text::from("Regional Pair")).build(),
                                TabbedShowcaseRow::builder().label(Text::from("Policy Source")).value(Text::from("GitOps")).build(),
                            ])
                            .build(),
                    )
                    .chips_label(Text::from("Coordinates"))
                    .chips(vec![Text::from("VPC"), Text::from("Service Mesh"), Text::from("DNS")])
                    .action(
                        TabbedShowcaseAction::builder()
                            .label(Text::from("Explore"))
                            .href(Text::from("#live-chat-demo"))
                            .build(),
                    )
                    .build(),
                TabbedShowcaseTab::builder()
                    .tab_icon(Text::from("||"))
                    .tab_label(Text::from("Detailed Activity Logging"))
                    .title(Text::from("Detailed Activity Logging"))
                    .subtitle(Text::from("Track who did what, where, and when across every action."))
                    .bullets(vec![
                        Text::from("Capture request + DB + SSE events end-to-end"),
                        Text::from("Stream key events into live operational views"),
                        Text::from("Keep diagnostic detail available for deep debugging"),
                        Text::from("Use structured fields for faster incident triage"),
                    ])
                    .mock_panel(
                        TabbedShowcaseMockPanel::builder()
                            .title(Text::from("Create Observability Profile"))
                            .subtitle(Text::from("Define signal classes and retention expectations."))
                            .rows(vec![
                                TabbedShowcaseRow::builder().label(Text::from("Request Trace")).value(Text::from("Enabled")).build(),
                                TabbedShowcaseRow::builder().label(Text::from("DB Logging")).value(Text::from("Statement + Duration")).build(),
                                TabbedShowcaseRow::builder().label(Text::from("SSE Events")).value(Text::from("Live Fanout")).build(),
                                TabbedShowcaseRow::builder().label(Text::from("Retention")).value(Text::from("Operational Window + Audit")).build(),
                            ])
                            .build(),
                    )
                    .chips_label(Text::from("Pipelines"))
                    .chips(vec![Text::from("Tracing"), Text::from("SSE"), Text::from("Audit Log")])
                    .action(
                        TabbedShowcaseAction::builder()
                            .label(Text::from("Explore"))
                            .href(Text::from("#live-chat-demo"))
                            .build(),
                    )
                    .build(),
            ])
            .build()
            .render()
    }
}
