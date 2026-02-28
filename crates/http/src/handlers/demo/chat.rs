use axum::{extract::Extension, http::StatusCode, response::IntoResponse};
use datastar::axum::ReadSignals;
use datastar::prelude::{ElementPatchMode, PatchElements};
use maud::Render;
use serde::Deserialize;

use crate::trace_log::{LogMessageKnown, LogTargetKnown};
use crate::types::{LogFieldKey, Text};
use crate::views::partials::chat;
use crate::{paths::Route, request, views};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatSignals {
    pub room_id: Text,
    pub body: Text,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DemoChatSignals {
    pub room_id: Text,
    pub bot_body: Text,
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
            .maybe_with_user(Some(user_nav))
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

    let decision = parse_moderation_decision(&form.decision.to_string())?;

    state
        .chat
        .moderate_message(
            app::chat::ModerateMessage::builder()
                .message_id(parse_message_id(&form.message_id.to_string())?)
                .reviewer_id(chat_user_id_from_user_id(user.id.to_domain()?))
                .decision(decision)
                .maybe_reason(parse_reason(form.reason)?)
                .build(),
        )
        .await?;

    moderation_page(Extension(state), auth_session).await
}

pub async fn post_chat_message(
    Extension(state): Extension<crate::State>,
    auth_session: crate::auth::Session,
    ReadSignals(signals): ReadSignals<ChatSignals>,
) -> Result<axum::response::Response, crate::error::Error> {
    let user = auth_session
        .user
        .as_ref()
        .ok_or(crate::error::Error::Internal)?;

    let body_text = signals.body.to_string();
    let message = state
        .chat
        .post_message(
            app::chat::PostMessage::builder()
                .room_id(parse_room_id(&signals.room_id.to_string())?)
                .user_id(chat_user_id_from_user_id(user.id.to_domain()?))
                .body(parse_message_body(&body_text)?)
                .build(),
        )
        .await?;

    let user_id = crate::types::UserIdText::new(user.id.to_string());
    record_incoming_chat_event(&state, ChatSender::You, &user_id, body_text.len());
    let message_html = render_chat_message_html(&message, &user.username.to_string());
    broadcast_message(&state, &message_html, ChatSender::You, user_id);

    Ok(chat_post_response())
}

pub async fn post_demo_chat_message(
    Extension(state): Extension<crate::State>,
    ReadSignals(signals): ReadSignals<DemoChatSignals>,
) -> Result<axum::response::Response, crate::error::Error> {
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
    let message = state
        .chat
        .post_message(
            app::chat::PostMessage::builder()
                .room_id(room_id)
                .user_id(chat_user_id_from_user_id(demo_user.id))
                .body(parse_message_body(&body_text)?)
                .build(),
        )
        .await?;

    let demo_user_id = crate::types::UserIdText::new(demo_user.id.as_uuid().to_string());
    record_incoming_chat_event(&state, ChatSender::Demo, &demo_user_id, body_text.len());
    let message_html = render_chat_message_html(&message, &demo_user.username.to_string());
    broadcast_message(&state, &message_html, ChatSender::Demo, demo_user_id);

    Ok(chat_post_response())
}

fn render_chat_message_html(message: &domain::chat::Message, author: &str) -> String {
    chat::Message::builder()
        .message_id(crate::types::Text::from(message.id.as_uuid().to_string()))
        .author(crate::types::Text::from(author.to_owned()))
        .timestamp(crate::types::Text::from(
            crate::chat_demo::format_message_time(message.created_at),
        ))
        .body(crate::types::Text::from(message.body.to_string()))
        .status(match message.status {
            domain::chat::MessageStatus::Visible => chat::message::Status::Visible,
            domain::chat::MessageStatus::Pending => chat::message::Status::Pending,
            domain::chat::MessageStatus::Removed => chat::message::Status::Removed,
        })
        .build()
        .render()
        .into_string()
}

fn record_incoming_chat_event(
    state: &crate::State,
    sender: ChatSender,
    user_id: &crate::types::UserIdText,
    payload_bytes: usize,
) {
    state.trace_log.record_sse_event(
        request::current_context()
            .and_then(|value| value.session_id)
            .as_ref(),
        crate::trace_log::TraceEntry::builder()
            .timestamp(crate::trace_log::now_timestamp_short())
            .level(crate::types::LogLevelText::new("INFO"))
            .target(crate::types::LogTargetText::from(LogTargetKnown::DemoChat))
            .message(crate::types::LogMessageText::from(
                LogMessageKnown::ChatMessageIncoming,
            ))
            .fields(vec![
                (
                    crate::types::LogFieldName::from(LogFieldKey::Direction),
                    crate::types::LogFieldValue::new("incoming"),
                ),
                (
                    crate::types::LogFieldName::from(LogFieldKey::Sender),
                    crate::types::LogFieldValue::new(sender.as_str()),
                ),
                (
                    crate::types::LogFieldName::from(LogFieldKey::Receiver),
                    crate::types::LogFieldValue::new("server"),
                ),
                (
                    crate::types::LogFieldName::from(LogFieldKey::UserId),
                    crate::types::LogFieldValue::new(user_id.to_string()),
                ),
                (
                    crate::types::LogFieldName::from(LogFieldKey::PayloadBytes),
                    crate::types::LogFieldValue::new(payload_bytes.to_string()),
                ),
            ])
            .build(),
    );
}

fn chat_post_response() -> axum::response::Response {
    match crate::request::current_kind() {
        crate::request::Kind::Datastar => StatusCode::ACCEPTED.into_response(),
        crate::request::Kind::Page => {
            let target =
                format!("{}#{}", Route::Home.as_str(), chat::DemoSection::ANCHOR_ID);
            axum::response::Redirect::to(target.as_str()).into_response()
        }
    }
}

fn broadcast_message(
    state: &crate::State,
    message_html: &str,
    sender: ChatSender,
    user_id: crate::types::UserIdText,
) {
    let event = PatchElements::new(message_html)
        .selector("[data-chat-messages]")
        .mode(ElementPatchMode::Prepend)
        .into_datastar_event();
    tracing::info!(
        target: LogTargetKnown::DemoSse.as_str(),
        message = LogMessageKnown::ChatMessageBroadcast.as_str(),
        selector = "[data-chat-messages]",
        mode = "prepend",
        payload_bytes = message_html.len() as u64
    );
    let _ = state.sse.broadcast(crate::sse::Event::from_event(event));

    let session_id = request::current_context().and_then(|value| value.session_id);
    state.trace_log.record_sse_event(
        session_id.as_ref(),
        crate::trace_log::TraceEntry::builder()
            .timestamp(crate::trace_log::now_timestamp_short())
            .level(crate::types::LogLevelText::new("INFO"))
            .target(crate::types::LogTargetText::from(LogTargetKnown::DemoSse))
            .message(crate::types::LogMessageText::from(
                LogMessageKnown::ChatMessageBroadcast,
            ))
            .fields(vec![
                (
                    crate::types::LogFieldName::from(LogFieldKey::Selector),
                    crate::types::LogFieldValue::new("[data-chat-messages]"),
                ),
                (
                    crate::types::LogFieldName::from(LogFieldKey::Mode),
                    crate::types::LogFieldValue::new("prepend"),
                ),
                (
                    crate::types::LogFieldName::from(LogFieldKey::PayloadBytes),
                    crate::types::LogFieldValue::new(message_html.len().to_string()),
                ),
                (
                    crate::types::LogFieldName::from(LogFieldKey::Direction),
                    crate::types::LogFieldValue::new("outgoing"),
                ),
                (
                    crate::types::LogFieldName::from(LogFieldKey::Sender),
                    crate::types::LogFieldValue::new(sender.as_str()),
                ),
                (
                    crate::types::LogFieldName::from(LogFieldKey::Receiver),
                    crate::types::LogFieldValue::new("clients"),
                ),
                (
                    crate::types::LogFieldName::from(LogFieldKey::UserId),
                    crate::types::LogFieldValue::new(user_id.to_string()),
                ),
            ])
            .build(),
    );
}

#[derive(Clone, Copy, Debug)]
enum ChatSender {
    You,
    Demo,
}

impl ChatSender {
    fn as_str(self) -> &'static str {
        match self {
            ChatSender::You => "you",
            ChatSender::Demo => "demo",
        }
    }
}

fn parse_room_id(value: &str) -> Result<domain::chat::RoomId, crate::error::Error> {
    let id = value.parse::<uuid::Uuid>().map_err(|error| {
        crate::error::Error::Chat(app::chat::Error::InvalidId(
            app::chat::InvalidIdText::new(error.to_string()),
        ))
    })?;
    Ok(domain::chat::RoomId::from_uuid(id))
}

fn parse_message_id(value: &str) -> Result<domain::chat::MessageId, crate::error::Error> {
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

fn parse_reason(
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

fn parse_moderation_decision(
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
