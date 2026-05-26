use std::net::TcpListener;

use web_server_binding::TcpPortReservation;

#[test]
fn into_local_addr_dropping_listener_releases_port_so_it_can_be_rebound() {
    let reservation = TcpPortReservation::pick_free().unwrap();
    let local_addr = reservation.into_local_addr_dropping_listener();

    let rebind_attempt = TcpListener::bind(local_addr).unwrap();

    assert_eq!(rebind_attempt.local_addr().unwrap(), local_addr);
}
