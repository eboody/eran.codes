use axum::{
    extract::Extension,
    http::StatusCode,
    response::IntoResponse,
};
use maud::Render;
use datastar::axum::ReadSignals;
use datastar::prelude::{ElementPatchMode, PatchElements};
use serde::Deserialize;

use crate::{paths::Route, request, views};
use crate::trace_log::{LogMessageKnown, LogTargetKnown};
use crate::types::{LogFieldKey, Text};

const DEMO_USER_EMAIL: &str = "demo.bot@example.com";
const DEMO_USER_NAME: &str = "Demo Bot";

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

pub async fn chat_page(
    Extension(_state): Extension<crate::State>,
    auth_session: crate::auth::Session,
) -> crate::Result<axum::response::Response> {
    let _ = auth_session
        .user
        .as_ref()
        .ok_or(crate::error::Error::Internal)?;
    let target = format!(
        "{}#{}",
        Route::Home.as_str(),
        crate::views::partials::ChatDemoSection::ANCHOR_ID
    );
    Ok(axum::response::Redirect::to(target.as_str()).into_response())
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

    let decision = match crate::views::partials::ModerationAction::parse(&form.decision.to_string()) {
        Some(crate::views::partials::ModerationAction::Approve) => {
            app::chat::ModerationDecision::Approve
        }
        Some(crate::views::partials::ModerationAction::Remove) => {
            app::chat::ModerationDecision::Remove
        }
        None => return Err(crate::error::Error::Internal),
    };

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

    let message = state
        .chat
        .post_message(
            app::chat::PostMessage::builder()
                .room_id(parse_room_id(&signals.room_id.to_string())?)
                .user_id(chat_user_id_from_user_id(user.id.to_domain()?))
                .body(parse_message_body(&signals.body.to_string())?)
                .build(),
        )
        .await?;

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
                    crate::types::LogFieldValue::new("you"),
                ),
                (
                    crate::types::LogFieldName::from(LogFieldKey::Receiver),
                    crate::types::LogFieldValue::new("server"),
                ),
                (
                    crate::types::LogFieldName::from(LogFieldKey::UserId),
                    crate::types::LogFieldValue::new(user.id.to_string()),
                ),
                (
                    crate::types::LogFieldName::from(LogFieldKey::Body),
                    crate::types::LogFieldValue::new(signals.body.to_string()),
                ),
            ])
            .build(),
    );

    let message_html = views::partials::ChatMessage::builder()
        .message_id(crate::types::Text::from(message.id.as_uuid().to_string()))
        .author(crate::types::Text::from(user.username.to_string()))
        .timestamp(crate::types::Text::from(crate::chat_demo::format_message_time(message.created_at)))
        .body(crate::types::Text::from(message.body.to_string()))
        .status(crate::types::Text::from(format!("{:?}", message.status)))
        .build()
        .render()
        .into_string();
    broadcast_message(
        &state,
        &message_html,
        Text::from(message.body.to_string()),
        ChatSender::You,
        crate::types::UserIdText::new(user.id.to_string()),
    );

    let response = match crate::request::current_kind() {
        crate::request::Kind::Datastar => (
            StatusCode::OK,
            axum::response::Html(message_html),
        )
            .into_response(),
        crate::request::Kind::Page => {
            let target = format!(
                "{}#{}",
                Route::Home.as_str(),
                crate::views::partials::ChatDemoSection::ANCHOR_ID
            );
            axum::response::Redirect::to(target.as_str()).into_response()
        }
    };

    Ok(response)
}

pub async fn post_demo_chat_message(
    Extension(state): Extension<crate::State>,
    auth_session: crate::auth::Session,
    ReadSignals(signals): ReadSignals<DemoChatSignals>,
) -> Result<axum::response::Response, crate::error::Error> {
    let _ = auth_session
        .user
        .as_ref()
        .ok_or(crate::error::Error::Internal)?;

    let demo_user = ensure_demo_user(&state).await?;
    let _ = state
        .chat
        .join_room(
            app::chat::JoinRoom::builder()
                .room_id(parse_room_id(&signals.room_id.to_string())?)
                .user_id(chat_user_id_from_user_id(demo_user.id))
                .build(),
        )
        .await;

    let message = state
        .chat
        .post_message(
            app::chat::PostMessage::builder()
                .room_id(parse_room_id(&signals.room_id.to_string())?)
                .user_id(chat_user_id_from_user_id(demo_user.id))
                .body(parse_message_body(&signals.bot_body.to_string())?)
                .build(),
        )
        .await?;

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
                    crate::types::LogFieldValue::new("demo"),
                ),
                (
                    crate::types::LogFieldName::from(LogFieldKey::Receiver),
                    crate::types::LogFieldValue::new("server"),
                ),
                (
                    crate::types::LogFieldName::from(LogFieldKey::UserId),
                    crate::types::LogFieldValue::new(
                        demo_user.id.as_uuid().to_string(),
                    ),
                ),
                (
                    crate::types::LogFieldName::from(LogFieldKey::Body),
                    crate::types::LogFieldValue::new(signals.bot_body.to_string()),
                ),
            ])
            .build(),
    );

    let message_html = views::partials::ChatMessage::builder()
        .message_id(crate::types::Text::from(message.id.as_uuid().to_string()))
        .author(crate::types::Text::from(demo_user.username.to_string()))
        .timestamp(crate::types::Text::from(crate::chat_demo::format_message_time(message.created_at)))
        .body(crate::types::Text::from(message.body.to_string()))
        .status(crate::types::Text::from(format!("{:?}", message.status)))
        .build()
        .render()
        .into_string();
    broadcast_message(
        &state,
        &message_html,
        Text::from(message.body.to_string()),
        ChatSender::Demo,
        crate::types::UserIdText::new(demo_user.id.as_uuid().to_string()),
    );

    let response = match crate::request::current_kind() {
        crate::request::Kind::Datastar => (
            StatusCode::OK,
            axum::response::Html(message_html),
        )
            .into_response(),
        crate::request::Kind::Page => {
            let target = format!(
                "{}#{}",
                Route::Home.as_str(),
                crate::views::partials::ChatDemoSection::ANCHOR_ID
            );
            axum::response::Redirect::to(target.as_str()).into_response()
        }
    };

    Ok(response)
}

async fn ensure_demo_user(
    state: &crate::State,
) -> Result<domain::user::User, crate::error::Error> {
    let demo_email = domain::user::Email::try_new(DEMO_USER_EMAIL)
        .map_err(|_| crate::error::Error::Internal)?;
    let demo_username = domain::user::Username::try_new(DEMO_USER_NAME)
        .map_err(|_| crate::error::Error::Internal)?;
    if let Some(user) = state
        .user
        .find_by_email(demo_email.clone())
        .await?
    {
        return Ok(user);
    }

    let password = secrecy::SecretString::new(
        uuid::Uuid::new_v4().to_string().into(),
    );
    match state
        .user
        .register_user(
            app::user::RegisterUser::builder()
                .username(demo_username)
                .email(demo_email.clone())
                .password(password)
                .build(),
        )
        .await
    {
        Ok(_) | Err(app::user::Error::EmailTaken) => {}
        Err(error) => return Err(error.into()),
    }

    state
        .user
        .find_by_email(demo_email)
        .await?
        .ok_or(crate::error::Error::Internal)
}

fn broadcast_message(
    state: &crate::State,
    message_html: &str,
    body: Text,
    sender: ChatSender,
    user_id: crate::types::UserIdText,
) {
    let event = PatchElements::new(message_html)
        .selector(".chat-messages")
        .mode(ElementPatchMode::Append)
        .into_datastar_event();
    tracing::info!(
        target: LogTargetKnown::DemoSse.as_str(),
        message = LogMessageKnown::ChatMessageBroadcast.as_str(),
        selector = ".chat-messages",
        mode = "append",
        payload_bytes = message_html.len() as u64
    );
    let _ = state
        .sse
        .broadcast(crate::sse::Event::from_event(event));

    let session_id = request::current_context()
        .and_then(|value| value.session_id);
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
                    crate::types::LogFieldValue::new(".chat-messages"),
                ),
                (
                    crate::types::LogFieldName::from(LogFieldKey::Mode),
                    crate::types::LogFieldValue::new("append"),
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
                (
                    crate::types::LogFieldName::from(LogFieldKey::Body),
                    crate::types::LogFieldValue::new(body.to_string()),
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

fn parse_room_id(
    value: &str,
) -> Result<domain::chat::RoomId, crate::error::Error> {
    let id = value
        .parse::<uuid::Uuid>()
        .map_err(|_| crate::error::Error::Internal)?;
    Ok(domain::chat::RoomId::from_uuid(id))
}

fn parse_message_id(
    value: &str,
) -> Result<domain::chat::MessageId, crate::error::Error> {
    let id = value
        .parse::<uuid::Uuid>()
        .map_err(|_| crate::error::Error::Internal)?;
    Ok(domain::chat::MessageId::from_uuid(id))
}

fn parse_message_body(
    value: &str,
) -> Result<domain::chat::MessageBody, crate::error::Error> {
    domain::chat::MessageBody::try_new(value)
        .map_err(|_| crate::error::Error::Internal)
}

fn parse_reason(
    value: Option<Text>,
) -> Result<Option<app::chat::ModerationReason>, crate::error::Error> {
    value
        .map(|value| {
            app::chat::ModerationReason::try_new(value.to_string())
                .map_err(|_| crate::error::Error::Internal)
        })
        .transpose()
}

fn chat_user_id_from_user_id(
    value: domain::user::Id,
) -> domain::chat::UserId {
    domain::chat::UserId::from_uuid(*value.as_uuid())
}
