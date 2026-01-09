moddef::moddef!(mod { error, new_user, repo, user });

pub use error::{Error, Result};
pub use new_user::NewUser;
use nutype::nutype;
pub use repo::Repository;
pub use user::User;

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

#[nutype(
    validate(greater = 0),
    derive(Debug, Clone, Copy, PartialEq, Eq, Hash)
)]
pub struct Id(u32);
