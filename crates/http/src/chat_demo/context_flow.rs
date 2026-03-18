use std::collections::HashMap;

use statum::{machine, state, transition};

use crate::views::partials;
use domain::{chat, user};

#[derive(Clone, Debug)]
pub struct ViewerResolvedData {
    chat_user_id: domain::chat::UserId,
}

#[derive(Clone, Debug)]
pub struct RoomReadyData {
    room: domain::chat::Room,
    chat_user_id: domain::chat::UserId,
}

#[derive(Clone, Debug)]
pub struct MessagesLoadedData {
    room: domain::chat::Room,
    messages: Vec<domain::chat::Message>,
}

#[derive(Clone, Debug)]
pub struct ContextBuiltData {
    room: domain::chat::Room,
    message_views: Vec<partials::components::chat::Message>,
}

#[state]
pub enum ChatDemoContextState {
    Incoming,
    ViewerResolved(ViewerResolvedData),
    RoomReady(RoomReadyData),
    MessagesLoaded(MessagesLoadedData),
    ContextBuilt(ContextBuiltData),
}

#[machine]
pub(super) struct ChatDemoContextFlow<ChatDemoContextState> {
    viewer_user_id: Option<user::Id>,
    room_name: chat::room::Name,
}

impl ChatDemoContextFlow<Incoming> {
    pub(super) fn from_viewer(maybe_viewer_user_id: Option<domain::user::Id>) -> Self {
        ChatDemoContextFlow::<Incoming>::builder()
            .viewer_user_id(maybe_viewer_user_id)
            .room_name(domain::chat::room::Name::Lobby)
            .build()
    }

    pub(super) async fn resolve_viewer(
        self,
        state: &crate::State,
    ) -> crate::Result<ChatDemoContextFlow<ViewerResolved>> {
        let viewer_user_id = match self.viewer_user_id {
            Some(viewer_user_id) => viewer_user_id,
            None => super::ensure_demo_user(state).await?.id,
        };
        Ok(self.mark_viewer_resolved(viewer_user_id))
    }
}

#[transition]
impl ChatDemoContextFlow<Incoming> {
    fn mark_viewer_resolved(
        self,
        viewer_user_id: domain::user::Id,
    ) -> ChatDemoContextFlow<ViewerResolved> {
        self.transition_with(ViewerResolvedData {
            chat_user_id: domain::chat::UserId::from_uuid(*viewer_user_id.as_uuid()),
        })
    }
}

impl ChatDemoContextFlow<ViewerResolved> {
    pub(super) async fn ensure_room(
        self,
        state: &crate::State,
    ) -> crate::Result<ChatDemoContextFlow<RoomReady>> {
        let room = match state.chat.find_room_by_name(self.room_name()).await? {
            Some(room) => {
                state
                    .chat
                    .join_room(
                        app::chat::JoinRoom::builder()
                            .room_id(room.id)
                            .user_id(self.chat_user_id())
                            .build(),
                    )
                    .await?;
                room
            }
            None => {
                state
                    .chat
                    .create_room(
                        app::chat::CreateRoom::builder()
                            .name(self.room_name())
                            .created_by(self.chat_user_id())
                            .build(),
                    )
                    .await?
            }
        };

        Ok(self.mark_room_ready(room))
    }

    pub(super) fn chat_user_id(&self) -> domain::chat::UserId {
        self.state_data.chat_user_id
    }
}

#[transition]
impl ChatDemoContextFlow<ViewerResolved> {
    fn mark_room_ready(self, room: domain::chat::Room) -> ChatDemoContextFlow<RoomReady> {
        let chat_user_id = self.state_data.chat_user_id;
        self.transition_with(RoomReadyData { room, chat_user_id })
    }
}

impl ChatDemoContextFlow<RoomReady> {
    pub(super) async fn load_messages(
        self,
        state: &crate::State,
    ) -> crate::Result<ChatDemoContextFlow<MessagesLoaded>> {
        let messages = state
            .chat
            .list_messages(
                app::chat::ListMessages::builder()
                    .room_id(self.room().id)
                    .user_id(self.chat_user_id())
                    .build(),
            )
            .await?;

        Ok(self.mark_messages_loaded(messages))
    }

    pub(super) fn room(&self) -> &domain::chat::Room {
        &self.state_data.room
    }

    pub(super) fn chat_user_id(&self) -> domain::chat::UserId {
        self.state_data.chat_user_id
    }
}

#[transition]
impl ChatDemoContextFlow<RoomReady> {
    fn mark_messages_loaded(
        self,
        messages: Vec<domain::chat::Message>,
    ) -> ChatDemoContextFlow<MessagesLoaded> {
        let room = self.state_data.room.clone();
        self.transition_with(MessagesLoadedData { room, messages })
    }
}

impl ChatDemoContextFlow<MessagesLoaded> {
    pub(super) async fn build_context(
        self,
        state: &crate::State,
    ) -> ChatDemoContextFlow<ContextBuilt> {
        let message_views = map_message_views(state, &self.state_data.messages).await;
        self.mark_context_built(message_views)
    }
}

#[transition]
impl ChatDemoContextFlow<MessagesLoaded> {
    fn mark_context_built(
        self,
        message_views: Vec<partials::components::chat::Message>,
    ) -> ChatDemoContextFlow<ContextBuilt> {
        let room = self.state_data.room.clone();
        self.transition_with(ContextBuiltData {
            room,
            message_views,
        })
    }
}

impl ChatDemoContextFlow<ContextBuilt> {
    pub(super) fn into_context(self) -> super::ChatContext {
        super::ChatContext {
            room: self.state_data.room,
            messages: self.state_data.message_views,
        }
    }
}

impl<S: ChatDemoContextStateTrait> ChatDemoContextFlow<S> {
    fn room_name(&self) -> domain::chat::room::Name {
        self.room_name
    }
}

async fn map_message_views(
    state: &crate::State,
    messages: &[domain::chat::Message],
) -> Vec<partials::components::chat::Message> {
    let mut names = HashMap::new();
    for message in messages {
        let user_id = domain::user::Id::from_uuid(*message.user_id.as_uuid());
        if names.contains_key(&user_id) {
            continue;
        }
        match state.auth.get_user(&user_id).await {
            Ok(Some(user)) => {
                names.insert(user_id, user.username.to_string());
            }
            Ok(None) => {}
            Err(error) => {
                tracing::debug!(
                    user_id = %user_id.as_uuid(),
                    %error,
                    "author lookup fell back to synthetic label"
                );
            }
        }
    }

    messages
        .iter()
        .map(|message| {
            let user_id = domain::user::Id::from_uuid(*message.user_id.as_uuid());
            let author = names
                .get(&user_id)
                .cloned()
                .unwrap_or_else(|| fallback_author_label(&user_id));
            partials::components::chat::Message::builder()
                .message_id(crate::types::Text::from(message.id.as_uuid().to_string()))
                .author(crate::types::Text::from(author))
                .timestamp(crate::types::Text::from(super::format_message_time(
                    message.created_at,
                )))
                .body(crate::types::Text::from(message.body.to_string()))
                .status(to_chat_message_status(message.status))
                .build()
        })
        .collect()
}

fn fallback_author_label(user_id: &domain::user::Id) -> String {
    format!("user-{}", &user_id.as_uuid().to_string()[..8])
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

pub(super) type IncomingFlow = ChatDemoContextFlow<Incoming>;

#[cfg(test)]
mod tests {
    use std::sync::{
        Arc, Mutex,
        atomic::{AtomicUsize, Ordering},
    };

    use async_trait::async_trait;

    use super::*;
    use app::{auth, user};
    use tower_cookies::Key;

    struct TestUserRepo {
        demo_user: domain::user::User,
    }

    #[async_trait]
    impl user::Repository for TestUserRepo {
        async fn find_by_email(
            &self,
            _email: &domain::user::Email,
        ) -> user::Result<Option<domain::user::User>> {
            Ok(Some(self.demo_user.clone()))
        }

        async fn create_with_credentials(
            &self,
            _user: &domain::user::User,
            _password_hash: &auth::password::Hash,
        ) -> user::Result<()> {
            Ok(())
        }
    }

    struct TestHasher;

    impl auth::password::Hasher for TestHasher {
        fn hash(&self, _password: &str) -> auth::Result<auth::password::Hash> {
            Ok(auth::password::Hash::new("hash"))
        }

        fn verify(
            &self,
            _password: &str,
            _password_hash: &auth::password::Hash,
        ) -> auth::Result<bool> {
            Ok(true)
        }
    }

    struct TestChatRepo {
        room: Mutex<Option<domain::chat::Room>>,
        messages: Mutex<Vec<domain::chat::Message>>,
        create_room_calls: AtomicUsize,
        membership_calls: AtomicUsize,
    }

    impl TestChatRepo {
        fn new(
            room: Option<domain::chat::Room>,
            messages: Vec<domain::chat::Message>,
        ) -> Self {
            Self {
                room: Mutex::new(room),
                messages: Mutex::new(messages),
                create_room_calls: AtomicUsize::new(0),
                membership_calls: AtomicUsize::new(0),
            }
        }

        fn create_room_calls(&self) -> usize {
            self.create_room_calls.load(Ordering::SeqCst)
        }

        fn membership_calls(&self) -> usize {
            self.membership_calls.load(Ordering::SeqCst)
        }
    }

    #[async_trait]
    impl app::chat::Repository for TestChatRepo {
        async fn create_room(&self, room: &domain::chat::Room) -> app::chat::Result<()> {
            self.create_room_calls.fetch_add(1, Ordering::SeqCst);
            *self.room.lock().expect("room lock") = Some(room.clone());
            Ok(())
        }

        async fn find_room(
            &self,
            room_id: &domain::chat::room::Id,
        ) -> app::chat::Result<Option<domain::chat::Room>> {
            let room = self.room.lock().expect("room lock");
            Ok(room.as_ref().filter(|room| room.id == *room_id).cloned())
        }

        async fn find_room_by_name(
            &self,
            name: &domain::chat::room::Name,
        ) -> app::chat::Result<Option<domain::chat::Room>> {
            let room = self.room.lock().expect("room lock");
            Ok(room.as_ref().filter(|room| room.name == *name).cloned())
        }

        async fn list_messages(
            &self,
            _room_id: &domain::chat::room::Id,
            _limit: usize,
        ) -> app::chat::Result<Vec<domain::chat::Message>> {
            Ok(self.messages.lock().expect("messages lock").clone())
        }

        async fn find_message(
            &self,
            _message_id: &domain::chat::message::Id,
        ) -> app::chat::Result<Option<domain::chat::Message>> {
            Ok(None)
        }

        async fn insert_message(
            &self,
            _message: &domain::chat::Message,
        ) -> app::chat::Result<()> {
            Ok(())
        }

        async fn add_membership(
            &self,
            _room_id: &domain::chat::room::Id,
            _user_id: &domain::chat::UserId,
            _role: app::chat::RoomRole,
        ) -> app::chat::Result<()> {
            self.membership_calls.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }

        async fn is_member(
            &self,
            _room_id: &domain::chat::room::Id,
            _user_id: &domain::chat::UserId,
        ) -> app::chat::Result<bool> {
            Ok(true)
        }

        async fn update_message_status(
            &self,
            _message_id: &domain::chat::message::Id,
            _status: domain::chat::message::Status,
        ) -> app::chat::Result<app::chat::PendingMutationResult> {
            Ok(app::chat::PendingMutationResult::Applied)
        }
    }

    struct ModerationQueue;

    #[async_trait]
    impl app::chat::moderation::Queue for ModerationQueue {
        async fn enqueue(
            &self,
            _message_id: &domain::chat::message::Id,
            _reason: &app::chat::moderation::Reason,
        ) -> app::chat::Result<()> {
            Ok(())
        }

        async fn list_pending(
            &self,
            _limit: usize,
        ) -> app::chat::Result<Vec<app::chat::moderation::Item>> {
            Ok(Vec::new())
        }

        async fn complete_if_pending(
            &self,
            _message_id: &domain::chat::message::Id,
            _reviewer_id: &domain::chat::UserId,
            _decision: app::chat::moderation::Decision,
            _reason: Option<app::chat::moderation::Reason>,
        ) -> app::chat::Result<app::chat::PendingMutationResult> {
            Ok(app::chat::PendingMutationResult::Applied)
        }
    }

    struct RateLimiter;

    #[async_trait]
    impl app::chat::RateLimiter for RateLimiter {
        async fn check(
            &self,
            _room_id: &domain::chat::room::Id,
            _user_id: &domain::chat::UserId,
        ) -> app::chat::Result<()> {
            Ok(())
        }
    }

    struct AuditLog;

    #[async_trait]
    impl app::chat::audit::Log for AuditLog {
        async fn record(&self, _entry: app::chat::audit::Entry) -> app::chat::Result<()> {
            Ok(())
        }
    }

    struct Clock;

    impl app::chat::Clock for Clock {
        fn now(&self) -> std::time::SystemTime {
            std::time::SystemTime::UNIX_EPOCH
        }
    }

    struct Ids;

    impl app::chat::IdGenerator for Ids {
        fn new_room_id(&self) -> domain::chat::room::Id {
            domain::chat::room::Id::from_uuid(uuid::Uuid::from_u128(0x1111))
        }

        fn new_message_id(&self) -> domain::chat::message::Id {
            domain::chat::message::Id::from_uuid(uuid::Uuid::from_u128(0x2222))
        }
    }

    fn demo_user() -> domain::user::User {
        domain::user::User::builder()
            .id(domain::user::Id::from_uuid(uuid::Uuid::nil()))
            .username(domain::user::Username::try_new("demo_bot").expect("username"))
            .email(domain::user::Email::try_new("demo.bot@example.com").expect("email"))
            .build()
    }

    fn sample_room() -> domain::chat::Room {
        domain::chat::Room::builder()
            .id(domain::chat::room::Id::from_uuid(uuid::Uuid::from_u128(
                0x1234,
            )))
            .name(domain::chat::room::Name::Lobby)
            .created_by(domain::chat::UserId::from_uuid(uuid::Uuid::nil()))
            .build()
    }

    fn sample_message(room_id: domain::chat::room::Id) -> domain::chat::Message {
        domain::chat::Message::builder()
            .id(domain::chat::message::Id::from_uuid(uuid::Uuid::from_u128(
                0x3333,
            )))
            .room_id(room_id)
            .user_id(domain::chat::UserId::from_uuid(uuid::Uuid::nil()))
            .body(
                domain::chat::message::Body::try_new("hello from test")
                    .expect("valid message body"),
            )
            .status(domain::chat::message::Status::Visible)
            .maybe_client_id(None)
            .created_at(std::time::SystemTime::UNIX_EPOCH)
            .build()
    }

    fn test_state(chat_repo: Arc<TestChatRepo>) -> crate::State {
        let user_service = user::Service::new(
            Arc::new(TestUserRepo {
                demo_user: demo_user(),
            }),
            Arc::new(TestHasher),
        );
        let auth_service = auth::Service::disabled();
        let sse_registry = crate::sse::Registry::new();
        let trace_log = crate::trace_log::Store::builder()
            .with_sse(sse_registry.clone())
            .build();
        let chat_service = app::chat::Service::builder()
            .with_repo(chat_repo)
            .with_moderation_queue(Arc::new(ModerationQueue))
            .with_rate_limiter(Arc::new(RateLimiter))
            .with_audit_log(Arc::new(AuditLog))
            .with_clock(Arc::new(Clock))
            .with_id_generator(Arc::new(Ids))
            .build();

        crate::State::builder()
            .with_user(user_service)
            .with_auth(auth_service)
            .with_chat(chat_service)
            .with_sse(sse_registry)
            .with_cookie_key(Key::generate())
            .with_trace_log(trace_log)
            .build()
    }

    #[tokio::test]
    async fn resolve_viewer_uses_authenticated_user_id() {
        let chat_repo = Arc::new(TestChatRepo::new(Some(sample_room()), Vec::new()));
        let state = test_state(chat_repo);
        let viewer = domain::user::Id::from_uuid(uuid::Uuid::from_u128(0x4444));

        let resolved = IncomingFlow::from_viewer(Some(viewer))
            .resolve_viewer(&state)
            .await
            .expect("viewer resolved");

        assert_eq!(resolved.chat_user_id().as_uuid(), viewer.as_uuid());
    }

    #[tokio::test]
    async fn resolve_viewer_falls_back_to_demo_user() {
        let chat_repo = Arc::new(TestChatRepo::new(Some(sample_room()), Vec::new()));
        let state = test_state(chat_repo);

        let resolved = IncomingFlow::from_viewer(None)
            .resolve_viewer(&state)
            .await
            .expect("viewer resolved");

        assert_eq!(resolved.chat_user_id().as_uuid(), demo_user().id.as_uuid());
    }

    #[tokio::test]
    async fn ensure_room_joins_existing_lobby() {
        let existing_room = sample_room();
        let chat_repo =
            Arc::new(TestChatRepo::new(Some(existing_room.clone()), Vec::new()));
        let state = test_state(chat_repo.clone());
        let viewer = domain::user::Id::from_uuid(uuid::Uuid::from_u128(0x5555));

        let room_ready = IncomingFlow::from_viewer(Some(viewer))
            .resolve_viewer(&state)
            .await
            .expect("viewer resolved")
            .ensure_room(&state)
            .await
            .expect("room ensured");

        assert_eq!(room_ready.room().id, existing_room.id);
        assert_eq!(chat_repo.create_room_calls(), 0);
        assert_eq!(chat_repo.membership_calls(), 1);
    }

    #[tokio::test]
    async fn ensure_room_creates_lobby_when_missing() {
        let chat_repo = Arc::new(TestChatRepo::new(None, Vec::new()));
        let state = test_state(chat_repo.clone());
        let viewer = domain::user::Id::from_uuid(uuid::Uuid::from_u128(0x6666));

        let room_ready = IncomingFlow::from_viewer(Some(viewer))
            .resolve_viewer(&state)
            .await
            .expect("viewer resolved")
            .ensure_room(&state)
            .await
            .expect("room ensured");

        assert_eq!(room_ready.room().name, domain::chat::room::Name::Lobby);
        assert_eq!(chat_repo.create_room_calls(), 1);
        assert_eq!(chat_repo.membership_calls(), 1);
    }

    #[tokio::test]
    async fn build_context_maps_room_and_messages() {
        let room = sample_room();
        let chat_repo = Arc::new(TestChatRepo::new(
            Some(room.clone()),
            vec![sample_message(room.id)],
        ));
        let state = test_state(chat_repo);
        let viewer = domain::user::Id::from_uuid(uuid::Uuid::from_u128(0x7777));

        let context = IncomingFlow::from_viewer(Some(viewer))
            .resolve_viewer(&state)
            .await
            .expect("viewer resolved")
            .ensure_room(&state)
            .await
            .expect("room ensured")
            .load_messages(&state)
            .await
            .expect("messages loaded")
            .build_context(&state)
            .await
            .into_context();

        assert_eq!(context.room, room);
        assert_eq!(context.messages.len(), 1);
        assert_eq!(context.messages[0].body.to_string(), "hello from test");
        assert_eq!(context.messages[0].author.to_string(), "user-00000000");
    }
}
