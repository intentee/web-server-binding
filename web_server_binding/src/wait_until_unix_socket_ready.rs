use std::path::Path;

use anyhow::Result;
use tokio::net::UnixStream;

use crate::poll_until_ready::poll_until_ready;
use crate::readiness_polling_options::ReadinessPollingOptions;

pub async fn wait_until_unix_socket_ready(
    socket_path: &Path,
    options: ReadinessPollingOptions,
) -> Result<()> {
    poll_until_ready(
        options,
        format!("unix socket {}", socket_path.display()),
        async || UnixStream::connect(socket_path).await.is_ok(),
    )
    .await
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::wait_until_unix_socket_ready;
    use crate::readiness_polling_options::ReadinessPollingOptions;

    #[tokio::test]
    async fn returns_ok_when_socket_already_bound() {
        let tempdir = tempfile::tempdir().unwrap();
        let socket_path = tempdir.path().join("ready.sock");
        let _listener = tokio::net::UnixListener::bind(&socket_path).unwrap();

        wait_until_unix_socket_ready(&socket_path, ReadinessPollingOptions::default())
            .await
            .unwrap();
    }

    #[tokio::test(start_paused = true)]
    async fn errors_when_socket_never_becomes_ready() {
        let tempdir = tempfile::tempdir().unwrap();
        let socket_path = tempdir.path().join("never-created.sock");

        let result = wait_until_unix_socket_ready(
            &socket_path,
            ReadinessPollingOptions {
                readiness_timeout: Duration::from_millis(50),
                ..ReadinessPollingOptions::default()
            },
        )
        .await;

        assert!(result.is_err());
    }
}
