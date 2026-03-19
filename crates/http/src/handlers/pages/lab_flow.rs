use statum::{machine, state, transition};

use crate::types::Text;
use crate::views::partials;

#[derive(Clone, Debug)]
pub struct ViewerData {
    maybe_user: Option<crate::views::page::UserNav>,
    viewer_id: Option<domain::user::Id>,
    chat_mode: partials::components::chat::Mode,
}

#[derive(Clone, Debug)]
pub struct ChatLoadedData {
    maybe_user: Option<crate::views::page::UserNav>,
    room_id: domain::chat::room::Id,
    chat_demo: partials::chat::DemoSection,
}

#[state]
pub enum LabPageState {
    Incoming,
    ViewerResolved(ViewerData),
    ChatLoaded(ChatLoadedData),
    PageReady(PageReadyData),
}

#[derive(Clone, Debug)]
pub struct PageReadyData {
    maybe_user: Option<crate::views::page::UserNav>,
    chat_demo: partials::chat::DemoSection,
    sse_tab_id: crate::types::SseTabId,
}

#[machine]
pub(super) struct LabPageFlow<LabPageState> {
    auth_user: Option<crate::auth::User>,
}

impl LabPageFlow<Incoming> {
    pub(super) fn from_auth_user(auth_user: Option<crate::auth::User>) -> Self {
        LabPageFlow::<Incoming>::builder()
            .auth_user(auth_user)
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
            chat_mode: partials::components::chat::Mode::from(is_authenticated),
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
        let chat_demo = partials::chat::DemoSection::builder()
            .room_id(Text::from(context.room.id.as_uuid().to_string()))
            .room_name(Text::from(context.room.name.to_string()))
            .messages(context.messages)
            .mode(self.state_data.chat_mode)
            .build();
        Ok(self.mark_chat_loaded(ChatLoadedData {
            maybe_user,
            room_id: context.room.id,
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
    pub(super) fn bind_live_tab(
        self,
        cookies: &tower_cookies::Cookies,
        cookie_key: &tower_cookies::Key,
        room_bindings: &crate::chat_demo::room::Bindings,
    ) -> LabPageFlow<PageReady> {
        let ChatLoadedData {
            maybe_user,
            room_id,
            chat_demo,
        } = self.state_data.clone();
        let sse_tab_id = crate::types::SseTabId::new(uuid::Uuid::new_v4().to_string());
        let session = crate::sse::Handle::from_cookies_with_tab(
            cookies,
            cookie_key,
            Some(sse_tab_id.clone()),
        );
        room_bindings.bind(&session, room_id);

        self.mark_page_ready(PageReadyData {
            maybe_user,
            chat_demo,
            sse_tab_id,
        })
    }
}

#[transition]
impl LabPageFlow<ChatLoaded> {
    fn mark_page_ready(self, data: PageReadyData) -> LabPageFlow<PageReady> {
        self.transition_with(data)
    }
}

impl LabPageFlow<PageReady> {
    pub(super) fn into_page(self) -> crate::views::pages::Lab {
        crate::views::pages::Lab::builder()
            .maybe_user(self.state_data.maybe_user)
            .chat_demo(self.state_data.chat_demo)
            .sse_tab_id(self.state_data.sse_tab_id)
            .build()
    }
}

pub(super) type IncomingFlow = LabPageFlow<Incoming>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Text;

    #[test]
    fn resolve_viewer_for_guest_sets_chat_to_read_only() {
        let incoming = LabPageFlow::<Incoming>::from_auth_user(None);
        let resolved = incoming.resolve_viewer().expect("resolved");
        assert!(matches!(
            resolved.state_data.chat_mode,
            partials::components::chat::Mode::DemoOnly
        ));
    }

    #[test]
    fn bind_live_tab_binds_loaded_room_to_generated_handle() {
        let room_id = domain::chat::room::Id::new_v4();
        let chat_demo = partials::chat::DemoSection::builder()
            .room_id(Text::from(room_id.as_uuid().to_string()))
            .room_name(Text::from(domain::chat::room::Name::Lobby.to_string()))
            .messages(Vec::new())
            .mode(partials::components::chat::Mode::DemoOnly)
            .build();
        let flow = LabPageFlow::<Incoming>::from_auth_user(None)
            .mark_viewer_resolved(ViewerData {
                maybe_user: None,
                viewer_id: None,
                chat_mode: partials::components::chat::Mode::DemoOnly,
            })
            .mark_chat_loaded(ChatLoadedData {
                maybe_user: None,
                room_id,
                chat_demo,
            });
        let cookies = tower_cookies::Cookies::default();
        let cookie_key = tower_cookies::Key::generate();
        let room_bindings = crate::chat_demo::room::Bindings::new();

        let page_ready = flow.bind_live_tab(&cookies, &cookie_key, &room_bindings);
        let handle = crate::sse::Handle::from_cookies_with_tab(
            &cookies,
            &cookie_key,
            Some(page_ready.state_data.sse_tab_id.clone()),
        );

        assert_eq!(room_bindings.room_id_for(&handle), Some(room_id));
    }
}
