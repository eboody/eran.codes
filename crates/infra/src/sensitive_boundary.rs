use std::collections::HashSet;
use std::time::SystemTime;

use app::sensitive::{self, ProviderBoundaryMeta, ProviderClient, ProviderRecords};
use async_trait::async_trait;
use domain::sensitive as sensitive_domain;
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct HttpProvider {
    http: reqwest::Client,
    base_url: reqwest::Url,
}

impl HttpProvider {
    pub fn new(http: reqwest::Client, base_url: &str) -> Self {
        Self {
            http,
            base_url: reqwest::Url::parse(base_url)
                .expect("sensitive provider base url should parse"),
        }
    }

    async fn send_refresh_request(
        &self,
        current_token: Option<&SecretString>,
    ) -> sensitive::Result<TokenRefreshResponse> {
        let response = self
            .http
            .post(self.endpoint("token", sensitive::ProviderOperation::RefreshToken)?)
            .json(&TokenRefreshRequest {
                has_current_token: current_token.is_some(),
            })
            .send()
            .await
            .map_err(|source| {
                sensitive::Error::provider_request(
                    sensitive::ProviderOperation::RefreshToken,
                    source,
                )
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
                sensitive::Error::provider_failure(
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
        let mut endpoint =
            self.endpoint("records", sensitive::ProviderOperation::FetchRecords)?;
        if let Some(cursor) = cursor {
            let cursor = cursor.to_string();
            endpoint.query_pairs_mut().append_pair("after", &cursor);
        }
        let request = self
            .http
            .get(endpoint)
            .bearer_auth(token.access_token.expose_secret());

        let response = request.send().await.map_err(|source| {
            sensitive::Error::provider_request(
                sensitive::ProviderOperation::FetchRecords,
                source,
            )
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
                sensitive::Error::provider_failure(
                    sensitive::ProviderOperation::FetchRecords,
                    sensitive::ProviderFailureKind::MalformedPayload,
                    source,
                )
            })
    }

    fn endpoint(
        &self,
        path: &str,
        operation: sensitive::ProviderOperation,
    ) -> sensitive::Result<reqwest::Url> {
        self.base_url
            .join(path)
            .map_err(|source| sensitive::Error::provider_request(operation, source))
    }
}

#[async_trait]
impl ProviderClient for HttpProvider {
    fn boundary_meta(&self, _provider: sensitive_domain::Provider) -> ProviderBoundaryMeta {
        ProviderBoundaryMeta::builder()
            .mode(sensitive_domain::ProviderMode::LocalStub)
            .endpoint(
                sensitive_domain::DetailText::try_new(self.base_url.as_str())
                    .expect("provider base url should be valid proof text"),
            )
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
            return Err(sensitive::Error::provider_failure(
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
        sensitive::Error::provider_failure(
            sensitive::ProviderOperation::FetchRecords,
            sensitive::ProviderFailureKind::MalformedPayload,
            source,
        )
    })
}

async fn provider_status_error(
    operation: sensitive::ProviderOperation,
    response: reqwest::Response,
) -> sensitive::Error {
    let status = response.status();
    let remote_error = response.json::<RemoteErrorResponse>().await.ok();
    let detail = remote_error
        .as_ref()
        .map(|error| error.message.as_str())
        .unwrap_or(status.as_str());

    let kind = match status {
        reqwest::StatusCode::UNAUTHORIZED => sensitive::ProviderFailureKind::Unauthorized,
        reqwest::StatusCode::TOO_MANY_REQUESTS => {
            sensitive::ProviderFailureKind::RateLimited
        }
        _ => sensitive::ProviderFailureKind::Transport,
    };

    sensitive::Error::provider_failure(operation, kind, std::io::Error::other(detail))
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TokenRefreshRequest {
    pub has_current_token: bool,
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
    pub external_id: String,
    pub redacted_label: String,
    pub redacted_last4: String,
    pub subject_name: String,
    pub classification: String,
    pub note: String,
}

impl RecordPayload {
    fn try_into_domain(self) -> sensitive::Result<sensitive_domain::Record> {
        Ok(sensitive_domain::Record::builder()
            .external_id(
                sensitive_domain::ExternalId::try_new(self.external_id)
                    .map_err(|source| malformed_payload_error(source))?,
            )
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
    pub category: String,
    pub message: String,
}

fn malformed_payload_error(
    source: impl std::error::Error + Send + Sync + 'static,
) -> sensitive::Error {
    sensitive::Error::provider_failure(
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

    async fn spawn_test_server(router: Router) -> String {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("listener");
        let addr = listener.local_addr().expect("local addr");
        tokio::spawn(async move {
            axum::serve(listener, router)
                .await
                .expect("server should run");
        });
        format!("http://{addr}/")
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
                                external_id: "stub-alpha".to_string(),
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
                                external_id: "stub-beta".to_string(),
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
        let provider = HttpProvider::new(reqwest::Client::new(), &base_url);
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
                        Json(RemoteErrorResponse {
                            category: "unauthorized".to_string(),
                            message: "stub forced unauthorized".to_string(),
                        }),
                    )
                }),
            );
        let base_url = spawn_test_server(router).await;
        let provider = HttpProvider::new(reqwest::Client::new(), &base_url);
        let token = refresh_token_for(&provider).await;

        let error = provider
            .fetch_records(test_provider(), &token, None, UNIX_EPOCH)
            .await
            .expect_err("records fetch should fail");

        assert!(matches!(
            error,
            sensitive::Error::Provider {
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
                            external_id: "stub-alpha".to_string(),
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
        let provider = HttpProvider::new(reqwest::Client::new(), &base_url);
        let token = refresh_token_for(&provider).await;

        let error = provider
            .fetch_records(test_provider(), &token, None, UNIX_EPOCH)
            .await
            .expect_err("malformed page should fail");

        assert!(matches!(
            error,
            sensitive::Error::Provider {
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
        let provider = HttpProvider::new(reqwest::Client::new(), &base_url);
        let token = refresh_token_for(&provider).await;

        let error = provider
            .fetch_records(test_provider(), &token, None, UNIX_EPOCH)
            .await
            .expect_err("repeated cursor should fail");

        assert!(matches!(
            error,
            sensitive::Error::Provider {
                operation: sensitive::ProviderOperation::FetchRecords,
                kind: sensitive::ProviderFailureKind::MalformedPayload,
                ..
            }
        ));
    }
}
