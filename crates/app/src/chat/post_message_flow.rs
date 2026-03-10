use std::time::SystemTime;

use statum::{machine, state, transition};

use super::{Error, ModerationReason, PostMessage};
use domain::chat;

#[derive(Clone, Debug, PartialEq)]
pub struct BuiltVisibleData {
    message: chat::Message,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BuiltPendingData {
    message: chat::Message,
    moderation_reason: ModerationReason,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PersistedVisibleData {
    message: chat::Message,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PersistedPendingData {
    message: chat::Message,
    moderation_reason: ModerationReason,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ModerationEnqueuedData {
    message: chat::Message,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AuditedData {
    message: chat::Message,
}

#[state]
pub enum PostMessageState {
    Incoming,
    RoomVerified,
    MembershipVerified,
    RateLimitPassed,
    BuiltVisible(BuiltVisibleData),
    BuiltPending(BuiltPendingData),
    PersistedVisible(PersistedVisibleData),
    PersistedPending(PersistedPendingData),
    ModerationEnqueued(ModerationEnqueuedData),
    Audited(AuditedData),
}

#[machine]
pub(super) struct PostMessageFlow<PostMessageState> {
    room_id: chat::RoomId,
    user_id: chat::UserId,
    body: chat::MessageBody,
    client_id: Option<chat::ClientId>,
}

#[transition]
impl PostMessageFlow<Incoming> {
    pub(super) fn room_verified(self) -> PostMessageFlow<RoomVerified> {
        self.transition()
    }
}

#[transition]
impl PostMessageFlow<RoomVerified> {
    pub(super) fn membership_verified(self) -> PostMessageFlow<MembershipVerified> {
        self.transition()
    }
}

#[transition]
impl PostMessageFlow<MembershipVerified> {
    pub(super) fn rate_limit_passed(self) -> PostMessageFlow<RateLimitPassed> {
        self.transition()
    }
}

#[transition]
impl PostMessageFlow<RateLimitPassed> {
    pub(super) fn build_visible(
        self,
        message_id: chat::MessageId,
        created_at: SystemTime,
    ) -> PostMessageFlow<BuiltVisible> {
        let message =
            self.build_message(message_id, created_at, chat::MessageStatus::Visible);
        self.transition_with(BuiltVisibleData { message })
    }

    pub(super) fn build_pending(
        self,
        message_id: chat::MessageId,
        created_at: SystemTime,
        moderation_reason: ModerationReason,
    ) -> PostMessageFlow<BuiltPending> {
        let message =
            self.build_message(message_id, created_at, chat::MessageStatus::Pending);
        self.transition_with(BuiltPendingData {
            message,
            moderation_reason,
        })
    }
}

#[transition]
impl PostMessageFlow<BuiltVisible> {
    pub(super) fn mark_persisted(self) -> PostMessageFlow<PersistedVisible> {
        let data = PersistedVisibleData {
            message: self.state_data.message.clone(),
        };
        self.transition_with(data)
    }
}

#[transition]
impl PostMessageFlow<BuiltPending> {
    pub(super) fn mark_persisted(self) -> PostMessageFlow<PersistedPending> {
        let data = PersistedPendingData {
            message: self.state_data.message.clone(),
            moderation_reason: self.state_data.moderation_reason.clone(),
        };
        self.transition_with(data)
    }
}

#[transition]
impl PostMessageFlow<PersistedPending> {
    pub(super) fn mark_moderation_enqueued(self) -> PostMessageFlow<ModerationEnqueued> {
        let data = ModerationEnqueuedData {
            message: self.state_data.message.clone(),
        };
        self.transition_with(data)
    }
}

#[transition]
impl PostMessageFlow<PersistedVisible> {
    pub(super) fn mark_audited(self) -> PostMessageFlow<Audited> {
        let data = AuditedData {
            message: self.state_data.message.clone(),
        };
        self.transition_with(data)
    }
}

#[transition]
impl PostMessageFlow<ModerationEnqueued> {
    pub(super) fn mark_audited(self) -> PostMessageFlow<Audited> {
        let data = AuditedData {
            message: self.state_data.message.clone(),
        };
        self.transition_with(data)
    }
}

impl<S: PostMessageStateTrait> PostMessageFlow<S> {
    pub(super) fn room_id(&self) -> &chat::RoomId {
        &self.room_id
    }

    pub(super) fn user_id(&self) -> &chat::UserId {
        &self.user_id
    }

    pub(super) fn body(&self) -> &chat::MessageBody {
        &self.body
    }
}

impl PostMessageFlow<Incoming> {
    pub(super) fn from_command(command: PostMessage) -> Self {
        let PostMessage {
            room_id,
            user_id,
            body,
            client_id,
        } = command;

        PostMessageFlow::<Incoming>::builder()
            .room_id(room_id)
            .user_id(user_id)
            .body(body)
            .maybe_client_id(client_id)
            .build()
    }
}

impl PostMessageFlow<Incoming> {
    pub(super) fn classify_room_lookup(self, room_exists: bool) -> RoomLookupOutcome {
        if room_exists {
            RoomLookupOutcome::Found(self.room_verified())
        } else {
            RoomLookupOutcome::Missing
        }
    }
}

impl PostMessageFlow<RoomVerified> {
    pub(super) fn classify_membership(self, is_member: bool) -> MembershipOutcome {
        if is_member {
            MembershipOutcome::Member(self.membership_verified())
        } else {
            MembershipOutcome::NotMember
        }
    }
}

impl PostMessageFlow<RateLimitPassed> {
    pub(super) fn build(
        self,
        message_id: chat::MessageId,
        created_at: SystemTime,
        requires_moderation: bool,
    ) -> BuiltPostMessage {
        if requires_moderation {
            let reason = ModerationReason::auto();
            BuiltPostMessage::Pending(self.build_pending(message_id, created_at, reason))
        } else {
            BuiltPostMessage::Visible(self.build_visible(message_id, created_at))
        }
    }

    fn build_message(
        &self,
        message_id: chat::MessageId,
        created_at: SystemTime,
        status: chat::MessageStatus,
    ) -> chat::Message {
        chat::Message {
            id: message_id,
            room_id: self.room_id,
            user_id: self.user_id,
            body: self.body.clone(),
            status,
            client_id: self.client_id.clone(),
            created_at,
        }
    }
}

impl PostMessageFlow<BuiltVisible> {
    pub(super) fn message(&self) -> &chat::Message {
        &self.state_data.message
    }
}

impl PostMessageFlow<BuiltPending> {
    pub(super) fn message(&self) -> &chat::Message {
        &self.state_data.message
    }
}

impl PostMessageFlow<PersistedVisible> {
    pub(super) fn message(&self) -> &chat::Message {
        &self.state_data.message
    }
}

impl PostMessageFlow<PersistedPending> {
    pub(super) fn message(&self) -> &chat::Message {
        &self.state_data.message
    }

    pub(super) fn moderation_reason(&self) -> &ModerationReason {
        &self.state_data.moderation_reason
    }
}

impl PostMessageFlow<Audited> {
    pub(super) fn into_message(self) -> chat::Message {
        self.state_data.message
    }
}

pub(super) enum BuiltPostMessage {
    Visible(PostMessageFlow<BuiltVisible>),
    Pending(PostMessageFlow<BuiltPending>),
}

pub(super) enum RoomLookupOutcome {
    Found(PostMessageFlow<RoomVerified>),
    Missing,
}

impl RoomLookupOutcome {
    pub(super) fn require_room(self) -> Result<PostMessageFlow<RoomVerified>, Error> {
        match self {
            Self::Found(found) => Ok(found),
            Self::Missing => Err(Error::RoomNotFound),
        }
    }
}

pub(super) enum MembershipOutcome {
    Member(PostMessageFlow<MembershipVerified>),
    NotMember,
}

impl MembershipOutcome {
    pub(super) fn require_member(
        self,
    ) -> Result<PostMessageFlow<MembershipVerified>, Error> {
        match self {
            Self::Member(member) => Ok(member),
            Self::NotMember => Err(Error::NotMember),
        }
    }
}

impl BuiltPostMessage {
    pub(super) async fn persist(
        self,
        repo: &dyn super::Repository,
    ) -> Result<PersistedPostMessage, Error> {
        match self {
            Self::Visible(visible) => {
                repo.insert_message(visible.message()).await?;
                Ok(PersistedPostMessage::Visible(visible.mark_persisted()))
            }
            Self::Pending(pending) => {
                repo.insert_message(pending.message()).await?;
                Ok(PersistedPostMessage::Pending(pending.mark_persisted()))
            }
        }
    }
}

pub(super) enum PersistedPostMessage {
    Visible(PostMessageFlow<PersistedVisible>),
    Pending(PostMessageFlow<PersistedPending>),
}

impl PersistedPostMessage {
    pub(super) async fn enqueue_if_pending(
        self,
        moderation: &dyn super::ModerationQueue,
    ) -> Result<ReadyForAudit, Error> {
        match self {
            Self::Visible(visible) => Ok(ReadyForAudit::Visible(visible)),
            Self::Pending(pending) => {
                moderation
                    .enqueue(&pending.message().id, pending.moderation_reason())
                    .await?;
                Ok(ReadyForAudit::Pending(pending.mark_moderation_enqueued()))
            }
        }
    }
}

pub(super) enum ReadyForAudit {
    Visible(PostMessageFlow<PersistedVisible>),
    Pending(PostMessageFlow<ModerationEnqueued>),
}

impl ReadyForAudit {
    pub(super) fn message(&self) -> &chat::Message {
        match self {
            Self::Visible(visible) => visible.message(),
            Self::Pending(pending) => &pending.state_data.message,
        }
    }

    pub(super) fn mark_audited(self) -> PostMessageFlow<Audited> {
        match self {
            Self::Visible(visible) => visible.mark_audited(),
            Self::Pending(pending) => pending.mark_audited(),
        }
    }
}

pub(super) type IncomingFlow = PostMessageFlow<Incoming>;

#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use std::sync::Mutex;

    use super::*;

    fn build_command(body: &str) -> PostMessage {
        PostMessage::builder()
            .room_id(chat::RoomId::new_v4())
            .user_id(chat::UserId::new_v4())
            .body(chat::MessageBody::try_new(body).expect("valid body"))
            .maybe_client_id(None)
            .build()
    }

    fn to_rate_limited(command: PostMessage) -> PostMessageFlow<RateLimitPassed> {
        PostMessageFlow::<Incoming>::from_command(command)
            .room_verified()
            .membership_verified()
            .rate_limit_passed()
    }

    #[test]
    fn visible_build_path_sets_visible_message_status() {
        let rate_limited = to_rate_limited(build_command("hello"));

        let built =
            rate_limited.build(chat::MessageId::new_v4(), SystemTime::UNIX_EPOCH, false);
        let BuiltPostMessage::Visible(visible) = built else {
            panic!("expected visible branch");
        };

        assert_eq!(visible.message().status, chat::MessageStatus::Visible);
    }

    #[test]
    fn pending_build_path_carries_reason_and_status() {
        let rate_limited = to_rate_limited(build_command("contains a link"));

        let built =
            rate_limited.build(chat::MessageId::new_v4(), SystemTime::UNIX_EPOCH, true);
        let BuiltPostMessage::Pending(pending) = built else {
            panic!("expected pending branch");
        };
        let persisted = pending.mark_persisted();

        assert_eq!(persisted.message().status, chat::MessageStatus::Pending);
        assert_eq!(persisted.moderation_reason().to_string(), "auto");
    }

    #[test]
    fn classify_room_lookup_rejects_missing_room() {
        let incoming = PostMessageFlow::<Incoming>::from_command(build_command("hello"));
        let result = incoming.classify_room_lookup(false).require_room();
        assert!(matches!(result, Err(Error::RoomNotFound)));
    }

    #[test]
    fn classify_membership_rejects_non_member() {
        let incoming = PostMessageFlow::<Incoming>::from_command(build_command("hello"));
        let room_verified = incoming
            .classify_room_lookup(true)
            .require_room()
            .expect("room exists");
        let result = room_verified.classify_membership(false).require_member();
        assert!(matches!(result, Err(Error::NotMember)));
    }

    struct TestRepository {
        inserted: Mutex<Vec<chat::MessageId>>,
    }

    impl TestRepository {
        fn inserted(&self) -> Vec<chat::MessageId> {
            self.inserted.lock().expect("inserted lock").clone()
        }
    }

    #[async_trait]
    impl super::super::Repository for TestRepository {
        async fn create_room(&self, _room: &chat::Room) -> super::super::Result<()> {
            unimplemented!("not used in this test")
        }

        async fn find_room(
            &self,
            _room_id: &chat::RoomId,
        ) -> super::super::Result<Option<chat::Room>> {
            unimplemented!("not used in this test")
        }

        async fn find_room_by_name(
            &self,
            _name: &chat::RoomName,
        ) -> super::super::Result<Option<chat::Room>> {
            unimplemented!("not used in this test")
        }

        async fn list_messages(
            &self,
            _room_id: &chat::RoomId,
            _limit: usize,
        ) -> super::super::Result<Vec<chat::Message>> {
            unimplemented!("not used in this test")
        }

        async fn find_message(
            &self,
            _message_id: &chat::MessageId,
        ) -> super::super::Result<Option<chat::Message>> {
            unimplemented!("not used in this test")
        }

        async fn insert_message(
            &self,
            message: &chat::Message,
        ) -> super::super::Result<()> {
            self.inserted
                .lock()
                .expect("inserted lock")
                .push(message.id);
            Ok(())
        }

        async fn add_membership(
            &self,
            _room_id: &chat::RoomId,
            _user_id: &chat::UserId,
            _role: super::super::RoomRole,
        ) -> super::super::Result<()> {
            unimplemented!("not used in this test")
        }

        async fn is_member(
            &self,
            _room_id: &chat::RoomId,
            _user_id: &chat::UserId,
        ) -> super::super::Result<bool> {
            unimplemented!("not used in this test")
        }

        async fn update_message_status(
            &self,
            _message_id: &chat::MessageId,
            _status: chat::MessageStatus,
        ) -> super::super::Result<super::super::PendingMutationResult> {
            unimplemented!("not used in this test")
        }
    }

    struct TestModerationQueue {
        enqueued: Mutex<Vec<chat::MessageId>>,
    }

    impl TestModerationQueue {
        fn enqueued(&self) -> Vec<chat::MessageId> {
            self.enqueued.lock().expect("enqueued lock").clone()
        }
    }

    #[async_trait]
    impl super::super::ModerationQueue for TestModerationQueue {
        async fn enqueue(
            &self,
            message_id: &chat::MessageId,
            _reason: &super::super::ModerationReason,
        ) -> super::super::Result<()> {
            self.enqueued
                .lock()
                .expect("enqueued lock")
                .push(*message_id);
            Ok(())
        }

        async fn list_pending(
            &self,
            _limit: usize,
        ) -> super::super::Result<Vec<super::super::ModerationItem>> {
            unimplemented!("not used in this test")
        }

        async fn complete_if_pending(
            &self,
            _message_id: &chat::MessageId,
            _reviewer_id: &chat::UserId,
            _decision: super::super::ModerationDecision,
            _reason: Option<super::super::ModerationReason>,
        ) -> super::super::Result<super::super::PendingMutationResult> {
            unimplemented!("not used in this test")
        }
    }

    #[tokio::test]
    async fn persist_visible_branch_inserts_message_and_keeps_visible_path() {
        let rate_limited = to_rate_limited(build_command("hello"));
        let built =
            rate_limited.build(chat::MessageId::new_v4(), SystemTime::UNIX_EPOCH, false);
        let repo = TestRepository {
            inserted: Mutex::new(Vec::new()),
        };

        let persisted = built.persist(&repo).await.expect("persisted");

        assert!(matches!(persisted, PersistedPostMessage::Visible(_)));
        assert_eq!(repo.inserted().len(), 1);
    }

    #[tokio::test]
    async fn pending_branch_enqueues_before_audit() {
        let rate_limited = to_rate_limited(build_command("contains a link"));
        let built =
            rate_limited.build(chat::MessageId::new_v4(), SystemTime::UNIX_EPOCH, true);
        let repo = TestRepository {
            inserted: Mutex::new(Vec::new()),
        };
        let moderation = TestModerationQueue {
            enqueued: Mutex::new(Vec::new()),
        };

        let ready = built
            .persist(&repo)
            .await
            .expect("persisted")
            .enqueue_if_pending(&moderation)
            .await
            .expect("enqueued");

        assert!(matches!(ready, ReadyForAudit::Pending(_)));
        assert_eq!(repo.inserted().len(), 1);
        assert_eq!(moderation.enqueued().len(), 1);
    }
}
