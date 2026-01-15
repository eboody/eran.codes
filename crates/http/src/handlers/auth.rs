use axum::{
    Form,
    extract::Extension,
    response::{IntoResponse, Redirect},
};
use serde::Deserialize;

use crate::views::{self, pages};
use secrecy::SecretString;

pub async fn login_form(
    auth_session: crate::auth::Session,
) -> crate::Result<axum::response::Response> {
    if auth_session.user.is_some() {
        return Ok(Redirect::to("/protected").into_response());
    }

    Ok(views::render(pages::Login {
        message: None,
        user: None,
    })
    .into_response())
}

pub async fn register_form(
    auth_session: crate::auth::Session,
) -> crate::Result<axum::response::Response> {
    if auth_session.user.is_some() {
        return Ok(Redirect::to("/protected").into_response());
    }

    Ok(views::render(pages::Register {
        message: None,
        user: None,
    })
    .into_response())
}

#[derive(Deserialize)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RegisterForm {
    pub username: String,
    pub email: String,
    pub password: String,
}

pub async fn login(
    mut auth_session: crate::auth::Session,
    Form(form): Form<LoginForm>,
) -> crate::Result<axum::response::Response> {
    let credentials = app::auth::Credentials {
        email: form.email,
        password: SecretString::new(form.password.into()),
    };

    if let Some(user) = auth_session.authenticate(credentials).await? {
        auth_session.login(&user).await?;
        return Ok(Redirect::to("/protected").into_response());
    }

    Ok(views::render(pages::Login {
        message: Some("Invalid email or password."),
        user: None,
    })
    .into_response())
}

pub async fn register(
    Extension(state): Extension<crate::State>,
    mut auth_session: crate::auth::Session,
    Form(form): Form<RegisterForm>,
) -> crate::Result<axum::response::Response> {
    let credentials = app::auth::Credentials {
        email: form.email.clone(),
        password: SecretString::new(form.password.clone().into()),
    };

    let command = app::user::RegisterUser {
        username: form.username,
        email: form.email,
        password: SecretString::new(form.password.into()),
    };

    match state.user.register_user(command).await {
        Ok(_) => {
            if let Some(user) = auth_session.authenticate(credentials).await? {
                auth_session.login(&user).await?;
                Ok(Redirect::to("/protected").into_response())
            } else {
                Err(crate::Error::Internal)
            }
        }
        Err(app::user::Error::EmailTaken) => Ok(views::render(pages::Register {
            message: Some("Email already in use."),
            user: None,
        })
        .into_response()),
        Err(app::user::Error::Domain(_)) => Ok(views::render(pages::Register {
            message: Some("Invalid input."),
            user: None,
        })
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

    Ok(views::render(pages::Protected {
        username: user.username.clone(),
        email: user.email.clone(),
        user: Some(crate::views::page::UserNav {
            username: user.username,
            email: user.email,
        }),
    })
    .into_response())
}
