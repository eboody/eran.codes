use statum::{machine, state, transition};

use super::{ModerateMessage, PendingMutationResult, audit, failure, moderation};
use domain::chat;

#[derive(Clone, Debug, PartialEq)]
pub struct LoadedData {
    room_id: chat::room::Id,
    message_status: chat::message::Status,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PendingVerifiedData {
    room_id: chat::room::Id,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ResolutionData {
    room_id: chat::room::Id,
    message_status: chat::message::Status,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AuditPreparedData {
    room_id: chat::room::Id,
    metadata: Vec<(audit::Key, audit::Value)>,
}

#[state]
pub enum ModerateMessageState {
    Incoming,
    MessageLoaded(LoadedData),
    PendingVerified(PendingVerifiedData),
    Resolved(ResolutionData),
    MessageStatusApplied(ResolutionData),
    QueueCompletionApplied(ResolutionData),
    AuditPrepared(AuditPreparedData),
    Audited(AuditPreparedData),
}

#[machine]
pub(super) struct ModerateMessageFlow<ModerateMessageState> {
    message_id: chat::message::Id,
    reviewer_id: chat::UserId,
    decision: moderation::Decision,
    reason: Option<moderation::Reason>,
}

#[transition]
impl ModerateMessageFlow<Incoming> {
    pub(super) fn load_message(
        self,
        message: &chat::Message,
    ) -> ModerateMessageFlow<MessageLoaded> {
        self.transition_with(LoadedData {
            room_id: message.room_id,
            message_status: message.status,
        })
    }
}

impl ModerateMessageFlow<Incoming> {
    pub(super) fn load_lookup(
        self,
        message: Option<chat::Message>,
    ) -> Result<ModerateMessageFlow<MessageLoaded>, failure::Error> {
        self.classify_lookup(message).require_message()
    }

    pub(super) async fn moderate(
        self,
        service: &super::Service,
    ) -> Result<ModerateMessageFlow<Audited>, failure::Error> {
        let loaded = self.load_from_repo(service).await?;
        let pending = loaded.classify_pending().require_pending()?;
        let resolved = pending.resolve();
        let message_status_applied = resolved.apply_message_status(service).await?;
        let queue_completion_applied =
            message_status_applied.complete_queue(service).await?;
        let audit_prepared = queue_completion_applied.prepare_audit();

        audit_prepared.record_audit(service).await
    }

    async fn load_from_repo(
        self,
        service: &super::Service,
    ) -> Result<ModerateMessageFlow<MessageLoaded>, failure::Error> {
        let message_lookup = service.repo.find_message(&self.message_id()).await?;
        self.load_lookup(message_lookup)
    }

    pub(super) fn classify_lookup(
        self,
        message: Option<chat::Message>,
    ) -> MessageLookupOutcome {
        match message {
            Some(message) => MessageLookupOutcome::Found(self.load_message(&message)),
            None => MessageLookupOutcome::Missing,
        }
    }
}

#[transition]
impl ModerateMessageFlow<MessageLoaded> {
    fn mark_pending_verified(self) -> ModerateMessageFlow<PendingVerified> {
        let data = PendingVerifiedData {
            room_id: self.state_data.room_id,
        };
        self.transition_with(data)
    }
}

impl ModerateMessageFlow<MessageLoaded> {
    pub(super) fn classify_pending(self) -> PendingOutcome {
        if self.state_data.message_status == chat::message::Status::Pending {
            PendingOutcome::Pending(self.mark_pending_verified())
        } else {
            PendingOutcome::Conflict
        }
    }
}

#[transition]
impl ModerateMessageFlow<PendingVerified> {
    pub(super) fn resolve(self) -> ModerateMessageFlow<Resolved> {
        let message_status = match self.decision {
            moderation::Decision::Approve => chat::message::Status::Visible,
            moderation::Decision::Remove => chat::message::Status::Removed,
        };
        let data = ResolutionData {
            room_id: self.state_data.room_id,
            message_status,
        };
        self.transition_with(data)
    }
}

#[transition]
impl ModerateMessageFlow<Resolved> {
    fn mark_message_status_applied(self) -> ModerateMessageFlow<MessageStatusApplied> {
        let data = self.state_data.clone();
        self.transition_with(data)
    }
}

impl ModerateMessageFlow<Resolved> {
    pub(super) fn classify_message_status_update(
        self,
        update_result: PendingMutationResult,
    ) -> MessageStatusUpdateOutcome {
        if update_result == PendingMutationResult::Applied {
            MessageStatusUpdateOutcome::Applied(self.mark_message_status_applied())
        } else {
            MessageStatusUpdateOutcome::Conflict
        }
    }
}

#[transition]
impl ModerateMessageFlow<MessageStatusApplied> {
    fn mark_queue_completion_applied(self) -> ModerateMessageFlow<QueueCompletionApplied> {
        let data = self.state_data.clone();
        self.transition_with(data)
    }
}

impl ModerateMessageFlow<MessageStatusApplied> {
    pub(super) fn classify_queue_completion_update(
        self,
        update_result: PendingMutationResult,
    ) -> QueueCompletionUpdateOutcome {
        if update_result == PendingMutationResult::Applied {
            QueueCompletionUpdateOutcome::Applied(self.mark_queue_completion_applied())
        } else {
            QueueCompletionUpdateOutcome::Conflict
        }
    }
}

#[transition]
impl ModerateMessageFlow<QueueCompletionApplied> {
    pub(super) fn prepare_audit(self) -> ModerateMessageFlow<AuditPrepared> {
        let room_id = self.state_data.room_id;
        let metadata = vec![
            (
                audit::Key::MessageId,
                audit::Value::new(self.message_id.as_ref().to_string()),
            ),
            (
                audit::Key::Decision,
                audit::Value::new(self.decision.to_string()),
            ),
            (
                audit::Key::Reason,
                self.reason
                    .clone()
                    .map(|reason| audit::Value::new(reason.to_string()))
                    .unwrap_or_else(|| audit::Value::new("")),
            ),
        ];
        self.transition_with(AuditPreparedData { room_id, metadata })
    }
}

impl<S: ModerateMessageStateTrait> ModerateMessageFlow<S> {
    pub(super) fn message_id(&self) -> chat::message::Id {
        self.message_id
    }

    pub(super) fn reviewer_id(&self) -> chat::UserId {
        self.reviewer_id
    }

    pub(super) fn decision(&self) -> moderation::Decision {
        self.decision
    }

    pub(super) fn reason(&self) -> Option<&moderation::Reason> {
        self.reason.as_ref()
    }
}

impl ModerateMessageFlow<Resolved> {
    pub(super) fn message_status(&self) -> chat::message::Status {
        self.state_data.message_status
    }

    async fn apply_message_status(
        self,
        service: &super::Service,
    ) -> Result<ModerateMessageFlow<MessageStatusApplied>, failure::Error> {
        let message_update = service
            .repo
            .update_message_status(&self.message_id(), self.message_status())
            .await?;
        self.classify_message_status_update(message_update)
            .require_applied()
    }
}

impl ModerateMessageFlow<MessageStatusApplied> {
    async fn complete_queue(
        self,
        service: &super::Service,
    ) -> Result<ModerateMessageFlow<QueueCompletionApplied>, failure::Error> {
        let queue_update = service
            .moderation
            .complete_if_pending(
                &self.message_id(),
                &self.reviewer_id(),
                self.decision(),
                self.reason().cloned(),
            )
            .await?;
        self.classify_queue_completion_update(queue_update)
            .require_applied()
    }
}

impl ModerateMessageFlow<AuditPrepared> {
    pub(super) fn room_id(&self) -> chat::room::Id {
        self.state_data.room_id
    }

    pub(super) fn audit_metadata(&self) -> Vec<(audit::Key, audit::Value)> {
        self.state_data.metadata.clone()
    }

    async fn record_audit(
        self,
        service: &super::Service,
    ) -> Result<ModerateMessageFlow<Audited>, failure::Error> {
        service
            .audit
            .record(service.audit_entry(
                self.room_id(),
                self.reviewer_id(),
                super::audit::Action::MessageModerate,
                self.audit_metadata(),
            ))
            .await?;
        Ok(self.mark_audited())
    }
}

#[transition]
impl ModerateMessageFlow<AuditPrepared> {
    pub(super) fn mark_audited(self) -> ModerateMessageFlow<Audited> {
        let data = self.state_data.clone();
        self.transition_with(data)
    }
}

impl ModerateMessageFlow<Incoming> {
    pub(super) fn from_command(command: ModerateMessage) -> Self {
        let ModerateMessage {
            message_id,
            reviewer_id,
            decision,
            reason,
        } = command;

        ModerateMessageFlow::<Incoming>::builder()
            .message_id(message_id)
            .reviewer_id(reviewer_id)
            .decision(decision)
            .reason(reason)
            .build()
    }
}

pub(super) enum MessageLookupOutcome {
    Found(ModerateMessageFlow<MessageLoaded>),
    Missing,
}

impl MessageLookupOutcome {
    pub(super) fn require_message(
        self,
    ) -> Result<ModerateMessageFlow<MessageLoaded>, failure::Error> {
        match self {
            Self::Found(found) => Ok(found),
            Self::Missing => Err(failure::Error::MessageNotFound),
        }
    }
}

pub(super) enum PendingOutcome {
    Pending(ModerateMessageFlow<PendingVerified>),
    Conflict,
}

impl PendingOutcome {
    pub(super) fn require_pending(
        self,
    ) -> Result<ModerateMessageFlow<PendingVerified>, failure::Error> {
        match self {
            Self::Pending(pending) => Ok(pending),
            Self::Conflict => Err(failure::Error::ModerationStateConflict),
        }
    }
}

pub(super) enum MessageStatusUpdateOutcome {
    Applied(ModerateMessageFlow<MessageStatusApplied>),
    Conflict,
}

impl MessageStatusUpdateOutcome {
    pub(super) fn require_applied(
        self,
    ) -> Result<ModerateMessageFlow<MessageStatusApplied>, failure::Error> {
        match self {
            Self::Applied(applied) => Ok(applied),
            Self::Conflict => Err(failure::Error::ModerationStateConflict),
        }
    }
}

pub(super) enum QueueCompletionUpdateOutcome {
    Applied(ModerateMessageFlow<QueueCompletionApplied>),
    Conflict,
}

impl QueueCompletionUpdateOutcome {
    pub(super) fn require_applied(
        self,
    ) -> Result<ModerateMessageFlow<QueueCompletionApplied>, failure::Error> {
        match self {
            Self::Applied(applied) => Ok(applied),
            Self::Conflict => Err(failure::Error::ModerationStateConflict),
        }
    }
}

pub(super) type IncomingFlow = ModerateMessageFlow<Incoming>;

#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use std::sync::{Arc, Mutex};

    use super::*;

    fn build_message(status: chat::message::Status) -> chat::Message {
        chat::Message::builder()
            .id(chat::message::Id::new_v4())
            .room_id(chat::room::Id::new_v4())
            .user_id(chat::UserId::new_v4())
            .body(chat::message::Body::try_new("hello").expect("valid body"))
            .status(status)
            .maybe_client_id(None)
            .created_at(std::time::SystemTime::UNIX_EPOCH)
            .build()
    }

    fn build_command(
        message_id: chat::message::Id,
        decision: moderation::Decision,
    ) -> ModerateMessage {
        ModerateMessage::builder()
            .message_id(message_id)
            .reviewer_id(chat::UserId::new_v4())
            .decision(decision)
            .maybe_reason(None)
            .build()
    }

    struct TestRepository {
        message: Mutex<Option<chat::Message>>,
        updated: Mutex<Vec<(chat::message::Id, chat::message::Status)>>,
        update_result: PendingMutationResult,
    }

    impl TestRepository {
        fn updated(&self) -> Vec<(chat::message::Id, chat::message::Status)> {
            self.updated.lock().expect("updated lock").clone()
        }
    }

    #[async_trait]
    impl super::super::Repository for TestRepository {
        async fn create_room(&self, _room: &chat::Room) -> super::super::Result<()> {
            unimplemented!("not used in this test")
        }

        async fn find_room(
            &self,
            _room_id: &chat::room::Id,
        ) -> super::super::Result<Option<chat::Room>> {
            unimplemented!("not used in this test")
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
            Ok(self.message.lock().expect("message lock").clone())
        }

        async fn insert_message(
            &self,
            _message: &chat::Message,
        ) -> super::super::Result<()> {
            unimplemented!("not used in this test")
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
            unimplemented!("not used in this test")
        }

        async fn update_message_status(
            &self,
            message_id: &chat::message::Id,
            status: chat::message::Status,
        ) -> super::super::Result<PendingMutationResult> {
            self.updated
                .lock()
                .expect("updated lock")
                .push((*message_id, status));
            Ok(self.update_result)
        }
    }

    #[derive(Default)]
    struct TestModerationQueue {
        completions: Mutex<Vec<(chat::message::Id, chat::UserId, moderation::Decision)>>,
    }

    impl TestModerationQueue {
        fn completions(
            &self,
        ) -> Vec<(chat::message::Id, chat::UserId, moderation::Decision)> {
            self.completions.lock().expect("completions lock").clone()
        }
    }

    #[async_trait]
    impl super::super::moderation::Queue for TestModerationQueue {
        async fn enqueue(
            &self,
            _message_id: &chat::message::Id,
            _reason: &super::super::moderation::Reason,
        ) -> super::super::Result<()> {
            unimplemented!("not used in this test")
        }

        async fn list_pending(
            &self,
            _limit: usize,
        ) -> super::super::Result<Vec<super::super::moderation::Item>> {
            unimplemented!("not used in this test")
        }

        async fn complete_if_pending(
            &self,
            message_id: &chat::message::Id,
            reviewer_id: &chat::UserId,
            decision: moderation::Decision,
            _reason: Option<super::super::moderation::Reason>,
        ) -> super::super::Result<PendingMutationResult> {
            self.completions.lock().expect("completions lock").push((
                *message_id,
                *reviewer_id,
                decision,
            ));
            Ok(PendingMutationResult::Applied)
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
        fn now(&self) -> std::time::SystemTime {
            std::time::SystemTime::UNIX_EPOCH
        }
    }

    struct NoopRateLimiter;

    #[async_trait]
    impl super::super::RateLimiter for NoopRateLimiter {
        async fn check(
            &self,
            _room_id: &chat::room::Id,
            _user_id: &chat::UserId,
        ) -> super::super::Result<()> {
            Ok(())
        }
    }

    struct FixedIds;

    impl super::super::IdGenerator for FixedIds {
        fn new_room_id(&self) -> chat::room::Id {
            chat::room::Id::new_v4()
        }

        fn new_message_id(&self) -> chat::message::Id {
            chat::message::Id::new_v4()
        }
    }

    fn test_service(
        repo: Arc<TestRepository>,
        moderation: Arc<TestModerationQueue>,
        audit: Arc<TestAuditLog>,
    ) -> super::super::Service {
        super::super::Service::builder()
            .with_repo(repo)
            .with_moderation_queue(moderation)
            .with_rate_limiter(Arc::new(NoopRateLimiter))
            .with_audit_log(audit)
            .with_clock(Arc::new(FixedClock))
            .with_id_generator(Arc::new(FixedIds))
            .build()
    }

    #[test]
    fn ensure_pending_rejects_non_pending_messages() {
        let message = build_message(chat::message::Status::Visible);
        let command = build_command(message.id, moderation::Decision::Approve);

        let loaded = ModerateMessageFlow::<Incoming>::from_command(command)
            .load_lookup(Some(message))
            .expect("loaded");

        let result = loaded.classify_pending().require_pending();
        assert!(matches!(
            result,
            Err(failure::Error::ModerationStateConflict)
        ));
    }

    #[test]
    fn resolve_maps_decision_to_message_status() {
        let message = build_message(chat::message::Status::Pending);
        let command = build_command(message.id, moderation::Decision::Remove);

        let pending = ModerateMessageFlow::<Incoming>::from_command(command)
            .load_lookup(Some(message))
            .expect("loaded")
            .classify_pending()
            .require_pending()
            .expect("pending");
        let resolved = pending.resolve();

        assert_eq!(resolved.message_status(), chat::message::Status::Removed);
        assert_eq!(resolved.decision(), moderation::Decision::Remove);
    }

    #[test]
    fn applied_markers_require_applied_mutation_result() {
        let message = build_message(chat::message::Status::Pending);
        let command = build_command(message.id, moderation::Decision::Approve);
        let pending = ModerateMessageFlow::<Incoming>::from_command(command)
            .load_lookup(Some(message))
            .expect("loaded")
            .classify_pending()
            .require_pending()
            .expect("pending");
        let resolved = pending.resolve();

        let status_err = resolved
            .classify_message_status_update(PendingMutationResult::NotPendingOrMissing)
            .require_applied();
        assert!(matches!(
            status_err,
            Err(failure::Error::ModerationStateConflict)
        ));
    }

    #[test]
    fn load_lookup_rejects_missing_message() {
        let command =
            build_command(chat::message::Id::new_v4(), moderation::Decision::Approve);
        let incoming = ModerateMessageFlow::<Incoming>::from_command(command);

        let result = incoming.load_lookup(None);
        assert!(matches!(result, Err(failure::Error::MessageNotFound)));
    }

    #[test]
    fn prepare_audit_contains_message_id_metadata() {
        let message = build_message(chat::message::Status::Pending);
        let command = build_command(message.id, moderation::Decision::Approve);
        let prepared = ModerateMessageFlow::<Incoming>::from_command(command)
            .load_lookup(Some(message))
            .expect("loaded")
            .classify_pending()
            .require_pending()
            .expect("pending")
            .resolve()
            .classify_message_status_update(PendingMutationResult::Applied)
            .require_applied()
            .expect("status applied")
            .classify_queue_completion_update(PendingMutationResult::Applied)
            .require_applied()
            .expect("queue applied")
            .prepare_audit();

        assert!(
            prepared
                .audit_metadata()
                .iter()
                .any(|(key, _)| *key == audit::Key::MessageId)
        );
    }

    #[tokio::test]
    async fn moderate_runs_full_pending_path_through_audit() {
        let message = build_message(chat::message::Status::Pending);
        let repo = Arc::new(TestRepository {
            message: Mutex::new(Some(message.clone())),
            updated: Mutex::new(Vec::new()),
            update_result: PendingMutationResult::Applied,
        });
        let moderation = Arc::new(TestModerationQueue::default());
        let audit = Arc::new(TestAuditLog::default());
        let service = test_service(repo.clone(), moderation.clone(), audit.clone());

        let result = ModerateMessageFlow::<Incoming>::from_command(build_command(
            message.id,
            moderation::Decision::Approve,
        ))
        .moderate(&service)
        .await;

        assert!(result.is_ok());
        assert_eq!(
            repo.updated(),
            vec![(message.id, chat::message::Status::Visible)]
        );
        assert_eq!(moderation.completions().len(), 1);
        assert_eq!(audit.entries().len(), 1);
        assert_eq!(
            audit.entries()[0].action,
            super::super::audit::Action::MessageModerate
        );
    }
}
