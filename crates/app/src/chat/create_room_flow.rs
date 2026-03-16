use statum::{machine, state, transition};

use super::{AuditAction, AuditKey, AuditValue, CreateRoom, RoomRole};
use domain::chat;

#[derive(Clone, Debug, PartialEq)]
pub struct RoomData {
    room: chat::Room,
}

#[state]
pub enum CreateRoomState {
    Incoming,
    RoomMaterialized(RoomData),
    RoomPersisted(RoomData),
    OwnerMembershipAdded(RoomData),
    Audited(RoomData),
}

#[machine]
pub(super) struct CreateRoomFlow<CreateRoomState> {
    room_name: chat::room::Name,
    created_by: chat::UserId,
}

impl CreateRoomFlow<Incoming> {
    pub(super) fn from_command(command: CreateRoom) -> Self {
        let CreateRoom { name, created_by } = command;
        CreateRoomFlow::<Incoming>::builder()
            .room_name(name)
            .created_by(created_by)
            .build()
    }

    pub(super) async fn create(
        self,
        service: &super::Service,
    ) -> super::Result<CreateRoomFlow<Audited>> {
        let materialized = self.materialize_room(service.ids.new_room_id());

        service.repo.create_room(materialized.room()).await?;
        let persisted = materialized.mark_room_persisted();
        service
            .repo
            .add_membership(
                &persisted.room_id(),
                &persisted.owner_id(),
                persisted.owner_role(),
            )
            .await?;
        let owner_added = persisted.mark_owner_membership_added();

        owner_added.record_audit(service).await
    }
}

#[transition]
impl CreateRoomFlow<Incoming> {
    pub(super) fn materialize_room(
        self,
        room_id: chat::room::Id,
    ) -> CreateRoomFlow<RoomMaterialized> {
        let room_name = self.room_name;
        let created_by = self.created_by;
        self.transition_with(RoomData {
            room: chat::Room {
                id: room_id,
                name: room_name,
                created_by,
            },
        })
    }
}

impl CreateRoomFlow<RoomMaterialized> {
    pub(super) fn room(&self) -> &chat::Room {
        &self.state_data.room
    }
}

#[transition]
impl CreateRoomFlow<RoomMaterialized> {
    pub(super) fn mark_room_persisted(self) -> CreateRoomFlow<RoomPersisted> {
        let data = self.state_data.clone();
        self.transition_with(data)
    }
}

impl CreateRoomFlow<RoomPersisted> {
    pub(super) fn room_id(&self) -> chat::room::Id {
        self.state_data.room.id
    }

    pub(super) fn owner_id(&self) -> chat::UserId {
        self.state_data.room.created_by
    }

    pub(super) fn owner_role(&self) -> RoomRole {
        RoomRole::Owner
    }
}

#[transition]
impl CreateRoomFlow<RoomPersisted> {
    pub(super) fn mark_owner_membership_added(
        self,
    ) -> CreateRoomFlow<OwnerMembershipAdded> {
        let data = self.state_data.clone();
        self.transition_with(data)
    }
}

impl CreateRoomFlow<OwnerMembershipAdded> {
    async fn record_audit(
        self,
        service: &super::Service,
    ) -> super::Result<CreateRoomFlow<Audited>> {
        let room_id = self.state_data.room.id;
        let created_by = self.state_data.room.created_by;

        service
            .audit
            .record(service.audit_entry(
                room_id,
                created_by,
                AuditAction::RoomCreate,
                vec![(
                    AuditKey::RoomId,
                    AuditValue::new(room_id.as_uuid().to_string()),
                )],
            ))
            .await?;
        Ok(self.mark_audited())
    }
}

#[transition]
impl CreateRoomFlow<OwnerMembershipAdded> {
    pub(super) fn mark_audited(self) -> CreateRoomFlow<Audited> {
        let data = self.state_data.clone();
        self.transition_with(data)
    }
}

impl CreateRoomFlow<Audited> {
    pub(super) fn into_room(self) -> chat::Room {
        self.state_data.room
    }
}

pub(super) type IncomingFlow = CreateRoomFlow<Incoming>;

#[cfg(test)]
mod tests {
    use super::*;

    fn command() -> CreateRoom {
        CreateRoom::builder()
            .name(chat::room::Name::Lobby)
            .created_by(chat::UserId::new_v4())
            .build()
    }

    #[test]
    fn create_room_flow_happy_path_reaches_audited() {
        let incoming = CreateRoomFlow::<Incoming>::from_command(command());
        let room_id = chat::room::Id::new_v4();

        let materialized = incoming.materialize_room(room_id);
        assert_eq!(materialized.room().id, room_id);

        let persisted = materialized.mark_room_persisted();
        assert_eq!(persisted.room_id(), room_id);
        assert_eq!(persisted.owner_role(), RoomRole::Owner);

        let owner_added = persisted.mark_owner_membership_added();
        let audited = owner_added.mark_audited();
        assert_eq!(audited.into_room().id, room_id);
    }
}
