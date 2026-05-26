use std::net::TcpListener;

use web_server_binding::WebServerBinding;

#[test]
fn into_tcp_listener_errors_when_bind_addr_already_in_use() {
    let blocker = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = blocker.local_addr().unwrap();
    let web_server_binding: WebServerBinding = addr.into();

    let result = web_server_binding.into_tcp_listener();

    assert!(result.is_err());
    drop(blocker);
}
