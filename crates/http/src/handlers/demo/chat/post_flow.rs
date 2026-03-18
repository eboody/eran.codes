use axum::{http, response::IntoResponse, response::Response};
use datastar::prelude::{ElementPatchMode, PatchElements};
use domain::chat;
use maud::Render;
use statum::{machine, state, transition};

use super::{ChatSignals, DemoChatSignals};
use crate::trace_log::log::{message, target};
use crate::types::{LogFieldKey, Text, UserIdText};
use crate::views::partials;
use crate::{paths::Route, request};

#[derive(Clone, Copy, Debug)]
pub(super) enum ChatSender {
    You,
    Demo,
}

impl ChatSender {
    pub(super) fn as_str(self) -> &'static str {
        match self {
            Self::You => "you",
            Self::Demo => "demo",
        }
    }
}

#[derive(Clone, Debug)]
pub struct PostedData {
    message: domain::chat::Message,
}

#[derive(Clone, Debug)]
pub struct HtmlData {
    message_markup: maud::Markup,
}

#[state]
pub enum ChatPostState {
    Incoming,
    CommandBuilt,
    MessagePosted(PostedData),
    IncomingRecorded(PostedData),
    HtmlRendered(HtmlData),
    Broadcasted(HtmlData),
}

#[machine]
pub(super) struct ChatPostFlow<ChatPostState> {
    room_id: chat::room::Id,
    user_id: chat::UserId,
    body: chat::message::Body,
    body_text: String,
    sender: ChatSender,
    author_name: String,
    user_id_text: UserIdText,
    request_id: Text,
}

impl ChatPostFlow<Incoming> {
    pub(super) fn from_authenticated_signals(
        signals: ChatSignals,
        user: &crate::auth::User,
    ) -> crate::Result<Self> {
        let body_text = signals.body.to_string();

        Ok(Self::new(
            Self::room_id_from_text(&signals.room_id.to_string())?,
            Self::chat_user_id_from_user_id(user.id.to_domain()?),
            Self::message_body_from_text(&body_text)?,
            body_text,
            ChatSender::You,
            user.username.to_string(),
            UserIdText::new(user.id.to_string()),
            Self::request_id_from_context(),
        ))
    }

    pub(super) async fn from_demo_signals(
        state: &crate::State,
        signals: DemoChatSignals,
    ) -> crate::Result<Self> {
        let demo_user = crate::chat_demo::ensure_demo_user(state).await?;
        let room_id = Self::room_id_from_text(&signals.room_id.to_string())?;
        let chat_user_id = Self::chat_user_id_from_user_id(demo_user.id);

        state
            .chat
            .join_room(
                app::chat::JoinRoom::builder()
                    .room_id(room_id)
                    .user_id(chat_user_id)
                    .build(),
            )
            .await?;

        let body_text = signals.bot_body.to_string();

        Ok(Self::new(
            room_id,
            chat_user_id,
            Self::message_body_from_text(&body_text)?,
            body_text,
            ChatSender::Demo,
            demo_user.username.to_string(),
            UserIdText::new(demo_user.id.as_uuid().to_string()),
            Self::request_id_from_context(),
        ))
    }

    #[allow(clippy::too_many_arguments)]
    fn new(
        room_id: domain::chat::room::Id,
        user_id: domain::chat::UserId,
        body: domain::chat::message::Body,
        body_text: String,
        sender: ChatSender,
        author_name: String,
        user_id_text: UserIdText,
        request_id: Text,
    ) -> Self {
        ChatPostFlow::<Incoming>::builder()
            .room_id(room_id)
            .user_id(user_id)
            .body(body)
            .body_text(body_text)
            .sender(sender)
            .author_name(author_name)
            .user_id_text(user_id_text)
            .request_id(request_id)
            .build()
    }

    pub(super) async fn post_and_respond(
        self,
        state: &crate::State,
    ) -> Result<Response, crate::Error> {
        let posted = self.mark_command_built().post_message(state).await?;
        let rendered = posted.record_incoming(state).render_message_html();
        let broadcasted = rendered.broadcast(state);

        Ok(broadcasted.into_response())
    }

    fn request_id_from_context() -> Text {
        request::current_context()
            .and_then(|context| context.request_id)
            .map(|request_id| Text::from(request_id.to_string()))
            .unwrap_or_else(|| Text::from(format!("fallback-{}", uuid::Uuid::new_v4())))
    }

    fn room_id_from_text(value: &str) -> Result<domain::chat::room::Id, crate::Error> {
        let id = value.parse::<uuid::Uuid>().map_err(|error| {
            crate::Error::from(app::chat::Error::invalid_room_id(error))
        })?;

        Ok(domain::chat::room::Id::from_uuid(id))
    }

    fn message_body_from_text(
        value: &str,
    ) -> Result<domain::chat::message::Body, crate::Error> {
        domain::chat::message::Body::try_new(value)
            .map_err(domain::chat::Error::from)
            .map_err(app::chat::Error::from)
            .map_err(crate::Error::from)
    }

    fn chat_user_id_from_user_id(value: domain::user::Id) -> domain::chat::UserId {
        domain::chat::UserId::from_uuid(*value.as_uuid())
    }
}

impl<S: ChatPostStateTrait> ChatPostFlow<S> {
    pub(super) fn sender(&self) -> ChatSender {
        self.sender
    }

    pub(super) fn user_id_text(&self) -> &UserIdText {
        &self.user_id_text
    }

    pub(super) fn request_id(&self) -> &Text {
        &self.request_id
    }
}

#[transition]
impl ChatPostFlow<Incoming> {
    pub(super) fn mark_command_built(self) -> ChatPostFlow<CommandBuilt> {
        self.transition()
    }
}

impl ChatPostFlow<CommandBuilt> {
    pub(super) fn command(&self) -> app::chat::PostMessage {
        app::chat::PostMessage::builder()
            .room_id(self.room_id)
            .user_id(self.user_id)
            .body(self.body.clone())
            .build()
    }

    pub(super) async fn post_message(
        self,
        state: &crate::State,
    ) -> Result<ChatPostFlow<MessagePosted>, crate::Error> {
        let message = state.chat.post_message(self.command()).await?;
        Ok(self.mark_message_posted(message))
    }
}

#[transition]
impl ChatPostFlow<CommandBuilt> {
    pub(super) fn mark_message_posted(
        self,
        message: domain::chat::Message,
    ) -> ChatPostFlow<MessagePosted> {
        self.transition_with(PostedData { message })
    }
}

impl ChatPostFlow<MessagePosted> {
    pub(super) fn payload_bytes(&self) -> usize {
        self.body_text.len()
    }

    pub(super) fn record_incoming(
        self,
        state: &crate::State,
    ) -> ChatPostFlow<IncomingRecorded> {
        self.trace_incoming(state);
        self.mark_incoming_recorded()
    }

    fn trace_incoming(&self, state: &crate::State) {
        let sse_tab_id = request::current_context().and_then(|value| value.sse_tab_id);
        state.trace_log.record_sse_event(
            request::current_context()
                .and_then(|value| value.session_id)
                .as_ref(),
            crate::trace_log::store::TraceEntry::builder()
                .timestamp(crate::trace_log::now_timestamp_short())
                .level(crate::types::LogLevelText::new("INFO"))
                .target(crate::types::LogTargetText::from(target::Known::DemoChat))
                .message(crate::types::LogMessageText::from(
                    message::Known::ChatMessageIncoming,
                ))
                .fields(vec![
                    (
                        crate::types::LogFieldName::from(LogFieldKey::Direction),
                        crate::types::LogFieldValue::new("incoming"),
                    ),
                    (
                        crate::types::LogFieldName::from(LogFieldKey::Sender),
                        crate::types::LogFieldValue::new(self.sender().as_str()),
                    ),
                    (
                        crate::types::LogFieldName::from(LogFieldKey::Receiver),
                        crate::types::LogFieldValue::new("server"),
                    ),
                    (
                        crate::types::LogFieldName::from(LogFieldKey::UserId),
                        crate::types::LogFieldValue::new(self.user_id_text().to_string()),
                    ),
                    (
                        crate::types::LogFieldName::from(LogFieldKey::PayloadBytes),
                        crate::types::LogFieldValue::new(self.payload_bytes().to_string()),
                    ),
                    (
                        crate::types::LogFieldName::from(LogFieldKey::RequestId),
                        crate::types::LogFieldValue::new(self.request_id().to_string()),
                    ),
                    (
                        crate::types::LogFieldName::from(LogFieldKey::SseTabId),
                        sse_tab_id
                            .clone()
                            .map(|value| {
                                crate::types::LogFieldValue::new(value.to_string())
                            })
                            .unwrap_or_else(crate::types::LogFieldValue::missing),
                    ),
                ])
                .build(),
        );
    }
}

#[transition]
impl ChatPostFlow<MessagePosted> {
    pub(super) fn mark_incoming_recorded(self) -> ChatPostFlow<IncomingRecorded> {
        let data = self.state_data.clone();
        self.transition_with(data)
    }
}

impl ChatPostFlow<IncomingRecorded> {
    pub(super) fn render_message_html(self) -> ChatPostFlow<HtmlRendered> {
        let message = &self.state_data.message;
        let markup = partials::components::chat::Message::builder()
            .message_id(Text::from(message.id.as_uuid().to_string()))
            .author(Text::from(self.author_name.clone()))
            .timestamp(Text::from(crate::chat_demo::format_message_time(
                message.created_at,
            )))
            .body(Text::from(message.body.to_string()))
            .status(to_chat_message_status(message.status))
            .build()
            .render();
        self.mark_html_rendered(markup)
    }
}

#[transition]
impl ChatPostFlow<IncomingRecorded> {
    fn mark_html_rendered(
        self,
        message_markup: maud::Markup,
    ) -> ChatPostFlow<HtmlRendered> {
        self.transition_with(HtmlData { message_markup })
    }
}

impl ChatPostFlow<HtmlRendered> {
    pub(super) fn message_markup(&self) -> &maud::Markup {
        &self.state_data.message_markup
    }

    pub(super) fn broadcast(self, state: &crate::State) -> ChatPostFlow<Broadcasted> {
        self.broadcast_message(state);
        self.mark_broadcasted()
    }

    fn broadcast_message(&self, state: &crate::State) {
        let message_html = self.message_markup().clone().into_string();
        let event = PatchElements::new(message_html.as_str())
            .selector("[data-chat-messages]")
            .mode(ElementPatchMode::Prepend)
            .into_datastar_event();
        tracing::info!(
            target: target::Known::DemoSse.as_str(),
            message = message::Known::ChatMessageBroadcast.as_str(),
            selector = "[data-chat-messages]",
            mode = "prepend",
            payload_bytes = message_html.len() as u64
        );
        if let Err(error) = state.sse.broadcast(crate::sse::Event::from_event(event)) {
            tracing::warn!(
                target: target::Known::DemoSse.as_str(),
                ?error,
                "chat broadcast could not reach active clients"
            );
        }

        let context = request::current_context();
        let session_id = context.as_ref().and_then(|value| value.session_id.clone());
        let sse_tab_id = context.and_then(|value| value.sse_tab_id);
        state.trace_log.record_sse_event(
            session_id.as_ref(),
            crate::trace_log::store::TraceEntry::builder()
                .timestamp(crate::trace_log::now_timestamp_short())
                .level(crate::types::LogLevelText::new("INFO"))
                .target(crate::types::LogTargetText::from(target::Known::DemoSse))
                .message(crate::types::LogMessageText::from(
                    message::Known::ChatMessageBroadcast,
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
                        crate::types::LogFieldValue::new(self.sender().as_str()),
                    ),
                    (
                        crate::types::LogFieldName::from(LogFieldKey::Receiver),
                        crate::types::LogFieldValue::new("clients"),
                    ),
                    (
                        crate::types::LogFieldName::from(LogFieldKey::UserId),
                        crate::types::LogFieldValue::new(self.user_id_text().to_string()),
                    ),
                    (
                        crate::types::LogFieldName::from(LogFieldKey::RequestId),
                        crate::types::LogFieldValue::new(self.request_id().to_string()),
                    ),
                    (
                        crate::types::LogFieldName::from(LogFieldKey::SseTabId),
                        sse_tab_id
                            .clone()
                            .map(|value| {
                                crate::types::LogFieldValue::new(value.to_string())
                            })
                            .unwrap_or_else(crate::types::LogFieldValue::missing),
                    ),
                ])
                .build(),
        );
    }
}

#[transition]
impl ChatPostFlow<HtmlRendered> {
    pub(super) fn mark_broadcasted(self) -> ChatPostFlow<Broadcasted> {
        let data = self.state_data.clone();
        self.transition_with(data)
    }
}

impl ChatPostFlow<Broadcasted> {
    pub(super) fn into_response(self) -> axum::response::Response {
        match request::current_kind() {
            request::Kind::Datastar => http::StatusCode::ACCEPTED.into_response(),
            request::Kind::Page => {
                let target = format!(
                    "{}#{}",
                    Route::Home.as_str(),
                    partials::chat::DemoSection::ANCHOR_ID
                );
                axum::response::Redirect::to(target.as_str()).into_response()
            }
        }
    }
}

fn to_chat_message_status(
    value: domain::chat::message::Status,
) -> partials::components::chat::Status {
    match value {
        domain::chat::message::Status::Visible => {
            partials::components::chat::Status::Visible
        }
        domain::chat::message::Status::Pending => {
            partials::components::chat::Status::Pending
        }
        domain::chat::message::Status::Removed => {
            partials::components::chat::Status::Removed
        }
    }
}

pub(super) type IncomingFlow = ChatPostFlow<Incoming>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::SseTabId;

    fn sample_body() -> domain::chat::message::Body {
        domain::chat::message::Body::try_new("hello").expect("valid body")
    }

    fn sample_message() -> domain::chat::Message {
        domain::chat::Message::builder()
            .id(domain::chat::message::Id::new_v4())
            .room_id(domain::chat::room::Id::new_v4())
            .user_id(domain::chat::UserId::new_v4())
            .body(sample_body())
            .status(domain::chat::message::Status::Visible)
            .maybe_client_id(None)
            .created_at(std::time::SystemTime::UNIX_EPOCH)
            .build()
    }

    fn auth_user() -> crate::auth::User {
        crate::auth::User::builder()
            .id(crate::auth::UserId::from(domain::user::Id::new_v4()))
            .username(domain::user::Username::try_new("person").expect("valid username"))
            .email(domain::user::Email::try_new("person@example.com").expect("valid email"))
            .session_hash_bytes(vec![1, 2, 3])
            .build()
    }

    fn incoming() -> ChatPostFlow<Incoming> {
        IncomingFlow::new(
            domain::chat::room::Id::new_v4(),
            domain::chat::UserId::new_v4(),
            sample_body(),
            "hello".to_string(),
            ChatSender::You,
            "person".to_string(),
            UserIdText::new("user-1"),
            Text::from("req-1"),
        )
    }

    #[test]
    fn command_built_state_produces_post_command() {
        let built = incoming().mark_command_built();
        let command = built.command();
        assert_eq!(command.body.to_string(), "hello");
    }

    #[test]
    fn rendered_state_contains_message_markup() {
        let posted = incoming()
            .mark_command_built()
            .mark_message_posted(sample_message());
        let rendered = posted.mark_incoming_recorded().render_message_html();
        let markup = rendered.message_markup().clone().into_string();

        assert!(markup.contains("person"));
        assert!(markup.contains("hello"));
    }

    #[test]
    fn authenticated_constructor_parses_signals_into_command() {
        let user = auth_user();
        let incoming = IncomingFlow::from_authenticated_signals(
            ChatSignals {
                room_id: Text::from(domain::chat::room::Id::new_v4().as_uuid().to_string()),
                body: Text::from("hello"),
                sse_tab_id: Some(SseTabId::new("tab-1")),
            },
            &user,
        )
        .expect("incoming");

        let command = incoming.mark_command_built().command();

        assert_eq!(command.body.to_string(), "hello");
        assert_eq!(
            command.user_id.as_uuid(),
            user.id.to_domain().unwrap().as_uuid()
        );
    }
}
