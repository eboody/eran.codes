use std::str::FromStr;

use domain::chat;

use super::ModerationForm;

#[derive(Clone, Debug)]
pub(super) struct ModerationInput {
    pub command: app::chat::ModerateMessage,
}

impl ModerationInput {
    pub(super) fn parse(
        form: ModerationForm,
        reviewer_id: crate::auth::UserId,
    ) -> crate::Result<Self> {
        let reviewer_id = domain::user::Id::try_from(&reviewer_id)?;
        let reviewer_id = chat::UserId::from(reviewer_id);
        let command = app::chat::ModerateMessage::builder()
            .message_id(parse_message_id(&form.message_id)?)
            .reviewer_id(reviewer_id)
            .decision(parse_decision(&form.decision)?)
            .maybe_reason(parse_reason(form.reason)?)
            .build();

        Ok(Self { command })
    }
}

fn parse_message_id(
    value: &crate::types::Text,
) -> crate::Result<domain::chat::message::Id> {
    let id = value.to_string().parse::<uuid::Uuid>().map_err(|error| {
        crate::Error::from(app::chat::failure::Error::invalid_message_id(error))
    })?;

    Ok(domain::chat::message::Id::from(id))
}

fn parse_decision(
    value: &crate::types::Text,
) -> crate::Result<app::chat::moderation::Decision> {
    app::chat::moderation::Decision::from_str(&value.to_string()).map_err(|_| {
        crate::Error::from(app::chat::failure::Error::invalid_moderation_decision(
            value.to_string(),
        ))
    })
}

fn parse_reason(
    value: Option<crate::types::Text>,
) -> crate::Result<Option<app::chat::moderation::Reason>> {
    value
        .map(|value| {
            app::chat::moderation::Reason::try_new(value.to_string()).map_err(|error| {
                crate::Error::from(app::chat::failure::Error::invalid_moderation_reason(
                    error,
                ))
            })
        })
        .transpose()
}
