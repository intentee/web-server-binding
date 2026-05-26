use std::net::SocketAddr;
use std::net::TcpListener;

use anyhow::Context;
use anyhow::Result;

pub enum WebServerBinding {
    BindTcp(SocketAddr),
    ListenTcp(TcpListener),
}

impl WebServerBinding {
    pub fn into_tcp_listener(self) -> Result<TcpListener> {
        match self {
            Self::BindTcp(addr) => {
                TcpListener::bind(addr).with_context(|| format!("failed to bind {addr}"))
            }
            Self::ListenTcp(listener) => Ok(listener),
        }
    }
}

impl From<SocketAddr> for WebServerBinding {
    fn from(addr: SocketAddr) -> Self {
        Self::BindTcp(addr)
    }
}

impl From<TcpListener> for WebServerBinding {
    fn from(listener: TcpListener) -> Self {
        Self::ListenTcp(listener)
    }
}
