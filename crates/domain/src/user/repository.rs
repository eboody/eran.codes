use crate::user;

pub trait Repository: Send + Sync {
    fn get_by_id(
        &self,
        id: &user::Id,
    ) -> user::Result<Option<user::User>>;
    fn get_by_email(
        &self,
        email: &user::Email,
    ) -> user::Result<Option<user::User>>;
    fn save(&self, user: &user::User) -> user::Result<()>;
}
