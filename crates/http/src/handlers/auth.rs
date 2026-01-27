use axum::{
    Form,
    extract::{Extension, Query},
    response::{IntoResponse, Redirect},
};
use bon::Builder;
use serde::Deserialize;

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
    pub email: String,
    pub password: String,
    pub next: Option<String>,
}

#[derive(Builder, Deserialize)]
pub struct RegisterForm {
    pub username: String,
    pub email: String,
    pub password: String,
    pub next: Option<String>,
}

pub async fn login(
    mut auth_session: crate::auth::Session,
    Form(form): Form<LoginForm>,
) -> crate::Result<axum::response::Response> {
    let next = sanitize_next(form.next.clone());
    let credentials = app::auth::Credentials::builder()
        .email(form.email)
        .password(SecretString::new(form.password.into()))
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
    let credentials = app::auth::Credentials::builder()
        .email(form.email.clone())
        .password(SecretString::new(form.password.clone().into()))
        .build();

    let command = app::user::RegisterUser::builder()
        .username(form.username)
        .email(form.email)
        .password(SecretString::new(form.password.into()))
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
    Ok(Redirect::to("/").into_response())
}

pub async fn protected(
    auth_session: crate::auth::Session,
) -> crate::Result<axum::response::Response> {
    let Some(user) = auth_session.user else {
        return Ok(Redirect::to("/login").into_response());
    };

    Ok(views::render(
        pages::Protected::builder()
            .username(user.username.clone())
            .email(user.email.clone())
            .maybe_user(Some(
                crate::views::page::UserNav::builder()
                    .username(user.username)
                    .email(user.email)
                    .build(),
            ))
            .build(),
    )
    .into_response())
}

#[derive(Deserialize)]
pub struct NextQuery {
    pub next: Option<String>,
}

fn sanitize_next(next: Option<String>) -> Option<String> {
    next.and_then(|value| {
        if value.starts_with('/') && !value.starts_with("//") {
            Some(value)
        } else {
            None
        }
    })
}

fn redirect_to_next(next: Option<String>) -> Redirect {
    let target = next.unwrap_or_else(|| "/protected".to_string());
    Redirect::to(&target)
}
