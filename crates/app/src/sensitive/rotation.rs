use super::*;

impl Service {
    #[tracing::instrument(skip(self))]
    pub async fn run_key_rotation_pass(
        &self,
        limit: usize,
    ) -> Result<sensitive::KeyRotationRun> {
        let started_at = self.clock.now();
        let progress = self
            .repo
            .rotate_ciphertext_to_active_key(limit, started_at)
            .await?;
        let finished_at = self.clock.now();
        let outcome = if progress.rows_failed == 0 {
            sensitive::RotationOutcome::Success
        } else {
            sensitive::RotationOutcome::Failed
        };
        let run = sensitive::KeyRotationRun::builder()
            .active_key_id(progress.active_key_id)
            .outcome(outcome)
            .rows_scanned(progress.rows_scanned)
            .rows_rewrapped(progress.rows_rewrapped)
            .rows_already_current(progress.rows_already_current)
            .rows_failed(progress.rows_failed)
            .detail(progress.detail)
            .started_at(started_at)
            .finished_at(finished_at)
            .build();
        self.repo.record_key_rotation_run(&run).await?;
        tracing::info!(
            target: "demo.sensitive",
            outcome = %run.outcome,
            rows_scanned = run.rows_scanned,
            rows_rewrapped = run.rows_rewrapped,
            rows_failed = run.rows_failed,
            active_key_id = %run.active_key_id,
            "sensitive key rotation pass completed",
        );
        Ok(run)
    }
}

pub(super) fn rotation_detail_text(message: impl Into<String>) -> sensitive::DetailText {
    super::bounded_detail_text(message, "sensitive key rotation status")
}
