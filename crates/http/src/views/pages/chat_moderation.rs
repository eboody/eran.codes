use bon::Builder;
use maud::Render;
use maud_extensions::css;

use crate::views::partials::{CtaRow, ModerationAction};
use crate::views::page::{Layout, UserNav};
use crate::paths::Route;

#[derive(Builder)]
pub struct ChatModeration {
    pub entries: Vec<app::chat::ModerationItem>,
    #[builder(setters(name = with_user))]
    pub user: Option<UserNav>,
}

impl Render for ChatModeration {
    fn render(&self) -> maud::Markup {
        let content = maud::html! {
            main class="container" {
                ({
                    css! {
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
                          border: 1px solid var(--ui-border-soft);
                          border-radius: var(--ui-radius-md);
                          background: var(--ui-surface-soft);
                          padding: 1rem;
                        }
                        me [data-chat-moderation-card] > header h3 {
                          margin-bottom: 0.2rem;
                        }
                        me [data-chat-moderation-card] [data-muted] {
                          color: color-mix(in srgb, var(--ui-text-muted) 94%, var(--ui-text) 6%);
                        }
                        me [data-chat-moderation-flow] [data-cta-row] {
                          margin-top: 0.75rem;
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
                        }
                    }
                })
                header data-chat-moderation-hero {
                    div {
                        h1 { "Chat moderation queue" }
                        p { "Review pending messages and apply moderation decisions." }
                    }
                }

                section data-chat-moderation-flow {
                    @if self.entries.is_empty() {
                        p data-muted { "No pending messages." }
                    } @else {
                        div data-chat-moderation-stack {
                            @for entry in &self.entries {
                                article data-chat-moderation-card {
                                    header {
                                        h3 { (&entry.room_name) }
                                        p data-muted {
                                            "Message " (&entry.message_id.as_uuid().to_string()[..8])
                                            " · User " (&entry.user_id.as_uuid().to_string()[..8])
                                            " · " (&entry.created_at)
                                        }
                                    }
                                    p { (&entry.body) }
                                    p data-muted { "Reason: " (&entry.reason) }
                                    form method="post" action=(Route::ChatModeration) {
                                        input type="hidden" name="message_id" value=(entry.message_id.as_uuid());
                                        input type="hidden" name="reason" value=(&entry.reason);
                                        (CtaRow::builder()
                                            .items(vec![
                                                maud::html! {
                                                    button type="submit" name="decision" value=(ModerationAction::Approve) class="button secondary" { "Approve" }
                                                },
                                                maud::html! {
                                                    button type="submit" name="decision" value=(ModerationAction::Remove) class="button" { "Remove" }
                                                },
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
