use snafu::prelude::*;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("domain repository error"))]
    Repo,
}
