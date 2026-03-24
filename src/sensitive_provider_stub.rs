use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

use axum::{
    Json, Router,
    extract::{Query, State},
    http::{HeaderMap, StatusCode, header},
    response::IntoResponse,
    routing::{get, post},
};

use crate::config::SensitiveProviderStubFailureMode;
use infra::sensitive_boundary::{
    RecordPayload, RecordsPageResponse, RemoteErrorResponse, TokenRefreshRequest,
    TokenRefreshResponse,
};

#[derive(Clone)]
struct StubState {
    failure_mode: SensitiveProviderStubFailureMode,
    unauthorized_triggered: Arc<AtomicBool>,
}

impl StubState {
    fn new(failure_mode: SensitiveProviderStubFailureMode) -> Self {
        Self {
            failure_mode,
            unauthorized_triggered: Arc::new(AtomicBool::new(false)),
        }
    }
}

#[derive(Debug, serde::Deserialize)]
struct RecordsQuery {
    after: Option<String>,
}

pub(crate) fn router(failure_mode: SensitiveProviderStubFailureMode) -> Router {
    Router::new()
        .route("/token", post(refresh_token))
        .route("/records", get(records_page))
        .with_state(StubState::new(failure_mode))
}

async fn refresh_token(
    State(_state): State<StubState>,
    Json(request): Json<TokenRefreshRequest>,
) -> impl IntoResponse {
    let token_kind = if request.has_current_token {
        "refresh"
    } else {
        "bootstrap"
    };

    Json(TokenRefreshResponse {
        access_token: format!("local-http-token-{token_kind}"),
        expires_in_secs: 900,
    })
}

async fn records_page(
    State(state): State<StubState>,
    Query(query): Query<RecordsQuery>,
    headers: HeaderMap,
) -> impl IntoResponse {
    if !bearer_token_present(&headers) {
        return error_response(
            StatusCode::UNAUTHORIZED,
            "unauthorized",
            "missing or invalid bearer token",
        );
    }

    match state.failure_mode {
        SensitiveProviderStubFailureMode::UnauthorizedOnce => {
            if !state.unauthorized_triggered.swap(true, Ordering::SeqCst) {
                return error_response(
                    StatusCode::UNAUTHORIZED,
                    "unauthorized",
                    "stub forced one unauthorized response",
                );
            }
        }
        SensitiveProviderStubFailureMode::RateLimited => {
            return error_response(
                StatusCode::TOO_MANY_REQUESTS,
                "rate_limited",
                "stub forced a rate-limited response",
            );
        }
        SensitiveProviderStubFailureMode::MalformedPage => {
            return (
                StatusCode::OK,
                Json(RecordsPageResponse {
                    records: vec![RecordPayload {
                        external_id: "synthetic-alpha".to_string(),
                        redacted_label: "Alpha file".to_string(),
                        redacted_last4: "bad".to_string(),
                        subject_name: "Case alpha".to_string(),
                        classification: "synthetic_record".to_string(),
                        note: "Malformed last4 should fail closed.".to_string(),
                    }],
                    cursor: Some("cursor-alpha".to_string()),
                    next_cursor: None,
                }),
            )
                .into_response();
        }
        SensitiveProviderStubFailureMode::Healthy => {}
    }

    (
        StatusCode::OK,
        Json(page_for_cursor(query.after.as_deref())),
    )
        .into_response()
}

fn bearer_token_present(headers: &HeaderMap) -> bool {
    headers
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .map(|value| value.starts_with("Bearer ") && value.len() > "Bearer ".len())
        .unwrap_or(false)
}

fn error_response(
    status: StatusCode,
    category: &str,
    message: &str,
) -> axum::response::Response {
    (
        status,
        Json(RemoteErrorResponse {
            category: category.to_string(),
            message: message.to_string(),
        }),
    )
        .into_response()
}

fn page_for_cursor(after: Option<&str>) -> RecordsPageResponse {
    match after {
        None => RecordsPageResponse {
            records: vec![
                record(
                    "synthetic-alpha",
                    "Alpha file",
                    "1001",
                    "Case alpha",
                    "synthetic_record",
                    "Encrypted runtime proof sample for authorized viewers.",
                ),
                record(
                    "synthetic-beta",
                    "Beta ledger",
                    "2002",
                    "Case beta",
                    "synthetic_record",
                    "Second page record proving paginated local integration.",
                ),
            ],
            cursor: Some("cursor-beta".to_string()),
            next_cursor: Some("cursor-beta".to_string()),
        },
        Some("cursor-beta") => RecordsPageResponse {
            records: vec![record(
                "synthetic-gamma",
                "Gamma intake",
                "3003",
                "Case gamma",
                "synthetic_record",
                "Final paginated record from the local stub boundary.",
            )],
            cursor: Some("cursor-gamma".to_string()),
            next_cursor: None,
        },
        Some(other) => RecordsPageResponse {
            records: Vec::new(),
            cursor: Some(other.to_string()),
            next_cursor: None,
        },
    }
}

fn record(
    external_id: &str,
    redacted_label: &str,
    redacted_last4: &str,
    subject_name: &str,
    classification: &str,
    note: &str,
) -> RecordPayload {
    RecordPayload {
        external_id: external_id.to_string(),
        redacted_label: redacted_label.to_string(),
        redacted_last4: redacted_last4.to_string(),
        subject_name: subject_name.to_string(),
        classification: classification.to_string(),
        note: note.to_string(),
    }
}
