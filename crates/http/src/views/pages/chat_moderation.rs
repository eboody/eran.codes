use bon::Builder;
use maud::Render;

use crate::types::Text;
use crate::views::partials::{
    CtaButton, CtaButtonType, CtaItem, CtaRow, CtaTone, ModerationAction,
};
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
                header class="ui-chat-moderation-hero" data-chat-moderation-hero {
                    div {
                        h1 { "Chat moderation queue" }
                        p { "Review pending messages and apply moderation decisions." }
                    }
                }

                section class="ui-chat-moderation-flow" data-chat-moderation-flow {
                    @if self.entries.is_empty() {
                        p data-muted { "No pending messages." }
                    } @else {
                        div class="ui-chat-moderation-stack" data-chat-moderation-stack {
                            @for entry in &self.entries {
                                article class="ui-chat-moderation-card" data-chat-moderation-card {
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
                                                CtaItem::Button(
                                                    CtaButton::builder()
                                                        .label(Text::from("Approve"))
                                                        .button_type(CtaButtonType::Submit)
                                                        .name(Text::from("decision"))
                                                        .value(Text::from(
                                                            ModerationAction::Approve.to_string(),
                                                        ))
                                                        .tone(CtaTone::Secondary)
                                                        .build(),
                                                ),
                                                CtaItem::Button(
                                                    CtaButton::builder()
                                                        .label(Text::from("Remove"))
                                                        .button_type(CtaButtonType::Submit)
                                                        .name(Text::from("decision"))
                                                        .value(Text::from(
                                                            ModerationAction::Remove.to_string(),
                                                        ))
                                                        .tone(CtaTone::Primary)
                                                        .build(),
                                                ),
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
