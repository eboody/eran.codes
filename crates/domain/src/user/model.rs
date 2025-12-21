use nutype::nutype;

pub struct Model {
    pub id: UserId,
    pub username: Username,
    pub email: Email,
}

// Define newtype Username
#[nutype(
    sanitize(trim, lowercase),
    validate(not_empty, len_char_max = 20),
    derive(Debug, PartialEq, Clone)
)]
pub struct Username(String);

#[nutype(sanitize(trim), validate(not_empty), derive(Debug, PartialEq, Clone))]
pub struct Email(String);

#[nutype(derive(Debug, PartialEq, Clone))]
pub struct UserId(u32);
