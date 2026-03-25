use snafu::Snafu;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("invalid external id: {source}"))]
    ExternalId { source: super::ExternalIdError },
    #[snafu(display("invalid label: {source}"))]
    Label { source: super::LabelError },
    #[snafu(display("invalid last4: {source}"))]
    Last4 { source: super::Last4Error },
    #[snafu(display("invalid detail text: {source}"))]
    DetailText { source: super::DetailTextError },
}

impl From<super::ExternalIdError> for Error {
    fn from(source: super::ExternalIdError) -> Self {
        Self::ExternalId { source }
    }
}

impl From<super::LabelError> for Error {
    fn from(source: super::LabelError) -> Self {
        Self::Label { source }
    }
}

impl From<super::Last4Error> for Error {
    fn from(source: super::Last4Error) -> Self {
        Self::Last4 { source }
    }
}

impl From<super::DetailTextError> for Error {
    fn from(source: super::DetailTextError) -> Self {
        Self::DetailText { source }
    }
}
