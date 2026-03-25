use nutype::nutype;

#[nutype(
    sanitize(trim),
    validate(not_empty, len_char_max = 128),
    derive(Debug, Clone, PartialEq, Eq, Display)
)]
pub struct Id(String);
