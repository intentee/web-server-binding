use std::net::TcpListener;

use web_server_binding::TcpPortReservation;

#[test]
fn second_bind_fails_while_reservation_alive() {
    let reservation = TcpPortReservation::pick_free().unwrap();
    let port = reservation.local_addr().port();

    let second_attempt = TcpListener::bind(("127.0.0.1", port));

    assert!(second_attempt.is_err());
    drop(reservation);
}
