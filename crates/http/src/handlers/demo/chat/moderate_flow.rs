use statum::{machine, state, transition};

use super::ModerationForm;

#[state]
pub enum ChatModerationState {
    Incoming,
    Applied,
}

#[machine]
pub(super) struct ChatModerationFlow<ChatModerationState> {
    command: app::chat::ModerateMessage,
}

impl ChatModerationFlow<Incoming> {
    pub(super) fn from_form(
        form: ModerationForm,
        reviewer_id: crate::auth::UserId,
    ) -> crate::Result<Self> {
        let input = super::moderation_input::ModerationInput::parse(form, reviewer_id)?;

        Ok(ChatModerationFlow::<Incoming>::builder()
            .command(input.command)
            .build())
    }
}

impl ChatModerationFlow<Incoming> {
    pub(super) async fn apply(
        self,
        state: &crate::State,
    ) -> Result<ChatModerationFlow<Applied>, crate::Error> {
        state.chat.moderate_message(self.command.clone()).await?;
        Ok(self.mark_applied())
    }
}

#[transition]
impl ChatModerationFlow<Incoming> {
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
        let result = IncomingFlow::from_form(
            ModerationForm {
                message_id: Text::from("not-a-uuid"),
                decision: Text::from("approve"),
                reason: None,
            },
            reviewer_id(),
        );
        assert!(result.is_err());
    }

    #[test]
    fn from_form_accepts_valid_form() {
        let result = IncomingFlow::from_form(
            ModerationForm {
                message_id: Text::from(
                    domain::chat::message::Id::new_v4().as_ref().to_string(),
                ),
                decision: Text::from("approve"),
                reason: Some(Text::from("looks good")),
            },
            reviewer_id(),
        );
        assert!(result.is_ok());
    }

    fn reviewer_id() -> crate::auth::UserId {
        crate::auth::UserId::new(uuid::Uuid::new_v4().to_string())
    }
}
