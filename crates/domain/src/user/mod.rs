mod error;
mod new_user;
mod repo;
mod user;

pub use error::*;
pub use new_user::*;
use nutype::nutype;
pub use repo::*;
pub use user::*;

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
