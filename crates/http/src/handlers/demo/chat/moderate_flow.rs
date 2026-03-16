use statum::{machine, state, transition};

use super::ModerationForm;
use crate::types::Text;
use domain::chat;
#[derive(Clone, Debug)]
pub struct ParsedData {
    command: app::chat::ModerateMessage,
}

#[state]
pub enum ChatModerationState {
    Incoming,
    Parsed(ParsedData),
    Applied,
}

#[machine]
pub(super) struct ChatModerationFlow<ChatModerationState> {
    message_id_text: Text,
    decision_text: Text,
    reason_text: Option<Text>,
    reviewer_id: chat::UserId,
}

impl ChatModerationFlow<Incoming> {
    pub(super) fn from_form(
        form: ModerationForm,
        reviewer_id: crate::auth::UserId,
    ) -> crate::Result<Self> {
        let reviewer_id = reviewer_id.to_domain()?;
        let reviewer_id = domain::chat::UserId::from_uuid(*reviewer_id.as_uuid());

        Ok(ChatModerationFlow::<Incoming>::builder()
            .message_id_text(form.message_id)
            .decision_text(form.decision)
            .reason_text(form.reason)
            .reviewer_id(reviewer_id)
            .build())
    }

    pub(super) fn parse(self) -> Result<ChatModerationFlow<Parsed>, crate::error::Error> {
        let command = app::chat::ModerateMessage::builder()
            .message_id(self.message_id()?)
            .reviewer_id(self.reviewer_id)
            .decision(self.decision()?)
            .maybe_reason(self.reason()?)
            .build();
        Ok(self.mark_parsed(command))
    }

    fn message_id(&self) -> Result<domain::chat::message::Id, crate::error::Error> {
        let id = self
            .message_id_text
            .to_string()
            .parse::<uuid::Uuid>()
            .map_err(|error| {
                crate::error::Error::from(app::chat::Error::invalid_message_id(error))
            })?;

        Ok(domain::chat::message::Id::from_uuid(id))
    }

    fn decision(&self) -> Result<app::chat::ModerationDecision, crate::error::Error> {
        match crate::views::partials::ModerationAction::parse(
            &self.decision_text.to_string(),
        ) {
            Some(crate::views::partials::ModerationAction::Approve) => {
                Ok(app::chat::ModerationDecision::Approve)
            }
            Some(crate::views::partials::ModerationAction::Remove) => {
                Ok(app::chat::ModerationDecision::Remove)
            }
            None => Err(crate::error::Error::from(
                app::chat::Error::invalid_moderation_decision(
                    self.decision_text.to_string(),
                ),
            )),
        }
    }

    fn reason(&self) -> Result<Option<app::chat::ModerationReason>, crate::error::Error> {
        self.reason_text
            .clone()
            .map(|value| {
                app::chat::ModerationReason::try_new(value.to_string()).map_err(|error| {
                    crate::error::Error::from(app::chat::Error::invalid_moderation_reason(
                        error,
                    ))
                })
            })
            .transpose()
    }
}

#[transition]
impl ChatModerationFlow<Incoming> {
    fn mark_parsed(
        self,
        command: app::chat::ModerateMessage,
    ) -> ChatModerationFlow<Parsed> {
        self.transition_with(ParsedData { command })
    }
}

impl ChatModerationFlow<Parsed> {
    pub(super) async fn apply(
        self,
        state: &crate::State,
    ) -> Result<ChatModerationFlow<Applied>, crate::error::Error> {
        let command = self.state_data.command.clone();
        state.chat.moderate_message(command).await?;
        Ok(self.mark_applied())
    }
}

#[transition]
impl ChatModerationFlow<Parsed> {
    fn mark_applied(self) -> ChatModerationFlow<Applied> {
        self.transition()
    }
}

pub(super) type IncomingFlow = ChatModerationFlow<Incoming>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Text;

    #[test]
    fn parse_rejects_invalid_message_id() {
        let flow = IncomingFlow::from_form(
            ModerationForm {
                message_id: Text::from("not-a-uuid"),
                decision: Text::from("approve"),
                reason: None,
            },
            reviewer_id(),
        )
        .expect("incoming");

        let result = flow.parse();
        assert!(result.is_err());
    }

    #[test]
    fn parse_accepts_valid_form() {
        let flow = IncomingFlow::from_form(
            ModerationForm {
                message_id: Text::from(
                    domain::chat::message::Id::new_v4().as_uuid().to_string(),
                ),
                decision: Text::from("approve"),
                reason: Some(Text::from("looks good")),
            },
            reviewer_id(),
        )
        .expect("incoming");

        let result = flow.parse();
        assert!(result.is_ok());
    }

    fn reviewer_id() -> crate::auth::UserId {
        crate::auth::UserId::new(uuid::Uuid::new_v4().to_string())
    }
}
