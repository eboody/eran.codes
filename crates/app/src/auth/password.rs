use nutype::nutype;

use super::{BoxError, Result};

#[derive(Debug)]
pub struct HashError(pub(super) BoxError);

impl core::fmt::Display for HashError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for HashError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&*self.0)
    }
}

pub trait Hasher: Send + Sync {
    fn hash(&self, password: &str) -> Result<Hash>;
    fn verify(&self, password: &str, password_hash: &Hash) -> Result<bool>;
}

#[nutype(sanitize(trim), derive(Clone, Debug, PartialEq, Display))]
pub struct Hash(String);
