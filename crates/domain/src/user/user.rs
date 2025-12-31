use bon::Builder;

use super::{Email, Id, Username};

#[derive(Debug, Clone, PartialEq, Builder)]
pub struct User {
    pub id: Id,
    pub username: Username,
    pub email: Email,
}
