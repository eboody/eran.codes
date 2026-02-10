use bon::Builder;
use maud::Render;

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub enum HighlightCategory {
    AuthSessions,
    Realtime,
    ArchitectureTracing,
}

impl HighlightCategory {
    fn title(self) -> &'static str {
        match self {
            HighlightCategory::AuthSessions => "Auth + sessions",
            HighlightCategory::Realtime => "Realtime UX",
            HighlightCategory::ArchitectureTracing => "Architecture + tracing",
        }
    }

    fn bullets(self) -> &'static [&'static str] {
        match self {
            HighlightCategory::AuthSessions => &[
                "axum-login with SQLx-backed session store",
                "Signed session cookies (HTTP-only, SameSite Lax)",
                "Argon2 password hashing with credentials table",
                "Session cleanup task for expired records",
            ],
            HighlightCategory::Realtime => &[
                "Single SSE stream per visitor",
                "Datastar patches for signals + fragments",
                "Scoped CSS for safe inline styling",
                "Server-rendered HTML with progressive enhancement",
            ],
            HighlightCategory::ArchitectureTracing => &[
                "Domain/app/infra/http boundaries enforced",
                "Request spans with request-id and user-id",
                "Centralized error rendering",
                "Config-driven wiring in the binary root",
            ],
        }
    }
}

#[derive(Clone, Debug, Builder)]
pub struct HighlightsSection {
    pub categories: Vec<HighlightCategory>,
}

impl Render for HighlightsSection {
    fn render(&self) -> maud::Markup {
        maud::html! {
            section {
                h2 { "Implementation highlights" }
                div class="grid highlights" {
                    @for category in &self.categories {
                        article {
                            h3 { (category.title()) }
                            ul {
                                @for bullet in category.bullets() {
                                    li { (bullet) }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
