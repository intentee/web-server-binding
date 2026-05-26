use std::net::SocketAddr;

use web_server_binding::WebServerBinding;

#[test]
fn binds_loopback_when_given_socket_addr() {
    let addr: SocketAddr = "127.0.0.1:0".parse().unwrap();
    let web_server_binding: WebServerBinding = addr.into();

    let listener = web_server_binding.into_tcp_listener().unwrap();

    let bound = listener.local_addr().unwrap();
    assert!(bound.ip().is_loopback());
    assert!(bound.port() > 0);
}
