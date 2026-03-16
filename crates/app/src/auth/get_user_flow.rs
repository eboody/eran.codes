use statum::{machine, state, transition};

use super::{AuthenticatedUser, Record};

#[derive(Clone, Debug)]
pub struct RecordData {
    record: Record,
}

#[derive(Clone, Debug)]
pub struct UserData {
    user: AuthenticatedUser,
}

#[state]
pub enum GetUserState {
    Incoming,
    RecordMissing,
    RecordFound(RecordData),
    UserMaterialized(UserData),
}

#[machine]
pub(super) struct GetUserFlow<GetUserState> {}

impl GetUserFlow<Incoming> {
    pub(super) fn new() -> Self {
        GetUserFlow::<Incoming>::builder().build()
    }

    pub(super) fn classify_lookup(self, record: Option<Record>) -> LookupOutcome {
        match record {
            Some(record) => LookupOutcome::Found(self.mark_record_found(record)),
            None => {
                let _ = self.mark_record_missing();
                LookupOutcome::Missing
            }
        }
    }
}

#[transition]
impl GetUserFlow<Incoming> {
    fn mark_record_found(self, record: Record) -> GetUserFlow<RecordFound> {
        self.transition_with(RecordData { record })
    }

    fn mark_record_missing(self) -> GetUserFlow<RecordMissing> {
        self.transition()
    }
}

#[transition]
impl GetUserFlow<RecordFound> {
    fn materialize_user(self) -> GetUserFlow<UserMaterialized> {
        let record = self.state_data.record.clone();
        let user = AuthenticatedUser::builder()
            .id(record.id)
            .username(record.username)
            .email(record.email)
            .session_hash(record.session_hash)
            .build();
        self.transition_with(UserData { user })
    }
}

impl GetUserFlow<UserMaterialized> {
    fn into_user(self) -> AuthenticatedUser {
        self.state_data.user
    }
}

pub(super) enum LookupOutcome {
    Found(GetUserFlow<RecordFound>),
    Missing,
}

impl LookupOutcome {
    pub(super) fn into_user_option(self) -> Option<AuthenticatedUser> {
        match self {
            Self::Found(found) => Some(found.materialize_user().into_user()),
            Self::Missing => None,
        }
    }
}

pub(super) type IncomingFlow = GetUserFlow<Incoming>;

#[cfg(test)]
mod tests {
    use super::*;
    use domain::user;

    fn test_record() -> Record {
        Record::builder()
            .id(user::Id::new_v4())
            .username(user::Username::try_new("person").expect("valid username"))
            .email(user::Email::try_new("person@example.com").expect("valid email"))
            .password_hash(super::super::PasswordHash::new("hash"))
            .session_hash(super::super::SessionHash::new("session"))
            .build()
    }

    #[test]
    fn missing_lookup_maps_to_none() {
        let incoming = GetUserFlow::<Incoming>::new();
        let user = incoming.classify_lookup(None).into_user_option();

        assert!(user.is_none());
    }

    #[test]
    fn found_lookup_materializes_user() {
        let incoming = GetUserFlow::<Incoming>::new();
        let user = incoming
            .classify_lookup(Some(test_record()))
            .into_user_option();

        assert!(user.is_some());
    }
}
