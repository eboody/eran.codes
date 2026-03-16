use axum::response::IntoResponse;
use statum::{machine, state, transition};

use crate::types::Text;
use crate::views::{self, pages};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum AuthPageKind {
    Login,
    Register,
}

#[state]
pub enum AuthPageGateState {
    Incoming,
    NextSanitized,
    RedirectReady,
    RenderReady,
}

#[machine]
pub(super) struct AuthPageGateFlow<AuthPageGateState> {
    page: AuthPageKind,
    next_raw: Option<Text>,
    next_sanitized: Option<String>,
    is_authenticated: bool,
}

impl AuthPageGateFlow<Incoming> {
    pub(super) fn from_query(
        page: AuthPageKind,
        next_raw: Option<Text>,
        is_authenticated: bool,
    ) -> Self {
        AuthPageGateFlow::<Incoming>::builder()
            .page(page)
            .next_raw(next_raw)
            .next_sanitized(None)
            .is_authenticated(is_authenticated)
            .build()
    }
}

#[transition]
impl AuthPageGateFlow<Incoming> {
    pub(super) fn sanitize_next(mut self) -> AuthPageGateFlow<NextSanitized> {
        self.next_sanitized = super::NextPath::sanitize(self.next_raw.take());
        self.transition()
    }
}

impl AuthPageGateFlow<NextSanitized> {
    pub(super) fn classify(self) -> AuthPageGateOutcome {
        if self.is_authenticated {
            AuthPageGateOutcome::Redirect(self.mark_redirect_ready())
        } else {
            AuthPageGateOutcome::Render(self.mark_render_ready())
        }
    }
}

#[transition]
impl AuthPageGateFlow<NextSanitized> {
    fn mark_redirect_ready(self) -> AuthPageGateFlow<RedirectReady> {
        self.transition()
    }

    fn mark_render_ready(self) -> AuthPageGateFlow<RenderReady> {
        self.transition()
    }
}

impl AuthPageGateFlow<RedirectReady> {
    pub(super) fn into_response(self) -> axum::response::Response {
        let target = self
            .next_sanitized
            .unwrap_or_else(|| crate::paths::Route::Protected.as_str().to_string());
        axum::response::Redirect::to(&target).into_response()
    }
}

impl AuthPageGateFlow<RenderReady> {
    pub(super) fn into_response(self) -> axum::response::Response {
        match self.page {
            AuthPageKind::Login => views::render(
                pages::Login::builder()
                    .maybe_next(self.next_sanitized.as_deref())
                    .build(),
            )
            .into_response(),
            AuthPageKind::Register => views::render(
                pages::Register::builder()
                    .maybe_next(self.next_sanitized.as_deref())
                    .build(),
            )
            .into_response(),
        }
    }
}

pub(super) enum AuthPageGateOutcome {
    Redirect(AuthPageGateFlow<RedirectReady>),
    Render(AuthPageGateFlow<RenderReady>),
}

impl AuthPageGateOutcome {
    pub(super) fn into_response(self) -> axum::response::Response {
        match self {
            Self::Redirect(redirect) => redirect.into_response(),
            Self::Render(render) => render.into_response(),
        }
    }
}

pub(super) type IncomingFlow = AuthPageGateFlow<Incoming>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn authenticated_user_is_redirected() {
        let incoming = IncomingFlow::from_query(
            AuthPageKind::Login,
            Some(Text::from("/protected")),
            true,
        );
        let sanitized = incoming.sanitize_next();

        assert!(matches!(
            sanitized.classify(),
            AuthPageGateOutcome::Redirect(_)
        ));
    }

    #[test]
    fn guest_user_is_rendered() {
        let incoming = IncomingFlow::from_query(
            AuthPageKind::Register,
            Some(Text::from("/protected")),
            false,
        );
        let sanitized = incoming.sanitize_next();

        assert!(matches!(
            sanitized.classify(),
            AuthPageGateOutcome::Render(_)
        ));
    }

    #[test]
    fn sanitize_next_drops_unsafe_redirects() {
        let incoming = IncomingFlow::from_query(
            AuthPageKind::Login,
            Some(Text::from("//evil.example")),
            false,
        );
        let sanitized = incoming.sanitize_next();

        assert!(sanitized.next_sanitized.is_none());
    }
}
