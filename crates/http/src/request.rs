#[derive(Clone, Copy, Debug)]
pub enum Kind {
    Page,
    Datastar,
}

#[derive(Clone, Copy, Debug)]
pub struct Context {
    kind: Kind,
}

impl Context {
    pub fn kind(&self) -> Kind {
        self.kind
    }

    pub fn fail(
        &self,
        source: crate::error::Error,
    ) -> crate::error::ContextualError {
        crate::error::ContextualError::from((self.kind, source))
    }

    pub fn wrap(
        &self,
        source: impl Into<crate::error::Error>,
    ) -> crate::error::ContextualError {
        crate::error::ContextualError::from((self.kind, source.into()))
    }
}

impl<S> axum::extract::FromRequestParts<S> for Context
where
    S: Send + Sync,
{
    type Rejection = core::convert::Infallible;

    fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> impl core::future::Future<
        Output = core::result::Result<Self, Self::Rejection>,
    > + Send {
        let has_header = parts
            .headers
            .contains_key("datastar-request");

        let kind = if has_header {
            Kind::Datastar
        } else {
            Kind::Page
        };

        std::future::ready(Ok(Self { kind }))
    }
}
