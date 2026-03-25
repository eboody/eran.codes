use std::collections::HashSet;
use std::time::{Duration, SystemTime};

use app::sensitive::{self, ProviderBoundaryMeta, ProviderClient, ProviderRecords};
use async_trait::async_trait;
use domain::sensitive as sensitive_domain;
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct HttpProvider {
    http: reqwest::Client,
    mode: sensitive_domain::ProviderMode,
    auth_mode: Option<sensitive_domain::ProviderAuthMode>,
    retry_backoff_secs: Option<u32>,
    endpoint: sensitive_domain::DetailText,
    request_timeout: Option<Duration>,
    boundary: BoundaryReadiness,
}

#[derive(Clone, Debug)]
pub struct SandboxHttpConfig {
    pub base_url: Option<SandboxBaseUrl>,
    pub client_id: Option<SandboxClientId>,
    pub client_secret: Option<String>,
    pub timeout: Duration,
    pub retry_backoff: Duration,
}

#[derive(Clone)]
struct SandboxCredentials {
    client_id: SandboxClientId,
    client_secret: SecretString,
}

#[derive(Clone)]
enum BoundaryReadiness {
    Ready(ReadyBoundary),
    Degraded { reason: String },
}

#[derive(Clone)]
struct ReadyBoundary {
    base_url: reqwest::Url,
    auth: BoundaryAuth,
}

#[derive(Clone)]
enum BoundaryAuth {
    Stub,
    Sandbox(SandboxCredentials),
}

struct UsableBoundary<'a> {
    base_url: &'a reqwest::Url,
    auth: &'a BoundaryAuth,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct SandboxClientId(String);

impl SandboxClientId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    fn is_blank(&self) -> bool {
        self.0.trim().is_empty()
    }
}

impl AsRef<str> for SandboxClientId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SandboxBaseUrl {
    Parsed(reqwest::Url),
    Invalid(String),
}

impl SandboxBaseUrl {
    pub fn parse(value: impl Into<String>) -> Self {
        let value = value.into();
        match reqwest::Url::parse(&value) {
            Ok(url) => Self::Parsed(url),
            Err(_) => Self::Invalid(value),
        }
    }

    fn as_url(&self) -> Option<&reqwest::Url> {
        match self {
            Self::Parsed(url) => Some(url),
            Self::Invalid(_) => None,
        }
    }
}

enum RefreshRequestBody {
    Stub(TokenRefreshRequest),
    Sandbox(SandboxTokenExchangeRequest),
}

impl serde::Serialize for RefreshRequestBody {
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Stub(body) => body.serialize(serializer),
            Self::Sandbox(body) => body.serialize(serializer),
        }
    }
}

impl HttpProvider {
    pub fn new_stub(http: reqwest::Client, base_url: reqwest::Url) -> Self {
        Self {
            http,
            mode: sensitive_domain::ProviderMode::LocalStub,
            auth_mode: Some(sensitive_domain::ProviderAuthMode::StubIssuedToken),
            retry_backoff_secs: None,
            endpoint: detail_text(base_url.as_str()),
            request_timeout: None,
            boundary: BoundaryReadiness::Ready(ReadyBoundary {
                base_url,
                auth: BoundaryAuth::Stub,
            }),
        }
    }

    pub fn new_sandbox(http: reqwest::Client, config: SandboxHttpConfig) -> Self {
        let parsed_url = config
            .base_url
            .as_ref()
            .and_then(SandboxBaseUrl::as_url)
            .cloned();
        let degraded_reason = sandbox_config_error(&config);

        Self {
            http,
            mode: sensitive_domain::ProviderMode::SandboxHttp,
            auth_mode: Some(sensitive_domain::ProviderAuthMode::ClientCredentials),
            retry_backoff_secs: Some(
                config.retry_backoff.as_secs().min(u32::MAX as u64) as u32
            ),
            endpoint: config
                .base_url
                .as_ref()
                .and_then(SandboxBaseUrl::as_url)
                .map(|url| detail_text(url.as_str()))
                .unwrap_or_else(|| detail_text("sandbox endpoint not configured")),
            request_timeout: Some(config.timeout),
            boundary: if let Some(reason) = degraded_reason {
                BoundaryReadiness::Degraded { reason }
            } else {
                BoundaryReadiness::Ready(ReadyBoundary {
                    base_url: parsed_url.expect(
                        "sandbox base url should exist when config is complete",
                    ),
                    auth: BoundaryAuth::Sandbox(SandboxCredentials {
                        client_id: config
                            .client_id
                            .expect("sandbox client id should exist when config is complete"),
                        client_secret: SecretString::new(
                            config
                                .client_secret
                                .expect(
                                    "sandbox client secret should exist when config is complete",
                                )
                                .into_boxed_str(),
                        ),
                    }),
                })
            },
        }
    }

    async fn send_refresh_request(
        &self,
        current_token: Option<&SecretString>,
    ) -> sensitive::Result<TokenRefreshResponse> {
        let boundary = self.usable_boundary(sensitive::ProviderOperation::RefreshToken)?;
        let response = self
            .request_builder_with_timeout(self.http.post(endpoint_url(
                boundary.base_url,
                "token",
                sensitive::ProviderOperation::RefreshToken,
            )?))
            .json(&self.refresh_request_body(boundary.auth, current_token))
            .send()
            .await
            .map_err(|source| {
                provider_request_error(sensitive::ProviderOperation::RefreshToken, source)
            })?;

        if !response.status().is_success() {
            return Err(provider_status_error(
                sensitive::ProviderOperation::RefreshToken,
                response,
            )
            .await);
        }

        response
            .json::<TokenRefreshResponse>()
            .await
            .map_err(|source| {
                sensitive::failure::Error::provider_failure(
                    sensitive::ProviderOperation::RefreshToken,
                    sensitive::ProviderFailureKind::MalformedPayload,
                    source,
                )
            })
    }

    async fn fetch_records_page(
        &self,
        token: &sensitive::ProviderToken,
        cursor: Option<&sensitive_domain::SyncCursor>,
    ) -> sensitive::Result<RecordsPageResponse> {
        let boundary = self.usable_boundary(sensitive::ProviderOperation::FetchRecords)?;
        let mut endpoint = endpoint_url(
            boundary.base_url,
            "records",
            sensitive::ProviderOperation::FetchRecords,
        )?;
        if let Some(cursor) = cursor {
            let cursor = cursor.to_string();
            endpoint.query_pairs_mut().append_pair("after", &cursor);
        }
        let request = self
            .request_builder_with_timeout(self.http.get(endpoint))
            .bearer_auth(token.access_token.expose_secret());

        let response = request.send().await.map_err(|source| {
            provider_request_error(sensitive::ProviderOperation::FetchRecords, source)
        })?;

        if !response.status().is_success() {
            return Err(provider_status_error(
                sensitive::ProviderOperation::FetchRecords,
                response,
            )
            .await);
        }

        response
            .json::<RecordsPageResponse>()
            .await
            .map_err(|source| {
                sensitive::failure::Error::provider_failure(
                    sensitive::ProviderOperation::FetchRecords,
                    sensitive::ProviderFailureKind::MalformedPayload,
                    source,
                )
            })
    }

    fn refresh_request_body(
        &self,
        auth: &BoundaryAuth,
        current_token: Option<&SecretString>,
    ) -> RefreshRequestBody {
        match auth {
            BoundaryAuth::Sandbox(credentials) => {
                RefreshRequestBody::Sandbox(SandboxTokenExchangeRequest {
                    client_id: credentials.client_id.clone(),
                    client_secret: credentials.client_secret.expose_secret().to_string(),
                    grant_type: "client_credentials".to_string(),
                })
            }
            BoundaryAuth::Stub => RefreshRequestBody::Stub(TokenRefreshRequest {
                has_current_token: current_token.is_some(),
            }),
        }
    }

    fn usable_boundary(
        &self,
        operation: sensitive::ProviderOperation,
    ) -> sensitive::Result<UsableBoundary<'_>> {
        match &self.boundary {
            BoundaryReadiness::Ready(boundary) => Ok(UsableBoundary {
                base_url: &boundary.base_url,
                auth: &boundary.auth,
            }),
            BoundaryReadiness::Degraded { reason } => {
                Err(configuration_error(operation, reason))
            }
        }
    }

    fn request_builder_with_timeout(
        &self,
        request: reqwest::RequestBuilder,
    ) -> reqwest::RequestBuilder {
        match self.request_timeout {
            Some(timeout) => request.timeout(timeout),
            None => request,
        }
    }
}

#[async_trait]
impl ProviderClient for HttpProvider {
    fn boundary_meta(&self, _provider: sensitive_domain::Provider) -> ProviderBoundaryMeta {
        ProviderBoundaryMeta::builder()
            .mode(self.mode)
            .endpoint(self.endpoint.clone())
            .maybe_auth_mode(self.auth_mode)
            .maybe_retry_backoff_secs(self.retry_backoff_secs)
            .build()
    }

    async fn refresh_token(
        &self,
        provider: sensitive_domain::Provider,
        now: SystemTime,
        current_token: Option<&SecretString>,
    ) -> sensitive::Result<sensitive::ProviderToken> {
        let response = self.send_refresh_request(current_token).await?;
        if response.access_token.trim().is_empty() {
            return Err(sensitive::failure::Error::provider_failure(
                sensitive::ProviderOperation::RefreshToken,
                sensitive::ProviderFailureKind::MalformedPayload,
                std::io::Error::other("provider returned an empty access token"),
            ));
        }

        Ok(sensitive::ProviderToken::builder()
            .status(
                sensitive_domain::TokenStatus::builder()
                    .provider(provider)
                    .expires_at(
                        now + std::time::Duration::from_secs(response.expires_in_secs),
                    )
                    .refreshed_at(now)
                    .build(),
            )
            .access_token(SecretString::new(response.access_token.into_boxed_str()))
            .build())
    }

    async fn fetch_records(
        &self,
        _provider: sensitive_domain::Provider,
        token: &sensitive::ProviderToken,
        cursor: Option<&sensitive_domain::SyncCursor>,
        _now: SystemTime,
    ) -> sensitive::Result<ProviderRecords> {
        let mut next_cursor = cursor.cloned();
        let mut final_cursor = cursor.cloned();
        let mut records = Vec::new();
        let mut seen_request_cursors = HashSet::new();

        loop {
            if !seen_request_cursors.insert(next_cursor.as_ref().map(ToString::to_string)) {
                return Err(malformed_payload_error(std::io::Error::other(
                    "provider repeated a sync cursor and would not make forward progress",
                )));
            }
            let page = self.fetch_records_page(token, next_cursor.as_ref()).await?;
            let page_cursor = page.cursor.as_deref().map(parse_cursor).transpose()?;
            if let Some(cursor) = page_cursor {
                final_cursor = Some(cursor);
            }

            for record in page.records {
                records.push(record.try_into_domain()?);
            }

            next_cursor = page.next_cursor.as_deref().map(parse_cursor).transpose()?;
            if next_cursor.is_none() {
                break;
            }
        }

        Ok(ProviderRecords::builder()
            .records(records)
            .maybe_cursor(final_cursor)
            .build())
    }
}

fn parse_cursor(value: &str) -> sensitive::Result<sensitive_domain::SyncCursor> {
    sensitive_domain::SyncCursor::try_new(value).map_err(|source| {
        sensitive::failure::Error::provider_failure(
            sensitive::ProviderOperation::FetchRecords,
            sensitive::ProviderFailureKind::MalformedPayload,
            source,
        )
    })
}

async fn provider_status_error(
    operation: sensitive::ProviderOperation,
    response: reqwest::Response,
) -> sensitive::failure::Error {
    let status = response.status();
    let remote_error = response.json::<RemoteErrorResponse>().await.ok();
    let detail = remote_error
        .as_ref()
        .and_then(RemoteErrorResponse::detail)
        .unwrap_or(status.as_str());

    let kind = match status {
        reqwest::StatusCode::UNAUTHORIZED => sensitive::ProviderFailureKind::Unauthorized,
        reqwest::StatusCode::FORBIDDEN => sensitive::ProviderFailureKind::Forbidden,
        reqwest::StatusCode::TOO_MANY_REQUESTS => {
            sensitive::ProviderFailureKind::RateLimited
        }
        status if status.is_server_error() => sensitive::ProviderFailureKind::ServerError,
        _ => sensitive::ProviderFailureKind::Transport,
    };

    sensitive::failure::Error::provider_status_failure(
        operation,
        kind,
        status.as_u16(),
        std::io::Error::other(detail),
    )
}

fn endpoint_url(
    base_url: &reqwest::Url,
    path: &str,
    operation: sensitive::ProviderOperation,
) -> sensitive::Result<reqwest::Url> {
    base_url
        .join(path)
        .map_err(|source| sensitive::failure::Error::provider_request(operation, source))
}

fn provider_request_error(
    operation: sensitive::ProviderOperation,
    source: reqwest::Error,
) -> sensitive::failure::Error {
    if source.is_timeout() {
        return sensitive::failure::Error::provider_failure(
            operation,
            sensitive::ProviderFailureKind::Timeout,
            source,
        );
    }

    sensitive::failure::Error::provider_request(operation, source)
}

fn configuration_error(
    operation: sensitive::ProviderOperation,
    message: &str,
) -> sensitive::failure::Error {
    sensitive::failure::Error::provider_failure(
        operation,
        sensitive::ProviderFailureKind::Configuration,
        std::io::Error::other(message.to_string()),
    )
}

fn sandbox_config_error(config: &SandboxHttpConfig) -> Option<String> {
    match config.base_url.as_ref() {
        Some(SandboxBaseUrl::Parsed(url)) if url.scheme() != "https" => {
            return Some("SENSITIVE_PROVIDER_BASE_URL must use https".to_string());
        }
        Some(SandboxBaseUrl::Invalid(_)) => {
            return Some(
                "SENSITIVE_PROVIDER_BASE_URL must be a valid absolute URL".to_string(),
            );
        }
        _ => {}
    }

    let mut missing = Vec::new();
    if config.base_url.is_none() {
        missing.push("SENSITIVE_PROVIDER_BASE_URL");
    }
    if config
        .client_id
        .as_ref()
        .is_none_or(SandboxClientId::is_blank)
    {
        missing.push("SENSITIVE_SANDBOX_CLIENT_ID");
    }
    if config
        .client_secret
        .as_deref()
        .map(str::trim)
        .unwrap_or_default()
        .is_empty()
    {
        missing.push("SENSITIVE_SANDBOX_CLIENT_SECRET");
    }
    if missing.is_empty() {
        None
    } else {
        Some(format!(
            "sandbox config incomplete: missing {}",
            missing.join(", ")
        ))
    }
}

fn detail_text(value: &str) -> sensitive_domain::DetailText {
    let trimmed = value.trim();
    let normalized = if trimmed.is_empty() {
        "provider endpoint unavailable"
    } else {
        trimmed
    };
    sensitive_domain::DetailText::try_new(normalized)
        .expect("boundary detail text should stay valid")
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TokenRefreshRequest {
    pub has_current_token: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SandboxTokenExchangeRequest {
    pub client_id: SandboxClientId,
    pub client_secret: String,
    pub grant_type: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TokenRefreshResponse {
    pub access_token: String,
    pub expires_in_secs: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RecordsPageResponse {
    pub records: Vec<RecordPayload>,
    pub cursor: Option<String>,
    pub next_cursor: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RecordPayload {
    pub external_id: sensitive_domain::ExternalId,
    pub redacted_label: String,
    pub redacted_last4: String,
    pub subject_name: String,
    pub classification: String,
    pub note: String,
}

impl RecordPayload {
    fn try_into_domain(self) -> sensitive::Result<sensitive_domain::Record> {
        Ok(sensitive_domain::Record::builder()
            .external_id(self.external_id)
            .label(
                sensitive_domain::Label::try_new(self.redacted_label)
                    .map_err(|source| malformed_payload_error(source))?,
            )
            .last4(
                sensitive_domain::Last4::try_new(self.redacted_last4)
                    .map_err(|source| malformed_payload_error(source))?,
            )
            .authorized(
                sensitive_domain::AuthorizedFields::builder()
                    .subject_name(
                        sensitive_domain::DetailText::try_new(self.subject_name)
                            .map_err(|source| malformed_payload_error(source))?,
                    )
                    .classification(
                        sensitive_domain::DetailText::try_new(self.classification)
                            .map_err(|source| malformed_payload_error(source))?,
                    )
                    .note(
                        sensitive_domain::DetailText::try_new(self.note)
                            .map_err(|source| malformed_payload_error(source))?,
                    )
                    .build(),
            )
            .build())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RemoteErrorResponse {
    #[serde(default)]
    category: Option<String>,
    #[serde(flatten)]
    detail: RemoteErrorDetail,
}

impl RemoteErrorResponse {
    pub fn message(category: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            category: Some(category.into()),
            detail: RemoteErrorDetail::Message {
                message: message.into(),
            },
        }
    }

    fn detail(&self) -> Option<&str> {
        self.detail.message_or_error().or(self.category.as_deref())
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(untagged)]
enum RemoteErrorDetail {
    #[default]
    Empty,
    Message {
        message: String,
    },
    Error {
        error: String,
    },
    MessageAndError {
        message: String,
        error: String,
    },
}

impl RemoteErrorDetail {
    fn message_or_error(&self) -> Option<&str> {
        match self {
            Self::Empty => None,
            Self::Message { message } => Some(message.as_str()),
            Self::Error { error } => Some(error.as_str()),
            Self::MessageAndError { message, error } if !message.is_empty() => {
                Some(message.as_str())
            }
            Self::MessageAndError { error, .. } => Some(error.as_str()),
        }
    }
}

fn malformed_payload_error(
    source: impl std::error::Error + Send + Sync + 'static,
) -> sensitive::failure::Error {
    sensitive::failure::Error::provider_failure(
        sensitive::ProviderOperation::FetchRecords,
        sensitive::ProviderFailureKind::MalformedPayload,
        source,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, UNIX_EPOCH};

    use axum::{
        Json, Router,
        extract::Query,
        http::StatusCode,
        routing::{get, post},
    };

    #[derive(Debug, Deserialize)]
    struct RecordsQuery {
        after: Option<String>,
    }

    async fn spawn_test_server(router: Router) -> reqwest::Url {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("listener");
        let addr = listener.local_addr().expect("local addr");
        tokio::spawn(async move {
            axum::serve(listener, router)
                .await
                .expect("server should run");
        });
        reqwest::Url::parse(&format!("http://{addr}/")).expect("test url should parse")
    }

    fn test_provider() -> sensitive_domain::Provider {
        sensitive_domain::Provider::SyntheticSecureFeed
    }

    async fn refresh_token_for(provider: &HttpProvider) -> sensitive::ProviderToken {
        provider
            .refresh_token(test_provider(), UNIX_EPOCH, None)
            .await
            .expect("token refresh")
    }

    fn paginated_success_router() -> Router {
        Router::new()
            .route(
                "/token",
                post(|| async {
                    Json(TokenRefreshResponse {
                        access_token: "local-http-token".to_string(),
                        expires_in_secs: 900,
                    })
                }),
            )
            .route(
                "/records",
                get(|Query(query): Query<RecordsQuery>| async move {
                    Json(match query.after.as_deref() {
                        None => RecordsPageResponse {
                            records: vec![RecordPayload {
                                external_id: sensitive_domain::ExternalId::try_new(
                                    "stub-alpha",
                                )
                                .expect("test external id should be valid"),
                                redacted_label: "Alpha file".to_string(),
                                redacted_last4: "1001".to_string(),
                                subject_name: "Case alpha".to_string(),
                                classification: "sanitized_record".to_string(),
                                note: "First paginated page".to_string(),
                            }],
                            cursor: Some("cursor-alpha".to_string()),
                            next_cursor: Some("cursor-alpha".to_string()),
                        },
                        Some("cursor-alpha") => RecordsPageResponse {
                            records: vec![RecordPayload {
                                external_id: sensitive_domain::ExternalId::try_new(
                                    "stub-beta",
                                )
                                .expect("test external id should be valid"),
                                redacted_label: "Beta ledger".to_string(),
                                redacted_last4: "2002".to_string(),
                                subject_name: "Case beta".to_string(),
                                classification: "sanitized_record".to_string(),
                                note: "Second paginated page".to_string(),
                            }],
                            cursor: Some("cursor-beta".to_string()),
                            next_cursor: None,
                        },
                        Some(other) => RecordsPageResponse {
                            records: Vec::new(),
                            cursor: Some(other.to_string()),
                            next_cursor: None,
                        },
                    })
                }),
            )
    }

    #[tokio::test]
    async fn fetch_records_accumulates_paginated_http_pages() {
        let base_url = spawn_test_server(paginated_success_router()).await;
        let provider = HttpProvider::new_stub(reqwest::Client::new(), base_url);
        let token = refresh_token_for(&provider).await;

        let records = provider
            .fetch_records(
                test_provider(),
                &token,
                None,
                UNIX_EPOCH + Duration::from_secs(1),
            )
            .await
            .expect("records should load");

        assert_eq!(records.records.len(), 2);
        assert_eq!(
            records.cursor.as_ref().map(ToString::to_string),
            Some("cursor-beta".to_string())
        );
        assert_eq!(records.records[0].external_id.to_string(), "stub-alpha");
        assert_eq!(records.records[1].external_id.to_string(), "stub-beta");
    }

    #[tokio::test]
    async fn unauthorized_http_records_response_maps_to_unauthorized_failure() {
        let router = Router::new()
            .route(
                "/token",
                post(|| async {
                    Json(TokenRefreshResponse {
                        access_token: "local-http-token".to_string(),
                        expires_in_secs: 900,
                    })
                }),
            )
            .route(
                "/records",
                get(|| async {
                    (
                        StatusCode::UNAUTHORIZED,
                        Json(RemoteErrorResponse::message(
                            "unauthorized",
                            "stub forced unauthorized",
                        )),
                    )
                }),
            );
        let base_url = spawn_test_server(router).await;
        let provider = HttpProvider::new_stub(reqwest::Client::new(), base_url);
        let token = refresh_token_for(&provider).await;

        let error = provider
            .fetch_records(test_provider(), &token, None, UNIX_EPOCH)
            .await
            .expect_err("records fetch should fail");

        assert!(matches!(
            error,
            sensitive::failure::Error::Provider {
                operation: sensitive::ProviderOperation::FetchRecords,
                kind: sensitive::ProviderFailureKind::Unauthorized,
                ..
            }
        ));
    }

    #[tokio::test]
    async fn malformed_http_payload_fails_closed() {
        let router = Router::new()
            .route(
                "/token",
                post(|| async {
                    Json(TokenRefreshResponse {
                        access_token: "local-http-token".to_string(),
                        expires_in_secs: 900,
                    })
                }),
            )
            .route(
                "/records",
                get(|| async {
                    Json(RecordsPageResponse {
                        records: vec![RecordPayload {
                            external_id: sensitive_domain::ExternalId::try_new(
                                "stub-alpha",
                            )
                            .expect("test external id should be valid"),
                            redacted_label: "Alpha file".to_string(),
                            redacted_last4: "bad".to_string(),
                            subject_name: "Case alpha".to_string(),
                            classification: "sanitized_record".to_string(),
                            note: "Malformed last4 should fail closed".to_string(),
                        }],
                        cursor: Some("cursor-alpha".to_string()),
                        next_cursor: None,
                    })
                }),
            );
        let base_url = spawn_test_server(router).await;
        let provider = HttpProvider::new_stub(reqwest::Client::new(), base_url);
        let token = refresh_token_for(&provider).await;

        let error = provider
            .fetch_records(test_provider(), &token, None, UNIX_EPOCH)
            .await
            .expect_err("malformed page should fail");

        assert!(matches!(
            error,
            sensitive::failure::Error::Provider {
                operation: sensitive::ProviderOperation::FetchRecords,
                kind: sensitive::ProviderFailureKind::MalformedPayload,
                ..
            }
        ));
    }

    #[tokio::test]
    async fn repeated_cursor_fails_closed_instead_of_looping_forever() {
        let router = Router::new()
            .route(
                "/token",
                post(|| async {
                    Json(TokenRefreshResponse {
                        access_token: "local-http-token".to_string(),
                        expires_in_secs: 900,
                    })
                }),
            )
            .route(
                "/records",
                get(|Query(query): Query<RecordsQuery>| async move {
                    Json(RecordsPageResponse {
                        records: Vec::new(),
                        cursor: query
                            .after
                            .clone()
                            .or_else(|| Some("cursor-loop".to_string())),
                        next_cursor: Some("cursor-loop".to_string()),
                    })
                }),
            );
        let base_url = spawn_test_server(router).await;
        let provider = HttpProvider::new_stub(reqwest::Client::new(), base_url);
        let token = refresh_token_for(&provider).await;

        let error = provider
            .fetch_records(test_provider(), &token, None, UNIX_EPOCH)
            .await
            .expect_err("repeated cursor should fail");

        assert!(matches!(
            error,
            sensitive::failure::Error::Provider {
                operation: sensitive::ProviderOperation::FetchRecords,
                kind: sensitive::ProviderFailureKind::MalformedPayload,
                ..
            }
        ));
    }

    #[tokio::test]
    async fn forbidden_http_records_response_maps_to_forbidden_failure() {
        let router = Router::new()
            .route(
                "/token",
                post(|| async {
                    Json(TokenRefreshResponse {
                        access_token: "local-http-token".to_string(),
                        expires_in_secs: 900,
                    })
                }),
            )
            .route(
                "/records",
                get(|| async {
                    (
                        StatusCode::FORBIDDEN,
                        Json(RemoteErrorResponse::message(
                            "forbidden",
                            "sandbox forbids this account",
                        )),
                    )
                }),
            );
        let base_url = spawn_test_server(router).await;
        let provider = HttpProvider::new_stub(reqwest::Client::new(), base_url);
        let token = refresh_token_for(&provider).await;

        let error = provider
            .fetch_records(test_provider(), &token, None, UNIX_EPOCH)
            .await
            .expect_err("records fetch should fail");

        assert!(matches!(
            error,
            sensitive::failure::Error::Provider {
                operation: sensitive::ProviderOperation::FetchRecords,
                kind: sensitive::ProviderFailureKind::Forbidden,
                status_code: Some(403),
                ..
            }
        ));
    }

    #[tokio::test]
    async fn server_error_records_response_maps_to_server_error_failure() {
        let router = Router::new()
            .route(
                "/token",
                post(|| async {
                    Json(TokenRefreshResponse {
                        access_token: "local-http-token".to_string(),
                        expires_in_secs: 900,
                    })
                }),
            )
            .route(
                "/records",
                get(|| async {
                    (
                        StatusCode::BAD_GATEWAY,
                        Json(RemoteErrorResponse::message(
                            "server_error",
                            "sandbox upstream timed out",
                        )),
                    )
                }),
            );
        let base_url = spawn_test_server(router).await;
        let provider = HttpProvider::new_stub(reqwest::Client::new(), base_url);
        let token = refresh_token_for(&provider).await;

        let error = provider
            .fetch_records(test_provider(), &token, None, UNIX_EPOCH)
            .await
            .expect_err("records fetch should fail");

        assert!(matches!(
            error,
            sensitive::failure::Error::Provider {
                operation: sensitive::ProviderOperation::FetchRecords,
                kind: sensitive::ProviderFailureKind::ServerError,
                status_code: Some(502),
                ..
            }
        ));
    }

    #[tokio::test]
    async fn sandbox_config_degrades_instead_of_panicking() {
        let provider = HttpProvider::new_sandbox(
            reqwest::Client::new(),
            SandboxHttpConfig {
                base_url: None,
                client_id: Some(SandboxClientId::new("sandbox-client")),
                client_secret: None,
                timeout: Duration::from_secs(10),
                retry_backoff: Duration::from_secs(45),
            },
        );

        let error = provider
            .refresh_token(test_provider(), UNIX_EPOCH, None)
            .await
            .expect_err("sandbox config should fail closed");

        assert!(matches!(
            error,
            sensitive::failure::Error::Provider {
                operation: sensitive::ProviderOperation::RefreshToken,
                kind: sensitive::ProviderFailureKind::Configuration,
                status_code: None,
                ..
            }
        ));
    }

    #[tokio::test]
    async fn sandbox_fetch_fails_closed_when_config_is_incomplete() {
        let provider = HttpProvider::new_sandbox(
            reqwest::Client::new(),
            SandboxHttpConfig {
                base_url: Some(SandboxBaseUrl::Parsed(
                    reqwest::Url::parse("https://127.0.0.1:9/")
                        .expect("test url should parse"),
                )),
                client_id: Some(SandboxClientId::new("sandbox-client")),
                client_secret: None,
                timeout: Duration::from_secs(10),
                retry_backoff: Duration::from_secs(45),
            },
        );
        let token = sensitive::ProviderToken::builder()
            .status(
                sensitive_domain::TokenStatus::builder()
                    .provider(test_provider())
                    .expires_at(UNIX_EPOCH + Duration::from_secs(300))
                    .refreshed_at(UNIX_EPOCH)
                    .build(),
            )
            .access_token(SecretString::new(
                "cached-token".to_string().into_boxed_str(),
            ))
            .build();

        let error = provider
            .fetch_records(test_provider(), &token, None, UNIX_EPOCH)
            .await
            .expect_err("sandbox fetch should fail closed");

        assert!(matches!(
            error,
            sensitive::failure::Error::Provider {
                operation: sensitive::ProviderOperation::FetchRecords,
                kind: sensitive::ProviderFailureKind::Configuration,
                status_code: None,
                ..
            }
        ));
    }
}
