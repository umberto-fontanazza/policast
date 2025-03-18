use local_ip_address::local_ip;
use std::net::{IpAddr, UdpSocket};

pub fn setup() {
    let ip = local_ip().expect("Couldn't retrieve local ip");
    let setup_socket = UdpSocket::bind((ip, 3401)).expect("Couldn't bind");
    let buffer = [1; 1];
    setup_socket
        .connect(("192.168.1.52", 3401))
        .expect("Couldn't connect");
    setup_socket.send(&buffer).expect("Couldn't send");
    listen();
}

pub fn send_udp_packet(destination_address: IpAddr, destination_port: u16) {
    let socket = UdpSocket::bind((local_ip().unwrap(), 3400)).expect("Couldn't bind to port ");
    let buffer = [1u8; 80];
    socket
        .connect((destination_address, destination_port))
        .expect("Woooo bad error in connect!");
    socket.send(&buffer).expect("Couldn't send");
}

pub fn listen() {
    let my_ip = local_ip().expect("Couldn't get local ip");
    let mut buffer = [0u8; 80];
    let socket = UdpSocket::bind((my_ip, 3400)).expect("couldn't bind to address");
    loop {
        let (_, sender_address) = socket
            .recv_from(&mut buffer)
            .expect("Sciagura e dannazione");
        println!("Packet received from {sender_address}");
        println!("{:?}", buffer);
    }
}
