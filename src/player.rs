use local_ip_address::local_ip;
use std::net::{IpAddr, UdpSocket};

pub fn send_udp_packet(destination_address: IpAddr, destination_port: u16) {
    let socket = UdpSocket::bind((local_ip().unwrap(), 3400)).expect("Couldn't bind to port ");
    let buffer = [1u8; 80];
    socket
        .connect((destination_address, destination_port))
        .expect("Woooo bad error in connect!");
    socket.send(&buffer).expect("Couldn't send");
}
