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

const DEMO_USER_EMAIL: &str = "demo.bot@example.com";
const DEMO_USER_NAME: &str = "Demo Bot";

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatSignals {
    pub room_id: String,
    pub body: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DemoChatSignals {
    pub room_id: String,
    pub bot_body: String,
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
    pub message_id: String,
    pub decision: String,
    pub reason: Option<String>,
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
        .username(user.username.clone())
        .email(user.email.clone())
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

    let decision = match crate::views::partials::ModerationAction::parse(&form.decision) {
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
                .message_id(form.message_id)
                .reviewer_id(user.id.clone())
                .decision(decision)
                .maybe_reason(form.reason)
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
                .room_id(signals.room_id.clone())
                .user_id(user.id.clone())
                .body(signals.body.clone())
                .build(),
        )
        .await?;

    state.trace_log.record_sse_event(
        request::current_context()
            .and_then(|value| value.session_id)
            .as_deref(),
        crate::trace_log::TraceEntry::builder()
            .timestamp(jiff::Timestamp::now().to_string())
            .level("INFO".to_string())
            .target("demo.chat".to_string())
            .message("chat.message.incoming".to_string())
            .fields(vec![
                ("direction".to_string(), "incoming".to_string()),
                ("sender".to_string(), "you".to_string()),
                ("receiver".to_string(), "server".to_string()),
                ("user_id".to_string(), user.id.clone()),
                ("body".to_string(), signals.body.clone()),
            ])
            .build(),
    );

    let message_html = views::partials::ChatMessage::builder()
        .message_id(message.id.as_uuid().to_string())
        .author(user.username.clone())
        .body(message.body.to_string())
        .status(format!("{:?}", message.status))
        .build()
        .render()
        .into_string();
    broadcast_message(
        &state,
        &message_html,
        message.body.to_string(),
        "you",
        user.id.clone(),
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
                .room_id(signals.room_id.clone())
                .user_id(demo_user.id.as_uuid().to_string())
                .build(),
        )
        .await;

    let message = state
        .chat
        .post_message(
            app::chat::PostMessage::builder()
                .room_id(signals.room_id.clone())
                .user_id(demo_user.id.as_uuid().to_string())
                .body(signals.bot_body.clone())
                .build(),
        )
        .await?;

    state.trace_log.record_sse_event(
        request::current_context()
            .and_then(|value| value.session_id)
            .as_deref(),
        crate::trace_log::TraceEntry::builder()
            .timestamp(jiff::Timestamp::now().to_string())
            .level("INFO".to_string())
            .target("demo.chat".to_string())
            .message("chat.message.incoming".to_string())
            .fields(vec![
                ("direction".to_string(), "incoming".to_string()),
                ("sender".to_string(), "demo".to_string()),
                ("receiver".to_string(), "server".to_string()),
                (
                    "user_id".to_string(),
                    demo_user.id.as_uuid().to_string(),
                ),
                ("body".to_string(), signals.bot_body.clone()),
            ])
            .build(),
    );

    let message_html = views::partials::ChatMessage::builder()
        .message_id(message.id.as_uuid().to_string())
        .author(demo_user.username.to_string())
        .body(message.body.to_string())
        .status(format!("{:?}", message.status))
        .build()
        .render()
        .into_string();
    broadcast_message(
        &state,
        &message_html,
        message.body.to_string(),
        "demo",
        demo_user.id.as_uuid().to_string(),
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
    if let Some(user) = state
        .user
        .find_by_email(DEMO_USER_EMAIL.to_string())
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
                .username(DEMO_USER_NAME.to_string())
                .email(DEMO_USER_EMAIL.to_string())
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
        .find_by_email(DEMO_USER_EMAIL.to_string())
        .await?
        .ok_or(crate::error::Error::Internal)
}

fn broadcast_message(
    state: &crate::State,
    message_html: &str,
    body: String,
    sender: &str,
    user_id: String,
) {
    let event = PatchElements::new(message_html)
        .selector(".chat-messages")
        .mode(ElementPatchMode::Append)
        .into_datastar_event();
    tracing::info!(
        target: "demo.sse",
        message = "chat message broadcast",
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
        session_id.as_deref(),
        crate::trace_log::TraceEntry::builder()
            .timestamp(jiff::Timestamp::now().to_string())
            .level("INFO".to_string())
            .target("demo.sse".to_string())
            .message("chat message broadcast".to_string())
            .fields(vec![
                ("selector".to_string(), ".chat-messages".to_string()),
                ("mode".to_string(), "append".to_string()),
                ("payload_bytes".to_string(), message_html.len().to_string()),
                ("direction".to_string(), "outgoing".to_string()),
                ("sender".to_string(), sender.to_string()),
                ("receiver".to_string(), "clients".to_string()),
                ("user_id".to_string(), user_id),
                ("body".to_string(), body),
            ])
            .build(),
    );
}
