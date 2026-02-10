use axum::{
    Form,
    extract::{Extension, Query},
    response::{IntoResponse, Redirect},
};
use bon::Builder;
use serde::Deserialize;

use crate::paths::Route;
use crate::types::Text;
use crate::views::{self, pages};
use secrecy::SecretString;

pub async fn login_form(
    auth_session: crate::auth::Session,
    Query(query): Query<NextQuery>,
) -> crate::Result<axum::response::Response> {
    let next = sanitize_next(query.next);
    if auth_session.user.is_some() {
        return Ok(redirect_to_next(next).into_response());
    }

    Ok(views::render(
        pages::Login::builder()
            .maybe_next(next.as_deref())
            .build(),
    )
    .into_response())
}

pub async fn register_form(
    auth_session: crate::auth::Session,
    Query(query): Query<NextQuery>,
) -> crate::Result<axum::response::Response> {
    let next = sanitize_next(query.next);
    if auth_session.user.is_some() {
        return Ok(redirect_to_next(next).into_response());
    }

    Ok(views::render(
        pages::Register::builder()
            .maybe_next(next.as_deref())
            .build(),
    )
    .into_response())
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
    let next = sanitize_next(form.next.clone());
    let email = domain::user::Email::try_new(form.email.to_string())
        .map_err(domain::user::Error::from)
        .map_err(app::user::Error::from)?;
    let credentials = app::auth::Credentials::builder()
        .email(email)
        .password(SecretString::new(form.password.to_string().into()))
        .build();

    if let Some(user) = auth_session.authenticate(credentials).await? {
        auth_session.login(&user).await?;
        return Ok(redirect_to_next(next).into_response());
    }

    Ok(views::render(
        pages::Login::builder()
            .maybe_message(Some("Invalid email or password."))
            .maybe_next(next.as_deref())
            .build(),
    )
    .into_response())
}

pub async fn register(
    Extension(state): Extension<crate::State>,
    mut auth_session: crate::auth::Session,
    Form(form): Form<RegisterForm>,
) -> crate::Result<axum::response::Response> {
    let next = sanitize_next(form.next.clone());
    let email = domain::user::Email::try_new(form.email.to_string())
        .map_err(domain::user::Error::from)
        .map_err(app::user::Error::from)?;
    let username = domain::user::Username::try_new(form.username.to_string())
        .map_err(domain::user::Error::from)
        .map_err(app::user::Error::from)?;
    let credentials = app::auth::Credentials::builder()
        .email(email.clone())
        .password(SecretString::new(form.password.to_string().into()))
        .build();

    let command = app::user::RegisterUser::builder()
        .username(username)
        .email(email)
        .password(SecretString::new(form.password.to_string().into()))
        .build();

    match state.user.register_user(command).await {
        Ok(_) => {
            if let Some(user) = auth_session.authenticate(credentials).await? {
                auth_session.login(&user).await?;
                Ok(redirect_to_next(next).into_response())
            } else {
                Err(crate::Error::Internal)
            }
        }
        Err(app::user::Error::EmailTaken) => Ok(views::render(
            pages::Register::builder()
                .maybe_message(Some("Email already in use."))
                .maybe_next(next.as_deref())
                .build(),
        )
        .into_response()),
        Err(app::user::Error::Domain(_)) => Ok(views::render(
            pages::Register::builder()
                .maybe_message(Some("Invalid input."))
                .maybe_next(next.as_deref())
                .build(),
        )
        .into_response()),
        Err(error) => Err(error.into()),
    }
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
    let Some(user) = auth_session.user else {
        return Ok(Redirect::to(Route::Login.as_str()).into_response());
    };

    Ok(views::render(
        pages::Protected::builder()
            .username(Text::from(user.username.to_string()))
            .email(Text::from(user.email.to_string()))
            .maybe_user(Some(
                crate::views::page::UserNav::builder()
                    .username(Text::from(user.username.to_string()))
                    .email(Text::from(user.email.to_string()))
                    .build(),
            ))
            .build(),
    )
    .into_response())
}

#[derive(Deserialize)]
pub struct NextQuery {
    pub next: Option<Text>,
}

fn sanitize_next(next: Option<Text>) -> Option<String> {
    next.and_then(|value| NextPath::from(value).into_safe())
}

fn redirect_to_next(next: Option<String>) -> Redirect {
    let target = next.unwrap_or_else(|| Route::Protected.as_str().to_string());
    Redirect::to(&target)
}

#[derive(Clone, Debug)]
struct NextPath(Text);

impl NextPath {
    fn from(value: Text) -> Self {
        Self(value)
    }

    fn into_safe(self) -> Option<String> {
        let value = self.0.to_string();
        if Self::is_safe(&value) {
            Some(value)
        } else {
            None
        }
    }

    fn is_safe(value: &str) -> bool {
        let bytes = value.as_bytes();
        matches!(bytes.first(), Some(b'/'))
            && !matches!(bytes.get(1), Some(b'/'))
    }
}
