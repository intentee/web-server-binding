use std::net::TcpListener;

use web_server_binding::TcpPortReservation;

#[test]
fn pick_free_for_errors_when_address_already_in_use() {
    let blocker = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = blocker.local_addr().unwrap().port();

    let result = TcpPortReservation::pick_free_for(&format!("127.0.0.1:{port}"));

    assert!(result.is_err());
    drop(blocker);
}
