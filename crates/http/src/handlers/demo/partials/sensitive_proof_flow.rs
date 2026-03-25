use axum::http;
use maud::Render;

pub(super) async fn prepare_snapshot_response(
    auth_session: &crate::auth::Session,
    state: &crate::State,
) -> crate::Result<(http::StatusCode, axum::response::Html<String>)> {
    let snapshot = state
        .sensitive
        .snapshot(viewer_from_session(auth_session)?)
        .await?;
    Ok(render_response(snapshot, super::trace_snapshot(state)))
}

fn viewer_from_session(
    auth_session: &crate::auth::Session,
) -> crate::Result<app::sensitive::Viewer> {
    Ok(if let Some(user) = auth_session.user.as_ref() {
        app::sensitive::Viewer::Authenticated(
            app::sensitive::AuthenticatedViewer::builder()
                .user_id(domain::user::Id::try_from(user.id.clone())?)
                .email(user.email.clone())
                .build(),
        )
    } else {
        app::sensitive::Viewer::Guest
    })
}

fn render_response(
    snapshot: app::sensitive::Snapshot,
    trace: Vec<crate::trace_log::store::TraceEntry>,
) -> (http::StatusCode, axum::response::Html<String>) {
    let partial = crate::views::partials::SensitiveProof::builder()
        .snapshot(snapshot)
        .trace(trace)
        .build();
    (
        http::StatusCode::OK,
        axum::response::Html(partial.render().into_string()),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::UNIX_EPOCH;

    #[test]
    fn render_response_renders_sensitive_proof_target() {
        let response = render_response(
            app::sensitive::Snapshot::builder()
                .viewer(app::sensitive::ViewerState::guest())
                .maybe_token(None)
                .maybe_latest_sync(None)
                .maybe_integration_state(None)
                .records(Vec::new())
                .maybe_authorized_record(None)
                .access_events(Vec::new())
                .build(),
            Vec::new(),
        );

        assert_eq!(response.0, http::StatusCode::OK);
        assert!(response.1.0.contains("sensitive-proof-target"));
        assert!(!response.1.0.contains("UNIX_EPOCH"));
        assert!(UNIX_EPOCH <= std::time::SystemTime::now());
    }
}
