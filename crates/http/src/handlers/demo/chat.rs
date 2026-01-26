use axum::{
    extract::Extension,
    http::StatusCode,
    response::IntoResponse,
};
use maud::Render;
use datastar::axum::ReadSignals;
use datastar::prelude::{ElementPatchMode, PatchElements};
use serde::Deserialize;

use crate::views;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatSignals {
    pub room_id: String,
    pub body: String,
}

pub async fn chat_page(
    Extension(state): Extension<crate::State>,
    auth_session: crate::auth::Session,
) -> crate::Result<axum::response::Html<String>> {
    let user = auth_session
        .user
        .as_ref()
        .ok_or(crate::error::Error::Internal)?;
    let room = ensure_room(&state, &user.id).await?;
    let messages = state
        .chat
        .list_messages(
            app::chat::ListMessages::builder()
                .room_id(room.id.as_uuid().to_string())
                .user_id(user.id.clone())
                .build(),
        )
        .await?;

    let message_views = to_message_views(&messages);
    let user_nav = crate::views::page::UserNav::builder()
        .username(user.username.clone())
        .email(user.email.clone())
        .build();

    Ok(views::render(
        views::pages::Chat::builder()
            .room_id(room.id.as_uuid().to_string())
            .room_name(room.name.to_string())
            .messages(message_views)
            .maybe_with_user(Some(user_nav))
            .build(),
    ))
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

    let decision = match form.decision.as_str() {
        "approve" => app::chat::ModerationDecision::Approve,
        "remove" => app::chat::ModerationDecision::Remove,
        _ => return Err(crate::error::Error::Internal),
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

    let message_html = views::partials::ChatMessage::builder()
        .message_id(message.id.as_uuid().to_string())
        .author(user.username.clone())
        .body(message.body.to_string())
        .status(format!("{:?}", message.status))
        .build()
        .render()
        .into_string();
    broadcast_message(&state, &message_html);

    let response = match crate::request::current_kind() {
        crate::request::Kind::Datastar => (
            StatusCode::OK,
            axum::response::Html(message_html),
        )
            .into_response(),
        crate::request::Kind::Page => {
            axum::response::Redirect::to("/demo/chat").into_response()
        }
    };

    Ok(response)
}

async fn ensure_room(
    state: &crate::State,
    user_id: &str,
) -> Result<domain::chat::Room, crate::error::Error> {
    let room_name = "Lobby".to_string();
    if let Some(room) = state.chat.find_room_by_name(room_name.clone()).await? {
        let _ = state
            .chat
            .join_room(
                app::chat::JoinRoom::builder()
                    .room_id(room.id.as_uuid().to_string())
                    .user_id(user_id.to_string())
                    .build(),
            )
            .await;
        return Ok(room);
    }

    let room = state
        .chat
        .create_room(
            app::chat::CreateRoom::builder()
                .name(room_name)
                .created_by(user_id.to_string())
                .build(),
        )
        .await?;
    Ok(room)
}

fn broadcast_message(
    state: &crate::State,
    message_html: &str,
) {
    let event = PatchElements::new(message_html)
        .selector("#chat-messages")
        .mode(ElementPatchMode::Append)
        .into_datastar_event();
    let _ = state
        .sse
        .broadcast(crate::sse::Event::from_event(event));
}

fn to_message_views(
    messages: &[domain::chat::Message],
) -> Vec<views::partials::ChatMessage> {
    messages
        .iter()
        .map(|message| {
            let author = format!(
                "User {}",
                &message.user_id.as_uuid().to_string()[..8]
            );
            views::partials::ChatMessage::builder()
                .message_id(message.id.as_uuid().to_string())
                .author(author)
                .body(message.body.to_string())
                .status(format!("{:?}", message.status))
                .build()
        })
        .collect()
}
