use axum::{
    extract::Extension,
    http::StatusCode,
    response::IntoResponse,
};
use maud::Render;
use serde::Deserialize;

use crate::views;

#[derive(Deserialize)]
pub struct ChatMessageForm {
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

    let message_views = to_message_views(&messages, &user.id);
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

pub async fn post_chat_message(
    Extension(state): Extension<crate::State>,
    auth_session: crate::auth::Session,
    axum::extract::Form(form): axum::extract::Form<ChatMessageForm>,
) -> Result<impl IntoResponse, crate::error::Error> {
    let user = auth_session
        .user
        .as_ref()
        .ok_or(crate::error::Error::Internal)?;

    let message = state
        .chat
        .post_message(
            app::chat::PostMessage::builder()
                .room_id(form.room_id.clone())
                .user_id(user.id.clone())
                .body(form.body.clone())
                .build(),
        )
        .await?;

    broadcast_message(&state, &message, user).await;

    let messages = state
        .chat
        .list_messages(
            app::chat::ListMessages::builder()
                .room_id(form.room_id)
                .user_id(user.id.clone())
                .build(),
        )
        .await?;

    let message_views = to_message_views(&messages, &user.id);
    let partial = views::partials::ChatMessages::builder()
        .messages(message_views)
        .build();

    Ok((
        StatusCode::OK,
        axum::response::Html(partial.render().into_string()),
    ))
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

async fn broadcast_message(
    state: &crate::State,
    message: &domain::chat::Message,
    user: &crate::auth::User,
) {
    let author = user.username.clone();
    let html = views::partials::ChatMessage::builder()
        .message_id(message.id.as_uuid().to_string())
        .author(author)
        .body(message.body.to_string())
        .status(format!("{:?}", message.status))
        .build()
        .render()
        .into_string();

    let html = html.replace('\\', "\\\\").replace('`', "\\`");
    let script = format!(
        "const list = document.getElementById('chat-messages'); if (list) {{ list.insertAdjacentHTML('beforeend', `{}`); }}",
        html
    );

    let _ = state
        .sse
        .broadcast(crate::sse::Event::execute_script(script));
}

fn to_message_views(
    messages: &[domain::chat::Message],
    current_user_id: &str,
) -> Vec<views::partials::ChatMessage> {
    messages
        .iter()
        .map(|message| {
            let author = author_label(message.user_id.as_uuid(), current_user_id);
            views::partials::ChatMessage::builder()
                .message_id(message.id.as_uuid().to_string())
                .author(author)
                .body(message.body.to_string())
                .status(format!("{:?}", message.status))
                .build()
        })
        .collect()
}

fn author_label(
    user_id: &uuid::Uuid,
    current_user_id: &str,
) -> String {
    if user_id.to_string() == current_user_id {
        "You".to_string()
    } else {
        format!("User {}", &user_id.to_string()[..8])
    }
}
