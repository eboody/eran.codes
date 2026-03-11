use bon::Builder;
use maud::Render;

use crate::paths::Route;
use crate::types::Text;
use crate::views::page::{Layout, UserNav};
use crate::views::partials::{button, ModerationAction};

crate::views::scoped::inline_css!(
    r#"
me {
  display: grid;
  gap: var(--space-section);
  margin-top: clamp(1.1rem, 0.9rem + 0.9vw, 1.8rem);
  padding-bottom: calc(var(--space-section) + var(--size-6));
}

me > :where(header, section) {
  margin-top: 0;
  scroll-margin-top: var(--nav-scroll-offset);
}

me [data-chat-moderation-hero] {
  display: grid;
  gap: 0.85rem;
  margin-top: 1.2rem;
  margin-bottom: 1.35rem;
  padding: 1.2rem;
  border-radius: 18px;
  border: 1px solid var(--portfolio-surface-border);
  background:
    linear-gradient(
      135deg,
      color-mix(in srgb, var(--portfolio-accent-a) 18%, transparent),
      transparent 58%
    ),
    var(--portfolio-surface);
  box-shadow: var(--portfolio-shadow);
}

me [data-chat-moderation-hero] h1 {
  margin: 0;
  font-size: clamp(1.75rem, 1.32rem + 1.9vw, 2.4rem);
  line-height: 1.08;
}

me [data-chat-moderation-flow] {
  overflow: visible;
  border: 1px dashed var(--ui-border-muted);
  border-radius: 20px;
  padding: 1.15rem;
  background: var(--ui-surface-card);
}

me [data-chat-moderation-stack] {
  display: grid;
  gap: 0.85rem;
}

me [data-chat-moderation-card] {
  overflow: visible;
  border: 1px solid var(--ui-border-soft);
  border-radius: var(--ui-radius-md);
  background: var(--ui-surface-soft);
  padding: 1rem;
}

me [data-chat-moderation-card] > header h3 {
  margin-bottom: 0.2rem;
}

me [data-chat-moderation-card] .u-muted {
  color: color-mix(in srgb, var(--ui-text-muted) 94%, var(--ui-text) 6%);
}

me [data-chat-moderation-flow] .ui-button-row {
  margin-top: calc(0.75rem - var(--interactive-bleed));
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
  me [data-chat-moderation-hero] {
    margin-top: 0.9rem;
    margin-bottom: 1.05rem;
    padding: 0.95rem;
    border-radius: 16px;
  }

  me [data-chat-moderation-flow] {
    padding: 0.9rem;
    border-radius: 16px;
  }

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
    pub entries: Vec<app::chat::ModerationItem>,
    #[builder(setters(name = with_user))]
    pub user: Option<UserNav>,
}

impl Render for ChatModeration {
    fn render(&self) -> maud::Markup {
        let content = maud::html! {
            main class="u-container" data-chat-page data-chat-moderation-page {
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
                                            "Message " (&entry.message_id.as_uuid().to_string()[..8])
                                            " · User " (&entry.user_id.as_uuid().to_string()[..8])
                                            " · " (&entry.created_at)
                                        }
                                    }
                                    p { (&entry.body) }
                                    p class="u-muted" { "Reason: " (&entry.reason) }
                                    form method="post" action=(Route::ChatModeration) {
                                        input type="hidden" name="message_id" value=(entry.message_id.as_uuid());
                                        input type="hidden" name="reason" value=(&entry.reason);
                                        (button::Row::builder()
                                            .items(vec![
                                                button::Button::builder()
                                                    .label(Text::from("Approve"))
                                                    .variant(button::Variant::Secondary)
                                                    .role(button::Role::submit_with(
                                                        "decision",
                                                        ModerationAction::Approve.to_string(),
                                                    ))
                                                    .build(),
                                                button::Button::builder()
                                                    .label(Text::from("Remove"))
                                                    .variant(button::Variant::Primary)
                                                    .role(button::Role::submit_with(
                                                        "decision",
                                                        ModerationAction::Remove.to_string(),
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

        Layout::builder()
            .title("Chat moderation")
            .content(content)
            .maybe_with_user(self.user.clone())
            .build()
            .render()
    }
}
