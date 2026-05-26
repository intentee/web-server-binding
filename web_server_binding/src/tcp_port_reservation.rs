use std::net::SocketAddr;
use std::net::TcpListener;

use anyhow::Context;
use anyhow::Result;

#[derive(Debug)]
pub struct TcpPortReservation {
    listener: TcpListener,
    local_addr: SocketAddr,
}

impl TcpPortReservation {
    pub fn pick_free() -> Result<Self> {
        Self::pick_free_for("127.0.0.1:0")
    }

    pub fn pick_free_for(bind_addr: &str) -> Result<Self> {
        let listener = TcpListener::bind(bind_addr)
            .with_context(|| format!("failed to bind reservation listener at {bind_addr}"))?;
        let local_addr_result = listener.local_addr();
        Self::try_new(listener, local_addr_result)
    }

    fn try_new(
        listener: TcpListener,
        local_addr_result: std::io::Result<SocketAddr>,
    ) -> Result<Self> {
        let local_addr =
            local_addr_result.context("failed to read reservation listener local addr")?;
        Ok(Self {
            listener,
            local_addr,
        })
    }

    #[must_use]
    pub fn local_addr(&self) -> SocketAddr {
        self.local_addr
    }

    #[must_use]
    pub fn into_tcp_listener(self) -> TcpListener {
        self.listener
    }

    #[must_use]
    pub fn into_local_addr_dropping_listener(self) -> SocketAddr {
        self.local_addr
    }
}

#[cfg(test)]
mod tests {
    use std::io;
    use std::net::TcpListener;

    use super::TcpPortReservation;

    #[test]
    fn try_new_succeeds_with_ok_local_addr() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let local_addr = listener.local_addr().unwrap();

        let reservation = TcpPortReservation::try_new(listener, Ok(local_addr)).unwrap();

        assert_eq!(reservation.local_addr(), local_addr);
    }

    #[test]
    fn try_new_propagates_io_error_with_context() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();

        let err = TcpPortReservation::try_new(listener, Err(io::Error::other("boom"))).unwrap_err();

        assert!(
            err.to_string()
                .contains("failed to read reservation listener local addr")
        );
    }
}
