use statum::{machine, state, transition};

use super::{
    AuditKey, AuditValue, Error, ModerateMessage, ModerationDecision, ModerationReason,
    PendingMutationResult,
};
use domain::chat;

#[derive(Clone, Debug, PartialEq)]
pub struct LoadedData {
    room_id: chat::RoomId,
    message_status: chat::MessageStatus,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PendingVerifiedData {
    room_id: chat::RoomId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ResolutionData {
    room_id: chat::RoomId,
    message_status: chat::MessageStatus,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AuditPreparedData {
    room_id: chat::RoomId,
    metadata: Vec<(AuditKey, AuditValue)>,
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
    message_id: chat::MessageId,
    reviewer_id: chat::UserId,
    decision: ModerationDecision,
    reason: Option<ModerationReason>,
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
    ) -> Result<ModerateMessageFlow<MessageLoaded>, Error> {
        self.classify_lookup(message).require_message()
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
        if self.state_data.message_status == chat::MessageStatus::Pending {
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
            ModerationDecision::Approve => chat::MessageStatus::Visible,
            ModerationDecision::Remove => chat::MessageStatus::Removed,
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
                AuditKey::MessageId,
                AuditValue::new(self.message_id.as_uuid().to_string()),
            ),
            (
                AuditKey::Decision,
                AuditValue::new(self.decision.to_string()),
            ),
            (
                AuditKey::Reason,
                self.reason
                    .clone()
                    .map(|reason| AuditValue::new(reason.to_string()))
                    .unwrap_or_else(|| AuditValue::new("")),
            ),
        ];
        self.transition_with(AuditPreparedData { room_id, metadata })
    }
}

impl<S: ModerateMessageStateTrait> ModerateMessageFlow<S> {
    pub(super) fn message_id(&self) -> chat::MessageId {
        self.message_id
    }

    pub(super) fn reviewer_id(&self) -> chat::UserId {
        self.reviewer_id
    }

    pub(super) fn decision(&self) -> ModerationDecision {
        self.decision
    }

    pub(super) fn reason(&self) -> Option<&ModerationReason> {
        self.reason.as_ref()
    }
}

impl ModerateMessageFlow<Resolved> {
    pub(super) fn message_status(&self) -> chat::MessageStatus {
        self.state_data.message_status
    }
}

impl ModerateMessageFlow<AuditPrepared> {
    pub(super) fn room_id(&self) -> chat::RoomId {
        self.state_data.room_id
    }

    pub(super) fn audit_metadata(&self) -> Vec<(AuditKey, AuditValue)> {
        self.state_data.metadata.clone()
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
            .maybe_reason(reason)
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
    ) -> Result<ModerateMessageFlow<MessageLoaded>, Error> {
        match self {
            Self::Found(found) => Ok(found),
            Self::Missing => Err(Error::MessageNotFound),
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
    ) -> Result<ModerateMessageFlow<PendingVerified>, Error> {
        match self {
            Self::Pending(pending) => Ok(pending),
            Self::Conflict => Err(Error::ModerationStateConflict),
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
    ) -> Result<ModerateMessageFlow<MessageStatusApplied>, Error> {
        match self {
            Self::Applied(applied) => Ok(applied),
            Self::Conflict => Err(Error::ModerationStateConflict),
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
    ) -> Result<ModerateMessageFlow<QueueCompletionApplied>, Error> {
        match self {
            Self::Applied(applied) => Ok(applied),
            Self::Conflict => Err(Error::ModerationStateConflict),
        }
    }
}

pub(super) type IncomingFlow = ModerateMessageFlow<Incoming>;

#[cfg(test)]
mod tests {
    use super::*;

    fn build_message(status: chat::MessageStatus) -> chat::Message {
        chat::Message::builder()
            .id(chat::MessageId::new_v4())
            .room_id(chat::RoomId::new_v4())
            .user_id(chat::UserId::new_v4())
            .body(chat::MessageBody::try_new("hello").expect("valid body"))
            .status(status)
            .maybe_client_id(None)
            .created_at(std::time::SystemTime::UNIX_EPOCH)
            .build()
    }

    fn build_command(
        message_id: chat::MessageId,
        decision: ModerationDecision,
    ) -> ModerateMessage {
        ModerateMessage::builder()
            .message_id(message_id)
            .reviewer_id(chat::UserId::new_v4())
            .decision(decision)
            .maybe_reason(None)
            .build()
    }

    #[test]
    fn ensure_pending_rejects_non_pending_messages() {
        let message = build_message(chat::MessageStatus::Visible);
        let command = build_command(message.id, ModerationDecision::Approve);

        let loaded = ModerateMessageFlow::<Incoming>::from_command(command)
            .load_lookup(Some(message))
            .expect("loaded");

        let result = loaded.classify_pending().require_pending();
        assert!(matches!(result, Err(Error::ModerationStateConflict)));
    }

    #[test]
    fn resolve_maps_decision_to_message_status() {
        let message = build_message(chat::MessageStatus::Pending);
        let command = build_command(message.id, ModerationDecision::Remove);

        let pending = ModerateMessageFlow::<Incoming>::from_command(command)
            .load_lookup(Some(message))
            .expect("loaded")
            .classify_pending()
            .require_pending()
            .expect("pending");
        let resolved = pending.resolve();

        assert_eq!(resolved.message_status(), chat::MessageStatus::Removed);
        assert_eq!(resolved.decision(), ModerationDecision::Remove);
    }

    #[test]
    fn applied_markers_require_applied_mutation_result() {
        let message = build_message(chat::MessageStatus::Pending);
        let command = build_command(message.id, ModerationDecision::Approve);
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
        assert!(matches!(status_err, Err(Error::ModerationStateConflict)));
    }

    #[test]
    fn load_lookup_rejects_missing_message() {
        let command = build_command(chat::MessageId::new_v4(), ModerationDecision::Approve);
        let incoming = ModerateMessageFlow::<Incoming>::from_command(command);

        let result = incoming.load_lookup(None);
        assert!(matches!(result, Err(Error::MessageNotFound)));
    }

    #[test]
    fn prepare_audit_contains_message_id_metadata() {
        let message = build_message(chat::MessageStatus::Pending);
        let command = build_command(message.id, ModerationDecision::Approve);
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
                .any(|(key, _)| *key == AuditKey::MessageId)
        );
    }
}
