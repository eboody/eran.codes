use bon::Builder;

use super::{Email, UserId, Username};

#[derive(Debug, Clone, PartialEq, Builder)]
pub struct User {
    pub id: UserId,
    pub username: Username,
    pub email: Email,
}
