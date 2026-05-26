use std::net::TcpListener;

use web_server_binding::WebServerBinding;

#[test]
fn returns_pre_bound_listener_unchanged() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let expected_addr = listener.local_addr().unwrap();
    let web_server_binding: WebServerBinding = listener.into();

    let returned_listener = web_server_binding.into_tcp_listener().unwrap();

    assert_eq!(returned_listener.local_addr().unwrap(), expected_addr);
}
