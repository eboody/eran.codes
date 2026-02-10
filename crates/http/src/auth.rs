use axum::{
    body::Body,
    http::Request,
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};
use axum_login::{AuthUser, AuthnBackend, AuthSession};
use bon::Builder;
use nutype::nutype;

use crate::{paths::Route, request};

#[derive(Clone, Debug, Builder)]
pub struct User {
    pub id: UserId,
    pub username: domain::user::Username,
    pub email: domain::user::Email,
    pub session_hash_bytes: Vec<u8>,
}

impl AuthUser for User {
    type Id = UserId;

    fn id(&self) -> Self::Id {
        self.id.clone()
    }

    fn session_auth_hash(&self) -> &[u8] {
        &self.session_hash_bytes
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
            let domain_id = user_id.to_domain()?;
            let user = auth.get_user(&domain_id).await?;
            Ok(user.map(User::from))
        }
    }
}

impl From<app::auth::AuthenticatedUser> for User {
    fn from(user: app::auth::AuthenticatedUser) -> Self {
        Self::builder()
            .id(UserId::from(user.id))
            .username(user.username)
            .email(user.email)
            .session_hash_bytes(user.session_hash.to_string().into_bytes())
            .build()
    }
}

#[nutype(
    sanitize(trim),
    derive(Clone, Debug, PartialEq, Eq, Hash, Display, Serialize, Deserialize)
)]
pub struct UserId(String);

impl UserId {
    pub fn to_domain(&self) -> Result<domain::user::Id, app::auth::Error> {
        let parsed = self
            .to_string()
            .parse::<uuid::Uuid>()
            .map_err(|error| {
                app::auth::Error::Repository(
                    app::auth::RepositoryErrorText::new(
                        error.to_string(),
                    ),
                )
            })?;
        Ok(domain::user::Id::from_uuid(parsed))
    }
}

impl From<domain::user::Id> for UserId {
    fn from(value: domain::user::Id) -> Self {
        UserId::new(value.as_uuid().to_string())
    }
}

pub type Session = AuthSession<Backend>;

pub async fn set_user_context_middleware(
    auth_session: Session,
    req: Request<Body>,
    next: Next,
) -> Response {
    if let Some(user) = auth_session.user.as_ref() {
        request::set_user_id(user.id.to_string());
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
