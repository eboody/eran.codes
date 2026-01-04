use crate::{error::Result, user};

pub trait Repository: Send + Sync {
    fn get_by_id(
        &self,
        id: &user::Id,
    ) -> Result<Option<user::User>>;
    fn get_by_email(
        &self,
        email: &user::Email,
    ) -> Result<Option<user::User>>;
    fn save(&self, user: &user::User) -> Result<()>;
}
