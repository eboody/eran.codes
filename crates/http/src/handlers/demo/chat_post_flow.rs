use axum::{http::StatusCode, response::IntoResponse};
use datastar::prelude::{ElementPatchMode, PatchElements};
use maud::Render;
use statum::{machine, state, transition};

use crate::trace_log::{LogMessageKnown, LogTargetKnown};
use crate::types::{LogFieldKey, Text, UserIdText};
use crate::views::partials::chat;
use crate::{paths::Route, request};
use domain::chat as domain_chat;

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
    message: domain_chat::Message,
}

#[derive(Clone, Debug)]
pub struct HtmlData {
    message_html: String,
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
    room_id: domain_chat::RoomId,
    user_id: domain_chat::UserId,
    body: domain_chat::MessageBody,
    body_text: String,
    sender: ChatSender,
    author_name: String,
    user_id_text: UserIdText,
    request_id: Text,
}

impl ChatPostFlow<Incoming> {
    #[allow(clippy::too_many_arguments)]
    pub(super) fn new(
        room_id: domain_chat::RoomId,
        user_id: domain_chat::UserId,
        body: domain_chat::MessageBody,
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
    ) -> Result<ChatPostFlow<MessagePosted>, crate::error::Error> {
        let message = state.chat.post_message(self.command()).await?;
        Ok(self.mark_message_posted(message))
    }
}

#[transition]
impl ChatPostFlow<CommandBuilt> {
    pub(super) fn mark_message_posted(
        self,
        message: domain_chat::Message,
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
        record_incoming_chat_event(
            state,
            self.sender(),
            self.user_id_text(),
            self.request_id(),
            self.payload_bytes(),
        );
        self.mark_incoming_recorded()
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
        let markup = chat::Message::builder()
            .message_id(Text::from(message.id.as_uuid().to_string()))
            .author(Text::from(self.author_name.clone()))
            .timestamp(Text::from(crate::chat_demo::format_message_time(
                message.created_at,
            )))
            .body(Text::from(message.body.to_string()))
            .status(to_chat_message_status(message.status))
            .build()
            .render()
            .into_string();
        self.mark_html_rendered(markup)
    }
}

#[transition]
impl ChatPostFlow<IncomingRecorded> {
    fn mark_html_rendered(self, message_html: String) -> ChatPostFlow<HtmlRendered> {
        self.transition_with(HtmlData { message_html })
    }
}

impl ChatPostFlow<HtmlRendered> {
    pub(super) fn message_html(&self) -> &str {
        &self.state_data.message_html
    }

    pub(super) fn broadcast(self, state: &crate::State) -> ChatPostFlow<Broadcasted> {
        broadcast_message(
            state,
            self.message_html(),
            self.sender(),
            self.user_id_text().clone(),
            self.request_id(),
        );
        self.mark_broadcasted()
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
        chat_post_response()
    }
}

fn to_chat_message_status(value: domain_chat::MessageStatus) -> chat::message::Status {
    match value {
        domain_chat::MessageStatus::Visible => chat::message::Status::Visible,
        domain_chat::MessageStatus::Pending => chat::message::Status::Pending,
        domain_chat::MessageStatus::Removed => chat::message::Status::Removed,
    }
}

fn record_incoming_chat_event(
    state: &crate::State,
    sender: ChatSender,
    user_id: &UserIdText,
    request_id: &Text,
    payload_bytes: usize,
) {
    let sse_tab_id = request::current_context().and_then(|value| value.sse_tab_id);
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
                (
                    crate::types::LogFieldName::from(LogFieldKey::RequestId),
                    crate::types::LogFieldValue::new(request_id.to_string()),
                ),
                (
                    crate::types::LogFieldName::from(LogFieldKey::SseTabId),
                    sse_tab_id
                        .clone()
                        .map(|value| crate::types::LogFieldValue::new(value.to_string()))
                        .unwrap_or_else(crate::types::LogFieldValue::missing),
                ),
            ])
            .build(),
    );
}

fn chat_post_response() -> axum::response::Response {
    match request::current_kind() {
        request::Kind::Datastar => StatusCode::ACCEPTED.into_response(),
        request::Kind::Page => {
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
    user_id: UserIdText,
    request_id: &Text,
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

    let context = request::current_context();
    let session_id = context.as_ref().and_then(|value| value.session_id.clone());
    let sse_tab_id = context.and_then(|value| value.sse_tab_id);
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
                (
                    crate::types::LogFieldName::from(LogFieldKey::RequestId),
                    crate::types::LogFieldValue::new(request_id.to_string()),
                ),
                (
                    crate::types::LogFieldName::from(LogFieldKey::SseTabId),
                    sse_tab_id
                        .clone()
                        .map(|value| crate::types::LogFieldValue::new(value.to_string()))
                        .unwrap_or_else(crate::types::LogFieldValue::missing),
                ),
            ])
            .build(),
    );
}

pub(super) type IncomingFlow = ChatPostFlow<Incoming>;

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_body() -> domain_chat::MessageBody {
        domain_chat::MessageBody::try_new("hello").expect("valid body")
    }

    fn sample_message() -> domain_chat::Message {
        domain_chat::Message::builder()
            .id(domain_chat::MessageId::new_v4())
            .room_id(domain_chat::RoomId::new_v4())
            .user_id(domain_chat::UserId::new_v4())
            .body(sample_body())
            .status(domain_chat::MessageStatus::Visible)
            .maybe_client_id(None)
            .created_at(std::time::SystemTime::UNIX_EPOCH)
            .build()
    }

    fn incoming() -> ChatPostFlow<Incoming> {
        IncomingFlow::new(
            domain_chat::RoomId::new_v4(),
            domain_chat::UserId::new_v4(),
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

        assert!(rendered.message_html().contains("person"));
        assert!(rendered.message_html().contains("hello"));
    }
}
