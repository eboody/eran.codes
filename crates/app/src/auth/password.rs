use nutype::nutype;
use snafu::Snafu;

use super::{BoxError, Result};

#[derive(Debug, Snafu)]
#[snafu(display("{source}"))]
pub struct HashError {
    pub(super) source: BoxError,
}

pub trait Hasher: Send + Sync {
    fn hash(&self, password: &str) -> Result<Hash>;
    fn verify(&self, password: &str, password_hash: &Hash) -> Result<bool>;
}

#[nutype(sanitize(trim), derive(Clone, Debug, PartialEq, Display))]
pub struct Hash(String);
