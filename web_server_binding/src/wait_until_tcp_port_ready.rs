use anyhow::Result;
use tokio::net::TcpStream;

use crate::poll_until_ready::poll_until_ready;
use crate::readiness_polling_options::ReadinessPollingOptions;

pub async fn wait_until_tcp_port_ready(
    host: &str,
    port: u16,
    options: ReadinessPollingOptions,
) -> Result<()> {
    poll_until_ready(options, format!("tcp port {host}:{port}"), async || {
        TcpStream::connect((host, port)).await.is_ok()
    })
    .await
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::wait_until_tcp_port_ready;
    use crate::readiness_polling_options::ReadinessPollingOptions;
    use crate::tcp_port_reservation::TcpPortReservation;

    #[tokio::test]
    async fn returns_ok_when_port_already_bound() {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();

        wait_until_tcp_port_ready("127.0.0.1", port, ReadinessPollingOptions::default())
            .await
            .unwrap();
    }

    #[tokio::test(start_paused = true)]
    async fn errors_when_port_never_becomes_ready() {
        let port = TcpPortReservation::pick_free()
            .unwrap()
            .into_local_addr_dropping_listener()
            .port();

        let result = wait_until_tcp_port_ready(
            "127.0.0.1",
            port,
            ReadinessPollingOptions {
                readiness_timeout: Duration::from_millis(100),
                ..ReadinessPollingOptions::default()
            },
        )
        .await;

        assert!(result.is_err());
    }
}
