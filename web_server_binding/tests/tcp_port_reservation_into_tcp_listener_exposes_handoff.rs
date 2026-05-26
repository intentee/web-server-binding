use web_server_binding::TcpPortReservation;

#[test]
fn into_tcp_listener_returns_the_listener_with_the_reserved_addr() {
    let reservation = TcpPortReservation::pick_free().unwrap();
    let expected_addr = reservation.local_addr();

    let listener = reservation.into_tcp_listener();

    assert_eq!(listener.local_addr().unwrap(), expected_addr);
}
