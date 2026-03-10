use statum::{machine, state, transition};

use super::chat::ModerationForm;
use crate::types::Text;
use domain::chat as domain_chat;

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
    reviewer_id: domain_chat::UserId,
}

impl ChatModerationFlow<Incoming> {
    pub(super) fn from_form(
        form: ModerationForm,
        reviewer_id: domain_chat::UserId,
    ) -> Self {
        ChatModerationFlow::<Incoming>::builder()
            .message_id_text(form.message_id)
            .decision_text(form.decision)
            .maybe_reason_text(form.reason)
            .reviewer_id(reviewer_id)
            .build()
    }

    pub(super) fn parse(self) -> Result<ChatModerationFlow<Parsed>, crate::error::Error> {
        let decision =
            super::chat::parse_moderation_decision(&self.decision_text.to_string())?;
        let command = app::chat::ModerateMessage::builder()
            .message_id(super::chat::parse_message_id(
                &self.message_id_text.to_string(),
            )?)
            .reviewer_id(self.reviewer_id)
            .decision(decision)
            .maybe_reason(super::chat::parse_reason(self.reason_text.clone())?)
            .build();
        Ok(self.mark_parsed(command))
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

    #[test]
    fn parse_rejects_invalid_message_id() {
        let flow = IncomingFlow::from_form(
            ModerationForm {
                message_id: Text::from("not-a-uuid"),
                decision: Text::from("approve"),
                reason: None,
            },
            domain_chat::UserId::new_v4(),
        );

        let result = flow.parse();
        assert!(result.is_err());
    }

    #[test]
    fn parse_accepts_valid_form() {
        let flow = IncomingFlow::from_form(
            ModerationForm {
                message_id: Text::from(
                    domain_chat::MessageId::new_v4().as_uuid().to_string(),
                ),
                decision: Text::from("approve"),
                reason: Some(Text::from("looks good")),
            },
            domain_chat::UserId::new_v4(),
        );

        let result = flow.parse();
        assert!(result.is_ok());
    }
}
