mod error;

use std::sync::Arc;

use domain::user;
pub use error::{Error, Result};

#[allow(unused)]
#[derive(Clone)]
pub struct Service {
    users: Arc<dyn user::Repository>,
}

impl Service {
    pub fn new(users: Arc<dyn user::Repository>) -> Self {
        Self { users }
    }

    #[tracing::instrument(skip(self))]
    pub async fn register_user(&self) -> Result<user::Id> {
        todo!()
    }
}
