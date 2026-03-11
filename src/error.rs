use snafu::prelude::*;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum Error {
    #[snafu(display("failed to load startup configuration: {source}"))]
    LoadConfig { source: crate::config::Error },
    #[snafu(display("failed to initialize infra: {source}"))]
    InitInfra { source: infra::Error },
    #[snafu(display("failed to bind HTTP listener at {addr}: {source}"))]
    BindHttpListener {
        addr: String,
        source: std::io::Error,
    },
    #[snafu(display("http server exited with an error: {source}"))]
    ServeHttp { source: std::io::Error },
}
