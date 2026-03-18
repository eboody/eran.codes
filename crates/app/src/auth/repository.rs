use domain::user;
use snafu::prelude::*;
use strum_macros::Display;

use super::BoxError;

#[derive(Clone, Copy, Debug, Display)]
pub enum Operation {
    #[strum(serialize = "find auth record by email")]
    FindByEmail,
    #[strum(serialize = "find auth record by id")]
    FindById,
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("auth repository query failed while {operation}: {source}"))]
    Query {
        operation: Operation,
        source: BoxError,
    },
    #[snafu(display("failed to decode auth username: {source}"))]
    DecodeUsername { source: user::UsernameError },
    #[snafu(display("failed to decode auth email: {source}"))]
    DecodeEmail { source: user::EmailError },
}
