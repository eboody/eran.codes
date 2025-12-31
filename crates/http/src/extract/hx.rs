use axum::extract::FromRequestParts;
use axum::http::request::Parts;

#[derive(Debug, Clone, Copy)]
pub struct HxRequest(pub bool);

impl HxRequest {
    pub fn is_hx(self) -> bool {
        self.0
    }
}

impl<S> FromRequestParts<S> for HxRequest
where
    S: Send + Sync,
{
    type Rejection = core::convert::Infallible;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        let is_hx = parts
            .headers
            .get("HX-Request")
            .and_then(|value| value.to_str().ok())
            .map(|value| value.eq_ignore_ascii_case("true"))
            .unwrap_or(false);

        Ok(Self(is_hx))
    }
}
