# web_server_binding

TCP and Unix-socket binding helpers and readiness polling for web servers.

`WebServerBinding` lets a server accept either a `SocketAddr` it should bind
itself or an already-bound `TcpListener` it should take over. Companion helpers
wait until a TCP port or Unix socket is reachable.

## Usage

```rust
use std::net::SocketAddr;

use web_server_binding::ReadinessPollingOptions;
use web_server_binding::WebServerBinding;
use web_server_binding::wait_until_tcp_port_ready;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let address: SocketAddr = "127.0.0.1:8080".parse()?;
    let listener = WebServerBinding::from(address).into_tcp_listener()?;

    wait_until_tcp_port_ready("127.0.0.1", 8080, ReadinessPollingOptions::default()).await?;

    drop(listener);
    Ok(())
}
```

## License

Apache-2.0.
