use std::sync::Arc;

use domain::user;

// use infra::repo;
use crate::error::Result;

#[allow(unused)]
#[derive(Clone)]
pub struct Service {
    users: Arc<dyn user::Repository>,
}

impl Service {
    pub fn new(users: Arc<dyn user::Repository>) -> Self {
        Self { users }
    }

    pub async fn register_user(&self) -> Result<user::Id> {
        todo!()
    }
}
