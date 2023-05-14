use dns::{build_query, TYPE_A};
use std::net::{Ipv4Addr, UdpSocket};

fn main() {
    let message = build_query("example.com".to_string(), TYPE_A);
    let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 0)).expect("Cannot connect.");
    socket.send_to(&message, "8.8.8.8:53").unwrap();
    let mut response_buffer = [0u8; 1024];
    socket
        .recv(&mut response_buffer)
        .expect("Unable to read response.");

    println!("{response_buffer:?}");
}
