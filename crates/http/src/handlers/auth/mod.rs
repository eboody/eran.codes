// ci: descriptive-module-import crate::handlers::auth
mod form_input;
mod login_flow;
mod page_gate_flow;
mod protected_flow;
mod register_flow;

use axum::{
    Form,
    extract::{Extension, Query},
    response::{IntoResponse, Redirect},
};
use bon::Builder;
use serde::Deserialize;

use crate::paths::Route;
use crate::types::Text;

pub async fn login_form(
    auth_session: crate::auth::Session,
    Query(query): Query<NextQuery>,
) -> crate::Result<axum::response::Response> {
    let incoming = page_gate_flow::IncomingFlow::from_query(
        page_gate_flow::AuthPageKind::Login,
        query.next,
        auth_session.user.is_some(),
    );
    let sanitized = incoming.sanitize_next();
    Ok(sanitized.classify().into_response())
}

pub async fn register_form(
    auth_session: crate::auth::Session,
    Query(query): Query<NextQuery>,
) -> crate::Result<axum::response::Response> {
    let incoming = page_gate_flow::IncomingFlow::from_query(
        page_gate_flow::AuthPageKind::Register,
        query.next,
        auth_session.user.is_some(),
    );
    let sanitized = incoming.sanitize_next();
    Ok(sanitized.classify().into_response())
}

#[derive(Builder, Deserialize)]
pub struct LoginForm {
    pub email: Text,
    pub password: Text,
    pub next: Option<Text>,
}

#[derive(Builder, Deserialize)]
pub struct RegisterForm {
    pub username: Text,
    pub email: Text,
    pub password: Text,
    pub next: Option<Text>,
}

pub async fn login(
    mut auth_session: crate::auth::Session,
    Form(form): Form<LoginForm>,
) -> crate::Result<axum::response::Response> {
    let incoming = login_flow::IncomingFlow::from_form(form)?;
    incoming
        .authenticate(&mut auth_session)
        .await?
        .into_response(&mut auth_session)
        .await
}

pub async fn register(
    Extension(state): Extension<crate::State>,
    mut auth_session: crate::auth::Session,
    Form(form): Form<RegisterForm>,
) -> crate::Result<axum::response::Response> {
    let incoming = register_flow::IncomingFlow::from_form(form)?;
    incoming
        .register(&state)
        .await?
        .into_response(&mut auth_session)
        .await
}

pub async fn logout(
    mut auth_session: crate::auth::Session,
) -> crate::Result<axum::response::Response> {
    auth_session.logout().await?;
    Ok(Redirect::to(Route::Home.as_str()).into_response())
}

pub async fn protected(
    auth_session: crate::auth::Session,
) -> crate::Result<axum::response::Response> {
    let incoming = protected_flow::IncomingFlow::from_session(auth_session);
    Ok(incoming.classify().into_response())
}

#[derive(Deserialize)]
pub struct NextQuery {
    pub next: Option<Text>,
}

#[derive(Clone, Debug)]
pub(super) struct NextPath(Text);

impl NextPath {
    fn from(value: Text) -> Self {
        Self(value)
    }

    fn into_safe(self) -> Option<String> {
        let value = self.0.to_string();
        if Self::is_safe(&value) { Some(value) } else { None }
    }

    pub(super) fn sanitize(next: Option<Text>) -> Option<String> {
        next.and_then(|value| Self::from(value).into_safe())
    }

    fn is_safe(value: &str) -> bool {
        let bytes = value.as_bytes();
        matches!(bytes.first(), Some(b'/')) && !matches!(bytes.get(1), Some(b'/'))
    }
}
