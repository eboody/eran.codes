use axum::extract::Extension;
use datastar::axum::ReadSignals;
use serde::Deserialize;

use super::chat_post_flow::{ChatSender, IncomingFlow as ChatPostIncomingFlow};
use crate::types::{SseTabId, Text, UserIdText};
use crate::{request, views};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatSignals {
    pub room_id: Text,
    pub body: Text,
    pub sse_tab_id: Option<SseTabId>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DemoChatSignals {
    pub room_id: Text,
    pub bot_body: Text,
    pub sse_tab_id: Option<SseTabId>,
}

#[derive(Deserialize)]
pub struct ModerationForm {
    pub message_id: Text,
    pub decision: Text,
    pub reason: Option<Text>,
}

pub async fn moderation_page(
    Extension(state): Extension<crate::State>,
    auth_session: crate::auth::Session,
) -> crate::Result<axum::response::Html<String>> {
    let user = auth_session
        .user
        .as_ref()
        .ok_or(crate::error::Error::Internal)?;

    let entries = state.chat.list_moderation_queue(50).await?;
    let user_nav = crate::views::page::UserNav::builder()
        .username(Text::from(user.username.to_string()))
        .email(Text::from(user.email.to_string()))
        .build();

    Ok(views::render(
        views::pages::ChatModeration::builder()
            .entries(entries)
            .with_user(user_nav)
            .build(),
    ))
}

pub async fn moderate_message(
    Extension(state): Extension<crate::State>,
    auth_session: crate::auth::Session,
    axum::extract::Form(form): axum::extract::Form<ModerationForm>,
) -> crate::Result<axum::response::Html<String>> {
    let user = auth_session
        .user
        .as_ref()
        .ok_or(crate::error::Error::Internal)?;

    let reviewer_id = chat_user_id_from_user_id(user.id.to_domain()?);
    let incoming = super::chat_moderate_flow::IncomingFlow::from_form(form, reviewer_id);
    let parsed = incoming.parse()?;
    let _applied = parsed.apply(&state).await?;

    moderation_page(Extension(state), auth_session).await
}

pub async fn post_chat_message(
    Extension(state): Extension<crate::State>,
    auth_session: crate::auth::Session,
    ReadSignals(signals): ReadSignals<ChatSignals>,
) -> Result<axum::response::Response, crate::error::Error> {
    if let Some(tab_id) = signals.sse_tab_id.clone() {
        request::set_sse_tab_id(tab_id);
    }
    let user = auth_session
        .user
        .as_ref()
        .ok_or(crate::error::Error::Internal)?;

    let room_id = parse_room_id(&signals.room_id.to_string())?;
    let body_text = signals.body.to_string();
    let incoming = ChatPostIncomingFlow::new(
        room_id,
        chat_user_id_from_user_id(user.id.to_domain()?),
        parse_message_body(&body_text)?,
        body_text,
        ChatSender::You,
        user.username.to_string(),
        UserIdText::new(user.id.to_string()),
        current_request_id_text(),
    );
    execute_chat_post(&state, incoming).await
}

pub async fn post_demo_chat_message(
    Extension(state): Extension<crate::State>,
    ReadSignals(signals): ReadSignals<DemoChatSignals>,
) -> Result<axum::response::Response, crate::error::Error> {
    if let Some(tab_id) = signals.sse_tab_id.clone() {
        request::set_sse_tab_id(tab_id);
    }
    let demo_user = crate::chat_demo::ensure_demo_user(&state).await?;
    let room_id = parse_room_id(&signals.room_id.to_string())?;
    state
        .chat
        .join_room(
            app::chat::JoinRoom::builder()
                .room_id(room_id)
                .user_id(chat_user_id_from_user_id(demo_user.id))
                .build(),
        )
        .await?;

    let body_text = signals.bot_body.to_string();
    let incoming = ChatPostIncomingFlow::new(
        room_id,
        chat_user_id_from_user_id(demo_user.id),
        parse_message_body(&body_text)?,
        body_text,
        ChatSender::Demo,
        demo_user.username.to_string(),
        UserIdText::new(demo_user.id.as_uuid().to_string()),
        current_request_id_text(),
    );
    execute_chat_post(&state, incoming).await
}

async fn execute_chat_post(
    state: &crate::State,
    incoming: ChatPostIncomingFlow,
) -> Result<axum::response::Response, crate::error::Error> {
    let posted = incoming.mark_command_built().post_message(state).await?;
    let rendered = posted.record_incoming(state).render_message_html();
    let broadcasted = rendered.broadcast(state);
    Ok(broadcasted.into_response())
}

fn current_request_id_text() -> Text {
    request::current_context()
        .and_then(|context| context.request_id)
        .map(|request_id| Text::from(request_id.to_string()))
        .unwrap_or_else(|| Text::from(format!("fallback-{}", uuid::Uuid::new_v4())))
}

fn parse_room_id(value: &str) -> Result<domain::chat::RoomId, crate::error::Error> {
    let id = value.parse::<uuid::Uuid>().map_err(|error| {
        crate::error::Error::Chat(app::chat::Error::InvalidId(
            app::chat::InvalidIdText::new(error.to_string()),
        ))
    })?;
    Ok(domain::chat::RoomId::from_uuid(id))
}

pub(super) fn parse_message_id(
    value: &str,
) -> Result<domain::chat::MessageId, crate::error::Error> {
    let id = value.parse::<uuid::Uuid>().map_err(|error| {
        crate::error::Error::Chat(app::chat::Error::InvalidId(
            app::chat::InvalidIdText::new(error.to_string()),
        ))
    })?;
    Ok(domain::chat::MessageId::from_uuid(id))
}

fn parse_message_body(
    value: &str,
) -> Result<domain::chat::MessageBody, crate::error::Error> {
    domain::chat::MessageBody::try_new(value)
        .map_err(domain::chat::Error::from)
        .map_err(app::chat::Error::from)
        .map_err(crate::error::Error::from)
}

pub(super) fn parse_reason(
    value: Option<Text>,
) -> Result<Option<app::chat::ModerationReason>, crate::error::Error> {
    value
        .map(|value| {
            app::chat::ModerationReason::try_new(value.to_string()).map_err(|error| {
                crate::error::Error::Chat(app::chat::Error::InvalidInput(
                    app::chat::InvalidInputText::new(error.to_string()),
                ))
            })
        })
        .transpose()
}

fn chat_user_id_from_user_id(value: domain::user::Id) -> domain::chat::UserId {
    domain::chat::UserId::from_uuid(*value.as_uuid())
}

pub(super) fn parse_moderation_decision(
    value: &str,
) -> Result<app::chat::ModerationDecision, crate::error::Error> {
    match crate::views::partials::ModerationAction::parse(value) {
        Some(crate::views::partials::ModerationAction::Approve) => {
            Ok(app::chat::ModerationDecision::Approve)
        }
        Some(crate::views::partials::ModerationAction::Remove) => {
            Ok(app::chat::ModerationDecision::Remove)
        }
        None => Err(crate::error::Error::Chat(app::chat::Error::InvalidInput(
            app::chat::InvalidInputText::new("invalid moderation decision"),
        ))),
    }
}
