use bon::Builder;
use maud::Render;

use crate::paths::Route;
use crate::types::Text;
use crate::views::{page, partials};

crate::views::scoped::inline_css!(
    r#"
me {
  display: grid;
  gap: var(--space-section);
  margin-top: clamp(1.1rem, 0.9rem + 0.9vw, 1.8rem);
  padding-bottom: calc(var(--space-section) + var(--space-6));
}

me > :where(header, section) {
  margin-top: 0;
  scroll-margin-top: var(--nav-scroll-offset);
}

me [data-chat-moderation-hero] {
  display: grid;
  gap: var(--space-3);
  margin-block: var(--space-5) var(--space-6);
  padding: var(--space-card);
  border-radius: var(--radius-card);
  border: 1px solid var(--portfolio-surface-border);
  background:
    linear-gradient(
      135deg,
      color-mix(in srgb, var(--portfolio-accent-a) 18%, transparent),
      transparent 58%
    ),
    var(--portfolio-surface);
  box-shadow: var(--portfolio-shadow);
  view-transition-name: chat-hero;
}

me [data-chat-moderation-hero] h1 {
  margin: 0;
  font-size: var(--text-size-hero-sm);
  line-height: var(--text-line-heading-relaxed);
}

me [data-chat-moderation-flow] {
  overflow: visible;
  border: 1px dashed var(--ui-border-muted);
  border-radius: var(--radius-card);
  padding: var(--space-card);
  background: var(--ui-surface-card);
}

me [data-chat-moderation-stack] {
  display: grid;
  gap: var(--space-3);
}

me [data-chat-moderation-card] {
  overflow: visible;
  border: 1px solid var(--ui-border-soft);
  border-radius: var(--ui-radius-md);
  background: var(--ui-surface-soft);
  padding: var(--space-4);
}

me [data-chat-moderation-card] > header h3 {
  margin-bottom: var(--space-1);
}

me [data-chat-moderation-card] .u-muted {
  color: color-mix(in srgb, var(--ui-text-muted) 94%, var(--ui-text) 6%);
}

me [data-chat-moderation-flow] .ui-button-row {
  margin-top: calc(var(--space-3) - var(--interactive-bleed));
}

me [data-chat-moderation-card] .button {
  position: relative;
  z-index: 0;
}

me [data-chat-moderation-card] .button:hover,
me [data-chat-moderation-card] .button:focus-visible {
  z-index: 1;
}

@media (max-width: 768px) {
  me [data-chat-moderation-card] .button {
    width: 100%;
  }
}

@media (max-width: 520px) {
  me [data-chat-moderation-flow] .ui-button-row > * {
    flex-basis: 100%;
  }
}
"#
);

#[derive(Builder)]
pub struct ChatModeration {
    pub entries: Vec<app::chat::moderation::Item>,
    #[builder(setters(name = with_user))]
    pub user: Option<page::UserNav>,
}

impl Render for ChatModeration {
    fn render(&self) -> maud::Markup {
        let content = maud::html! {
            div data-chat-page data-chat-moderation-page data-page-section {
                (css())
                header data-chat-moderation-hero {
                    div {
                        h1 { "Chat moderation queue" }
                        p { "Review pending messages and apply moderation decisions." }
                    }
                }

                section data-chat-moderation-flow {
                    @if self.entries.is_empty() {
                        p class="u-muted" { "No pending messages." }
                    } @else {
                        div data-chat-moderation-stack {
                            @for entry in &self.entries {
                                article data-chat-moderation-card {
                                    header {
                                        h3 { (&entry.room_name) }
                                        p class="u-muted" {
                                            "Message " (&entry.message_id.as_ref().to_string()[..8])
                                            " · User " (&entry.user_id.as_ref().to_string()[..8])
                                            " · " (&entry.created_at)
                                        }
                                    }
                                    p { (&entry.body) }
                                    p class="u-muted" { "Reason: " (&entry.reason) }
                                    form method="post" action=(Route::ChatModeration) {
                                        input type="hidden" name="message_id" value=(entry.message_id.as_ref());
                                        input type="hidden" name="reason" value=(&entry.reason);
                                        (partials::button::Row::builder()
                                            .items(vec![
                                                partials::button::Button::builder()
                                                    .label(Text::from("Approve"))
                                                    .variant(partials::button::Variant::Secondary)
                                                    .role(partials::button::Role::submit_with(
                                                        "decision",
                                                        app::chat::moderation::Decision::Approve.to_string(),
                                                    ))
                                                    .build(),
                                                partials::button::Button::builder()
                                                    .label(Text::from("Remove"))
                                                    .variant(partials::button::Variant::Primary)
                                                    .role(partials::button::Role::submit_with(
                                                        "decision",
                                                        app::chat::moderation::Decision::Remove.to_string(),
                                                    ))
                                                    .build(),
                                            ])
                                            .build())
                                    }
                                }
                            }
                        }
                    }
                }
            }
        };
        let content = page::Frame::builder().content(content).build().render();

        page::Layout::builder()
            .title("Chat moderation")
            .content(content)
            .maybe_with_user(self.user.clone())
            .build()
            .render()
    }
}
