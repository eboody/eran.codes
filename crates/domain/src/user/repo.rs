use crate::{error::Result, user};

pub trait Repository: Send + Sync {
    fn get_by_id(&self, id: &user::Id) -> Result<Option<user::Model>>;
    fn get_by_email(&self, email: &user::Email) -> Result<Option<user::Model>>;
    fn save(&self, user: &user::Model) -> Result<()>;
}
