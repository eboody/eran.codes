use axum::{
    body::Body,
    http::Request,
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};
use axum_login::{AuthUser, AuthnBackend, AuthSession};
use bon::Builder;

use crate::{paths::Route, request};

#[derive(Clone, Debug, Builder)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub session_hash: String,
}

impl AuthUser for User {
    type Id = String;

    fn id(&self) -> Self::Id {
        self.id.clone()
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.session_hash.as_bytes()
    }
}

#[derive(Clone)]
pub struct Backend {
    auth: app::auth::Service,
}

impl Backend {
    pub fn new(auth: app::auth::Service) -> Self {
        Self { auth }
    }
}

impl AuthnBackend for Backend {
    type User = User;
    type Credentials = app::auth::Credentials;
    type Error = app::auth::Error;

    fn authenticate(
        &self,
        credentials: Self::Credentials,
    ) -> impl core::future::Future<Output = Result<Option<Self::User>, Self::Error>> + Send {
        let auth = self.auth.clone();
        async move {
            let user = auth.authenticate(credentials).await?;
            Ok(user.map(User::from))
        }
    }

    fn get_user(
        &self,
        user_id: &<Self::User as AuthUser>::Id,
    ) -> impl core::future::Future<Output = Result<Option<Self::User>, Self::Error>> + Send {
        let auth = self.auth.clone();
        let user_id = user_id.clone();
        async move {
            let user = auth.get_user(&user_id).await?;
            Ok(user.map(User::from))
        }
    }
}

impl From<app::auth::AuthenticatedUser> for User {
    fn from(user: app::auth::AuthenticatedUser) -> Self {
        Self::builder()
            .id(user.id)
            .username(user.username)
            .email(user.email)
            .session_hash(user.session_hash)
            .build()
    }
}

pub type Session = AuthSession<Backend>;

pub async fn set_user_context_middleware(
    auth_session: Session,
    req: Request<Body>,
    next: Next,
) -> Response {
    if let Some(user) = auth_session.user.as_ref() {
        request::set_user_id(&user.id);
    }

    next.run(req).await
}

pub async fn require_auth_middleware(
    auth_session: Session,
    req: Request<Body>,
    next: Next,
) -> Response {
    if auth_session.user.is_some() {
        return next.run(req).await;
    }

    let next_path = req
        .uri()
        .path_and_query()
        .map(|value| value.as_str())
        .unwrap_or("/");
    let redirect = format!(
        "{}?next={}",
        Route::Login.as_str(),
        urlencoding::encode(next_path)
    );
    Redirect::to(&redirect).into_response()
}
