use axum::http;
use maud::Render;

#[derive(Default)]
pub(super) struct IncomingFlow;

impl IncomingFlow {
    pub(super) fn new() -> Self {
        Self
    }

    pub(super) async fn prepare_snapshot(
        self,
        auth_session: &crate::auth::Session,
        state: &crate::State,
    ) -> crate::Result<PreparedFlow> {
        let viewer = if let Some(user) = auth_session.user.as_ref() {
            app::sensitive::Viewer::Authenticated(
                app::sensitive::AuthenticatedViewer::builder()
                    .user_id(domain::user::Id::try_from(user.id.clone())?)
                    .email(user.email.clone())
                    .build(),
            )
        } else {
            app::sensitive::Viewer::Guest
        };
        let snapshot = state.sensitive.snapshot(viewer).await?;

        Ok(PreparedFlow {
            snapshot,
            trace: super::trace_snapshot(state),
        })
    }
}

pub(super) struct PreparedFlow {
    snapshot: app::sensitive::Snapshot,
    trace: Vec<crate::trace_log::store::TraceEntry>,
}

impl PreparedFlow {
    pub(super) fn into_response(self) -> (http::StatusCode, axum::response::Html<String>) {
        let partial = crate::views::partials::SensitiveProof::builder()
            .snapshot(self.snapshot)
            .trace(self.trace)
            .build();
        (
            http::StatusCode::OK,
            axum::response::Html(partial.render().into_string()),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::UNIX_EPOCH;

    #[test]
    fn prepared_flow_renders_sensitive_proof_target() {
        let prepared = PreparedFlow {
            snapshot: app::sensitive::Snapshot::builder()
                .viewer(app::sensitive::ViewerState::guest())
                .maybe_token(None)
                .maybe_latest_sync(None)
                .maybe_integration_state(None)
                .records(Vec::new())
                .maybe_authorized_record(None)
                .access_events(Vec::new())
                .build(),
            trace: Vec::new(),
        };

        let response = prepared.into_response();
        assert_eq!(response.0, http::StatusCode::OK);
        assert!(response.1.0.contains("sensitive-proof-target"));
        assert!(!response.1.0.contains("UNIX_EPOCH"));
        assert!(UNIX_EPOCH <= std::time::SystemTime::now());
    }
}
