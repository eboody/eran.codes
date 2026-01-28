use bon::Builder;
use maud::Render;

use crate::views::partials::ModerationAction;
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
                header class="hero" {
                    div {
                        h1 { "Chat moderation queue" }
                        p { "Review pending messages and apply moderation decisions." }
                    }
                }

                section class="flow-card" {
                    @if self.entries.is_empty() {
                        p class="muted" { "No pending messages." }
                    } @else {
                        div class="stack" {
                            @for entry in &self.entries {
                                article class="card" {
                                    header {
                                        h3 { (&entry.room_name) }
                                        p class="muted" {
                                            "Message " (&entry.message_id.as_uuid().to_string()[..8])
                                            " · User " (&entry.user_id.as_uuid().to_string()[..8])
                                            " · " (&entry.created_at)
                                        }
                                    }
                                    p { (&entry.body) }
                                    p class="muted" { "Reason: " (&entry.reason) }
                                    form method="post" action=(Route::ChatModeration.as_str()) class="cta-row" {
                                        input type="hidden" name="message_id" value=(entry.message_id.as_uuid().to_string());
                                        input type="hidden" name="reason" value=(entry.reason.clone());
                                        button type="submit" name="decision" value=(ModerationAction::Approve.as_str()) class="button secondary" { "Approve" }
                                        button type="submit" name="decision" value=(ModerationAction::Remove.as_str()) class="button" { "Remove" }
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
