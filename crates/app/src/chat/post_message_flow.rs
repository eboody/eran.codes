use std::time::SystemTime;

use statum::{machine, state, transition};

use super::{PostMessage, audit, failure, moderation};
use domain::chat;

#[derive(Clone, Debug, PartialEq)]
pub struct MessageData {
    message: chat::Message,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PendingMessageData {
    message: chat::Message,
    moderation_reason: moderation::Reason,
}

#[state]
pub enum PostMessageState {
    Incoming,
    RoomVerified,
    MembershipVerified,
    RateLimitPassed,
    BuiltVisible(MessageData),
    BuiltPending(PendingMessageData),
    PersistedVisible(MessageData),
    PersistedPending(PendingMessageData),
    ModerationEnqueued(MessageData),
    Audited(MessageData),
}

#[machine]
pub(super) struct PostMessageFlow<PostMessageState> {
    room_id: chat::room::Id,
    user_id: chat::UserId,
    body: chat::message::Body,
    client_id: Option<chat::Client>,
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
        message_id: chat::message::Id,
        created_at: SystemTime,
    ) -> PostMessageFlow<BuiltVisible> {
        let message =
            self.build_message(message_id, created_at, chat::message::Status::Visible);
        self.transition_with(MessageData { message })
    }

    pub(super) fn build_pending(
        self,
        message_id: chat::message::Id,
        created_at: SystemTime,
        moderation_reason: moderation::Reason,
    ) -> PostMessageFlow<BuiltPending> {
        let message =
            self.build_message(message_id, created_at, chat::message::Status::Pending);
        self.transition_with(PendingMessageData {
            message,
            moderation_reason,
        })
    }
}

#[transition]
impl PostMessageFlow<BuiltVisible> {
    pub(super) fn mark_persisted(self) -> PostMessageFlow<PersistedVisible> {
        let data = self.state_data.clone();
        self.transition_with(data)
    }
}

#[transition]
impl PostMessageFlow<BuiltPending> {
    pub(super) fn mark_persisted(self) -> PostMessageFlow<PersistedPending> {
        let data = self.state_data.clone();
        self.transition_with(data)
    }
}

#[transition]
impl PostMessageFlow<PersistedPending> {
    pub(super) fn mark_moderation_enqueued(self) -> PostMessageFlow<ModerationEnqueued> {
        let data = MessageData {
            message: self.state_data.message.clone(),
        };
        self.transition_with(data)
    }
}

#[transition]
impl PostMessageFlow<PersistedVisible> {
    pub(super) fn mark_audited(self) -> PostMessageFlow<Audited> {
        let data = self.state_data.clone();
        self.transition_with(data)
    }
}

#[transition]
impl PostMessageFlow<ModerationEnqueued> {
    pub(super) fn mark_audited(self) -> PostMessageFlow<Audited> {
        let data = self.state_data.clone();
        self.transition_with(data)
    }
}

impl<S: PostMessageStateTrait> PostMessageFlow<S> {
    pub(super) fn room_id(&self) -> &chat::room::Id {
        &self.room_id
    }

    pub(super) fn user_id(&self) -> &chat::UserId {
        &self.user_id
    }

    pub(super) fn body(&self) -> &chat::message::Body {
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
            .client_id(client_id)
            .build()
    }

    pub(super) async fn post(
        self,
        service: &super::Service,
    ) -> Result<PostMessageFlow<Audited>, failure::Error> {
        let room_verified = self.verify_room(service).await?;
        let membership_verified = room_verified.verify_membership(service).await?;
        let rate_limited = membership_verified.check_rate_limit(service).await?;
        let ready_for_audit = rate_limited
            .build_for_delivery(service)
            .persist(service.repo.as_ref())
            .await?
            .enqueue_if_pending(service.moderation.as_ref())
            .await?;

        ready_for_audit.record_audit(service).await
    }

    async fn verify_room(
        self,
        service: &super::Service,
    ) -> Result<PostMessageFlow<RoomVerified>, failure::Error> {
        let room_exists = service.repo.find_room(self.room_id()).await?.is_some();
        self.classify_room_lookup(room_exists).require_room()
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

    async fn verify_membership(
        self,
        service: &super::Service,
    ) -> Result<PostMessageFlow<MembershipVerified>, failure::Error> {
        let is_member = service
            .repo
            .is_member(self.room_id(), self.user_id())
            .await?;
        self.classify_membership(is_member).require_member()
    }
}

impl PostMessageFlow<MembershipVerified> {
    async fn check_rate_limit(
        self,
        service: &super::Service,
    ) -> Result<PostMessageFlow<RateLimitPassed>, failure::Error> {
        service
            .rate_limiter
            .check(self.room_id(), self.user_id())
            .await?;
        Ok(self.rate_limit_passed())
    }
}

impl PostMessageFlow<RateLimitPassed> {
    pub(super) fn build(
        self,
        message_id: chat::message::Id,
        created_at: SystemTime,
        requires_moderation: bool,
    ) -> BuiltPostMessage {
        if requires_moderation {
            let reason = moderation::Reason::auto();
            BuiltPostMessage::Pending(self.build_pending(message_id, created_at, reason))
        } else {
            BuiltPostMessage::Visible(self.build_visible(message_id, created_at))
        }
    }

    fn build_for_delivery(self, service: &super::Service) -> BuiltPostMessage {
        let requires_moderation = requires_moderation(self.body());
        self.build(
            service.ids.new_message_id(),
            service.clock.now(),
            requires_moderation,
        )
    }

    fn build_message(
        &self,
        message_id: chat::message::Id,
        created_at: SystemTime,
        status: chat::message::Status,
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
    pub(super) fn require_room(
        self,
    ) -> Result<PostMessageFlow<RoomVerified>, failure::Error> {
        match self {
            Self::Found(found) => Ok(found),
            Self::Missing => Err(failure::Error::RoomNotFound),
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
    ) -> Result<PostMessageFlow<MembershipVerified>, failure::Error> {
        match self {
            Self::Member(member) => Ok(member),
            Self::NotMember => Err(failure::Error::NotMember),
        }
    }
}

impl BuiltPostMessage {
    pub(super) async fn persist(
        self,
        repo: &dyn super::Repository,
    ) -> Result<PersistedPostMessage, failure::Error> {
        match self {
            Self::Visible(visible) => {
                repo.insert_message(&visible.state_data.message).await?;
                Ok(PersistedPostMessage::Visible(visible.mark_persisted()))
            }
            Self::Pending(pending) => {
                repo.insert_message(&pending.state_data.message).await?;
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
        moderation: &dyn super::moderation::Queue,
    ) -> Result<ReadyForAudit, failure::Error> {
        match self {
            Self::Visible(visible) => Ok(ReadyForAudit::Visible(visible)),
            Self::Pending(pending) => {
                moderation
                    .enqueue(
                        &pending.state_data.message.id,
                        &pending.state_data.moderation_reason,
                    )
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
            Self::Visible(visible) => &visible.state_data.message,
            Self::Pending(pending) => &pending.state_data.message,
        }
    }

    pub(super) fn mark_audited(self) -> PostMessageFlow<Audited> {
        match self {
            Self::Visible(visible) => visible.mark_audited(),
            Self::Pending(pending) => pending.mark_audited(),
        }
    }

    async fn record_audit(
        self,
        service: &super::Service,
    ) -> Result<PostMessageFlow<Audited>, failure::Error> {
        let (room_id, user_id, message_id, status) = {
            let message = self.message();
            (message.room_id, message.user_id, message.id, message.status)
        };
        service
            .audit
            .record(service.audit_entry(
                room_id,
                user_id,
                audit::Action::MessagePost,
                vec![
                    (
                        audit::Key::MessageId,
                        audit::Value::new(message_id.as_ref().to_string()),
                    ),
                    (audit::Key::Status, audit::Value::new(status.to_string())),
                ],
            ))
            .await?;
        Ok(self.mark_audited())
    }
}

pub(super) type IncomingFlow = PostMessageFlow<Incoming>;

fn requires_moderation(body: &chat::message::Body) -> bool {
    let value = body.to_string();
    value.len() > 300 || URL_PREFIXES.iter().any(|prefix| value.contains(prefix))
}

const URL_PREFIXES: &[&str] = &["http://", "https://"];

#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use std::sync::{Arc, Mutex};

    use super::*;

    fn build_command(body: &str) -> PostMessage {
        PostMessage::builder()
            .room_id(chat::room::Id::new_v4())
            .user_id(chat::UserId::new_v4())
            .body(chat::message::Body::try_new(body).expect("valid body"))
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
            rate_limited.build(chat::message::Id::new_v4(), SystemTime::UNIX_EPOCH, false);
        let BuiltPostMessage::Visible(visible) = built else {
            panic!("expected visible branch");
        };

        assert_eq!(
            visible.state_data.message.status,
            chat::message::Status::Visible
        );
    }

    #[test]
    fn pending_build_path_carries_reason_and_status() {
        let rate_limited = to_rate_limited(build_command("contains a link"));

        let built =
            rate_limited.build(chat::message::Id::new_v4(), SystemTime::UNIX_EPOCH, true);
        let BuiltPostMessage::Pending(pending) = built else {
            panic!("expected pending branch");
        };
        let persisted = pending.mark_persisted();

        assert_eq!(
            persisted.state_data.message.status,
            chat::message::Status::Pending
        );
        assert_eq!(persisted.state_data.moderation_reason.to_string(), "auto");
    }

    #[test]
    fn classify_room_lookup_rejects_missing_room() {
        let incoming = PostMessageFlow::<Incoming>::from_command(build_command("hello"));
        let result = incoming.classify_room_lookup(false).require_room();
        assert!(matches!(result, Err(failure::Error::RoomNotFound)));
    }

    #[test]
    fn classify_membership_rejects_non_member() {
        let incoming = PostMessageFlow::<Incoming>::from_command(build_command("hello"));
        let room_verified = incoming
            .classify_room_lookup(true)
            .require_room()
            .expect("room exists");
        let result = room_verified.classify_membership(false).require_member();
        assert!(matches!(result, Err(failure::Error::NotMember)));
    }

    struct TestRepository {
        room_exists: bool,
        is_member: bool,
        inserted: Mutex<Vec<chat::message::Id>>,
    }

    impl TestRepository {
        fn available() -> Self {
            Self {
                room_exists: true,
                is_member: true,
                inserted: Mutex::new(Vec::new()),
            }
        }

        fn inserted(&self) -> Vec<chat::message::Id> {
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
            room_id: &chat::room::Id,
        ) -> super::super::Result<Option<chat::Room>> {
            Ok(self.room_exists.then_some(chat::Room {
                id: *room_id,
                name: chat::room::Name::Lobby,
                created_by: chat::UserId::new_v4(),
            }))
        }

        async fn find_room_by_name(
            &self,
            _name: &chat::room::Name,
        ) -> super::super::Result<Option<chat::Room>> {
            unimplemented!("not used in this test")
        }

        async fn list_messages(
            &self,
            _room_id: &chat::room::Id,
            _limit: usize,
        ) -> super::super::Result<Vec<chat::Message>> {
            unimplemented!("not used in this test")
        }

        async fn find_message(
            &self,
            _message_id: &chat::message::Id,
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
            _room_id: &chat::room::Id,
            _user_id: &chat::UserId,
            _role: super::super::RoomRole,
        ) -> super::super::Result<()> {
            unimplemented!("not used in this test")
        }

        async fn is_member(
            &self,
            _room_id: &chat::room::Id,
            _user_id: &chat::UserId,
        ) -> super::super::Result<bool> {
            Ok(self.is_member)
        }

        async fn update_message_status(
            &self,
            _message_id: &chat::message::Id,
            _status: chat::message::Status,
        ) -> super::super::Result<super::super::PendingMutationResult> {
            unimplemented!("not used in this test")
        }
    }

    struct TestModerationQueue {
        enqueued: Mutex<Vec<chat::message::Id>>,
    }

    impl TestModerationQueue {
        fn enqueued(&self) -> Vec<chat::message::Id> {
            self.enqueued.lock().expect("enqueued lock").clone()
        }
    }

    #[async_trait]
    impl super::super::moderation::Queue for TestModerationQueue {
        async fn enqueue(
            &self,
            message_id: &chat::message::Id,
            _reason: &super::super::moderation::Reason,
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
        ) -> super::super::Result<Vec<super::super::moderation::Item>> {
            unimplemented!("not used in this test")
        }

        async fn complete_if_pending(
            &self,
            _message_id: &chat::message::Id,
            _reviewer_id: &chat::UserId,
            _decision: super::super::moderation::Decision,
            _reason: Option<super::super::moderation::Reason>,
        ) -> super::super::Result<super::super::PendingMutationResult> {
            unimplemented!("not used in this test")
        }
    }

    #[derive(Default)]
    struct TestRateLimiter {
        checks: Mutex<Vec<(chat::room::Id, chat::UserId)>>,
    }

    impl TestRateLimiter {
        fn checks(&self) -> Vec<(chat::room::Id, chat::UserId)> {
            self.checks.lock().expect("checks lock").clone()
        }
    }

    #[async_trait]
    impl super::super::RateLimiter for TestRateLimiter {
        async fn check(
            &self,
            room_id: &chat::room::Id,
            user_id: &chat::UserId,
        ) -> super::super::Result<()> {
            self.checks
                .lock()
                .expect("checks lock")
                .push((*room_id, *user_id));
            Ok(())
        }
    }

    #[derive(Default)]
    struct TestAuditLog {
        entries: Mutex<Vec<super::super::audit::Entry>>,
    }

    impl TestAuditLog {
        fn entries(&self) -> Vec<super::super::audit::Entry> {
            self.entries.lock().expect("entries lock").clone()
        }
    }

    #[async_trait]
    impl super::super::audit::Log for TestAuditLog {
        async fn record(
            &self,
            entry: super::super::audit::Entry,
        ) -> super::super::Result<()> {
            self.entries.lock().expect("entries lock").push(entry);
            Ok(())
        }
    }

    struct FixedClock;

    impl super::super::Clock for FixedClock {
        fn now(&self) -> SystemTime {
            SystemTime::UNIX_EPOCH
        }
    }

    struct FixedIds {
        message_id: chat::message::Id,
    }

    impl super::super::IdGenerator for FixedIds {
        fn new_room_id(&self) -> chat::room::Id {
            chat::room::Id::new_v4()
        }

        fn new_message_id(&self) -> chat::message::Id {
            self.message_id
        }
    }

    fn test_service(
        repo: Arc<TestRepository>,
        moderation: Arc<TestModerationQueue>,
        rate_limiter: Arc<TestRateLimiter>,
        audit: Arc<TestAuditLog>,
        ids: Arc<FixedIds>,
    ) -> super::super::Service {
        super::super::Service::builder()
            .with_repo(repo)
            .with_moderation_queue(moderation)
            .with_rate_limiter(rate_limiter)
            .with_audit_log(audit)
            .with_clock(Arc::new(FixedClock))
            .with_id_generator(ids)
            .build()
    }

    #[tokio::test]
    async fn persist_visible_branch_inserts_message_and_keeps_visible_path() {
        let rate_limited = to_rate_limited(build_command("hello"));
        let built =
            rate_limited.build(chat::message::Id::new_v4(), SystemTime::UNIX_EPOCH, false);
        let repo = TestRepository::available();

        let persisted = built.persist(&repo).await.expect("persisted");

        assert!(matches!(persisted, PersistedPostMessage::Visible(_)));
        assert_eq!(repo.inserted().len(), 1);
    }

    #[tokio::test]
    async fn pending_branch_enqueues_before_audit() {
        let rate_limited = to_rate_limited(build_command("contains a link"));
        let built =
            rate_limited.build(chat::message::Id::new_v4(), SystemTime::UNIX_EPOCH, true);
        let repo = TestRepository::available();
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

    #[tokio::test]
    async fn post_runs_visible_path_through_audit() {
        let repo = Arc::new(TestRepository::available());
        let moderation = Arc::new(TestModerationQueue {
            enqueued: Mutex::new(Vec::new()),
        });
        let rate_limiter = Arc::new(TestRateLimiter::default());
        let audit = Arc::new(TestAuditLog::default());
        let message_id = chat::message::Id::new_v4();
        let service = test_service(
            repo.clone(),
            moderation.clone(),
            rate_limiter.clone(),
            audit.clone(),
            Arc::new(FixedIds { message_id }),
        );

        let audited = PostMessageFlow::<Incoming>::from_command(build_command("hello"))
            .post(&service)
            .await
            .expect("posted");

        assert_eq!(audited.into_message().id, message_id);
        assert_eq!(repo.inserted(), vec![message_id]);
        assert_eq!(rate_limiter.checks().len(), 1);
        assert!(moderation.enqueued().is_empty());
        assert_eq!(audit.entries().len(), 1);
        assert_eq!(audit.entries()[0].action, audit::Action::MessagePost);
    }

    #[tokio::test]
    async fn post_auto_moderates_links_before_audit() {
        let repo = Arc::new(TestRepository::available());
        let moderation = Arc::new(TestModerationQueue {
            enqueued: Mutex::new(Vec::new()),
        });
        let rate_limiter = Arc::new(TestRateLimiter::default());
        let audit = Arc::new(TestAuditLog::default());
        let message_id = chat::message::Id::new_v4();
        let service = test_service(
            repo.clone(),
            moderation.clone(),
            rate_limiter,
            audit,
            Arc::new(FixedIds { message_id }),
        );

        let audited = PostMessageFlow::<Incoming>::from_command(build_command(
            "visit https://example.com",
        ))
        .post(&service)
        .await
        .expect("posted");

        assert_eq!(
            audited.into_message().status,
            chat::message::Status::Pending
        );
        assert_eq!(repo.inserted(), vec![message_id]);
        assert_eq!(moderation.enqueued(), vec![message_id]);
    }
}
