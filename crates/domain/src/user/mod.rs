mod repo;

use bon::Builder;
use nutype::nutype;
pub use repo::Repository;

#[nutype(
    sanitize(trim, lowercase),
    validate(not_empty, len_char_max = 20),
    derive(Debug, Clone, PartialEq)
)]
pub struct Username(String);

#[nutype(
    sanitize(trim, lowercase),
    validate(not_empty),
    derive(Debug, Clone, PartialEq)
)]
pub struct Email(String);

#[nutype(validate(greater = 0), derive(Debug, Clone, Copy, PartialEq, Eq, Hash))]
pub struct Id(u32);

#[derive(Debug, Clone, PartialEq, Builder)]
pub struct User {
    id: Id,
    username: Username,
    email: Email,
}

impl User {
    // Optional, but I like making the "canonical" construction explicit.
    pub fn new(id: Id, username: Username, email: Email) -> Self {
        Self {
            id,
            username,
            email,
        }
    }

    pub fn id(&self) -> Id {
        self.id
    }
    pub fn username(&self) -> &Username {
        &self.username
    }
    pub fn email(&self) -> &Email {
        &self.email
    }
}
