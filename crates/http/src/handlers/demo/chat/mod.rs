// ci: descriptive-module-import crate::handlers::demo::chat
mod moderate_flow;
mod post_flow;

use axum::extract::Extension;
use axum::response::Response;
use datastar::axum::ReadSignals;
use serde::Deserialize;

use crate::types::{SseTabId, Text};
use crate::{request, views};
use post_flow::IncomingFlow as ChatPostIncomingFlow;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatSignals {
    #[serde(rename = "chatDraftBody")]
    pub draft_body: Text,
    pub sse_tab_id: Option<SseTabId>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DemoChatSignals {
    #[serde(rename = "chatDemoDraftBody")]
    pub draft_body: Text,
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

    let incoming = moderate_flow::IncomingFlow::from_form(form, user.id.clone())?;
    let parsed = incoming.parse()?;
    let _applied = parsed.apply(&state).await?;

    moderation_page(Extension(state), auth_session).await
}

pub async fn post_chat_message(
    Extension(state): Extension<crate::State>,
    auth_session: crate::auth::Session,
    ReadSignals(signals): ReadSignals<ChatSignals>,
) -> Result<Response, crate::Error> {
    if let Some(tab_id) = signals.sse_tab_id.clone() {
        request::set_sse_tab_id(tab_id);
    }
    let user = auth_session
        .user
        .as_ref()
        .ok_or(crate::error::Error::Internal)?;

    let incoming = ChatPostIncomingFlow::from_authenticated_signals(&state, signals, user)?;
    incoming.post_and_respond(&state).await
}

pub async fn post_demo_chat_message(
    Extension(state): Extension<crate::State>,
    ReadSignals(signals): ReadSignals<DemoChatSignals>,
) -> Result<Response, crate::Error> {
    if let Some(tab_id) = signals.sse_tab_id.clone() {
        request::set_sse_tab_id(tab_id);
    }
    let incoming = ChatPostIncomingFlow::from_demo_signals(&state, signals).await?;
    incoming.post_and_respond(&state).await
}
