use statum::{machine, state, transition};

use crate::types::Text;
use crate::views::partials::chat;

#[derive(Clone, Debug)]
pub struct ViewerData {
    maybe_user: Option<crate::views::page::UserNav>,
    viewer_id: Option<domain::user::UserId>,
    interactivity: chat::Mode,
}

#[derive(Clone, Debug)]
pub struct ChatLoadedData {
    maybe_user: Option<crate::views::page::UserNav>,
    chat_demo: chat::DemoSection,
}

#[state]
pub enum LabPageState {
    Incoming,
    ViewerResolved(ViewerData),
    ChatLoaded(ChatLoadedData),
}

#[machine]
pub(super) struct LabPageFlow<LabPageState> {
    auth_user: Option<crate::auth::User>,
}

impl LabPageFlow<Incoming> {
    pub(super) fn from_auth_user(auth_user: Option<crate::auth::User>) -> Self {
        LabPageFlow::<Incoming>::builder()
            .maybe_auth_user(auth_user)
            .build()
    }

    pub(super) fn resolve_viewer(self) -> crate::Result<LabPageFlow<ViewerResolved>> {
        let is_authenticated = self.auth_user.is_some();
        let maybe_user = self.auth_user.as_ref().map(|user| {
            crate::views::page::UserNav::builder()
                .username(Text::from(user.username.to_string()))
                .email(Text::from(user.email.to_string()))
                .build()
        });
        let viewer_id = self
            .auth_user
            .as_ref()
            .map(|user| user.id.to_domain())
            .transpose()?;
        Ok(self.mark_viewer_resolved(ViewerData {
            maybe_user,
            viewer_id,
            interactivity: chat::Mode::from(is_authenticated),
        }))
    }
}

#[transition]
impl LabPageFlow<Incoming> {
    fn mark_viewer_resolved(self, data: ViewerData) -> LabPageFlow<ViewerResolved> {
        self.transition_with(data)
    }
}

impl LabPageFlow<ViewerResolved> {
    pub(super) async fn load_chat_context(
        self,
        state: &crate::State,
    ) -> crate::Result<LabPageFlow<ChatLoaded>> {
        let context =
            crate::chat_demo::load_chat_context(state, self.state_data.viewer_id).await?;
        let maybe_user = self.state_data.maybe_user.clone();
        let chat_demo = chat::DemoSection::builder()
            .room_id(Text::from(context.room.id.as_uuid().to_string()))
            .room_name(Text::from(context.room.name.to_string()))
            .messages(context.messages)
            .interactivity(self.state_data.interactivity)
            .build();
        Ok(self.mark_chat_loaded(ChatLoadedData {
            maybe_user,
            chat_demo,
        }))
    }
}

#[transition]
impl LabPageFlow<ViewerResolved> {
    fn mark_chat_loaded(self, data: ChatLoadedData) -> LabPageFlow<ChatLoaded> {
        self.transition_with(data)
    }
}

impl LabPageFlow<ChatLoaded> {
    pub(super) fn into_page(self) -> crate::views::pages::Lab {
        crate::views::pages::Lab::builder()
            .maybe_user(self.state_data.maybe_user)
            .chat_demo(self.state_data.chat_demo)
            .build()
    }
}

pub(super) type IncomingFlow = LabPageFlow<Incoming>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_viewer_for_guest_sets_chat_to_read_only() {
        let incoming = LabPageFlow::<Incoming>::from_auth_user(None);
        let resolved = incoming.resolve_viewer().expect("resolved");
        assert!(matches!(
            resolved.state_data.interactivity,
            chat::Mode::DemoOnly
        ));
    }
}
