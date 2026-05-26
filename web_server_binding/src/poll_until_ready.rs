use std::fmt::Display;
use std::time::Duration;

use anyhow::Result;
use anyhow::anyhow;
use tokio::time::Instant;

use crate::readiness_polling_options::ReadinessPollingOptions;

pub async fn poll_until_ready<TAttempt>(
    ReadinessPollingOptions {
        poll_interval,
        readiness_timeout,
    }: ReadinessPollingOptions,
    target_description: impl Display,
    mut attempt: TAttempt,
) -> Result<()>
where
    TAttempt: AsyncFnMut() -> bool,
{
    let deadline = Instant::now() + readiness_timeout;

    loop {
        let remaining = deadline.saturating_duration_since(Instant::now());
        if remaining.is_zero() {
            return Err(anyhow!(
                "{target_description} did not become ready within {readiness_timeout:?}"
            ));
        }

        if let Ok(true) = tokio::time::timeout(remaining, attempt()).await {
            return Ok(());
        }

        let sleep_for: Duration =
            poll_interval.min(deadline.saturating_duration_since(Instant::now()));
        tokio::time::sleep(sleep_for).await;
    }
}
