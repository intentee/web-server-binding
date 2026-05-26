use web_server_binding::TcpPortReservation;

#[test]
fn binds_loopback_with_nonzero_port() {
    let reservation = TcpPortReservation::pick_free().unwrap();
    let local_addr = reservation.local_addr();

    assert!(local_addr.ip().is_loopback());
    assert!(local_addr.port() > 0);
}
