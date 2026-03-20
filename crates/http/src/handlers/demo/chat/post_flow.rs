use axum::{http, response::IntoResponse, response::Response};
use datastar::prelude::{ElementPatchMode, PatchElements};
use domain::chat;
use maud::Render;
use statum::{machine, state, transition};

use super::post_input::{ChatSender, PostInput};
use super::{ChatSignals, DemoChatSignals};
use crate::trace_log::log::{message, target};
use crate::types::{LogFieldKey, Text, UserIdText};
use crate::views::partials;
use crate::{paths::Route, request};

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
        state: &crate::State,
        signals: ChatSignals,
        user: &crate::auth::User,
    ) -> crate::Result<Self> {
        let input = PostInput::from_authenticated_signals(state, signals, user)?;
        Ok(Self::from_input(input))
    }

    pub(super) async fn from_demo_signals(
        state: &crate::State,
        signals: DemoChatSignals,
    ) -> crate::Result<Self> {
        let input = PostInput::from_demo_signals(state, signals).await?;
        Ok(Self::from_input(input))
    }

    fn from_input(input: PostInput) -> Self {
        ChatPostFlow::<Incoming>::builder()
            .room_id(input.room_id)
            .user_id(input.user_id)
            .body(input.body)
            .body_text(input.body_text)
            .sender(input.sender)
            .author_name(input.author_name)
            .user_id_text(input.user_id_text)
            .request_id(input.request_id)
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
                        crate::types::LogFieldValue::new(self.sender().as_ref()),
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
            .message_id(Text::from(message.id.as_ref().to_string()))
            .author(Text::from(self.author_name.clone()))
            .timestamp(Text::from(crate::chat_demo::format_message_time(
                message.created_at,
            )))
            .body(Text::from(message.body.to_string()))
            .status(partials::components::chat::Status::from(message.status))
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
        let targets = state
            .demo
            .chat_room_bindings
            .stream_keys_for_room(&self.room_id);
        tracing::info!(
            target: target::Known::DemoSse.as_str(),
            message = message::Known::ChatMessageBroadcast.as_str(),
            selector = "[data-chat-messages]",
            mode = "prepend",
            payload_bytes = message_html.len() as u64,
            targets = targets.len()
        );
        if let Err(error) = state
            .sse
            .send_to_stream_keys(targets, crate::sse::Event::from(event))
        {
            tracing::warn!(
                target: target::Known::DemoSse.as_str(),
                ?error,
                room_id = %self.room_id.as_ref(),
                "chat room fanout could not reach active clients"
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
                        crate::types::LogFieldValue::new(self.sender().as_ref()),
                    ),
                    (
                        crate::types::LogFieldName::from(LogFieldKey::Receiver),
                        crate::types::LogFieldValue::new("room-clients"),
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

pub(super) type IncomingFlow = ChatPostFlow<Incoming>;

#[cfg(test)]
mod tests {
    use super::*;

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

    fn incoming() -> ChatPostFlow<Incoming> {
        IncomingFlow::from_input(PostInput {
            room_id: domain::chat::room::Id::new_v4(),
            user_id: domain::chat::UserId::new_v4(),
            body: sample_body(),
            body_text: "hello".to_string(),
            sender: ChatSender::You,
            author_name: "person".to_string(),
            user_id_text: UserIdText::new("user-1"),
            request_id: Text::from("req-1"),
        })
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
}
