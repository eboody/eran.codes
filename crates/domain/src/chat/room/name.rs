use nutype::nutype;
use snafu::Snafu;

#[nutype(
    sanitize(trim),
    validate(not_empty, len_char_max = 64),
    derive(Debug, Clone, PartialEq, Eq, Display)
)]
pub struct Text(String);

#[derive(Debug, Clone, PartialEq, Snafu)]
pub enum Error {
    #[snafu(display("{source}"))]
    Invalid { source: TextError },
    #[snafu(display("{value}"))]
    Unknown { value: Text },
}
