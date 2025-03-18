use local_ip_address::local_ip;
use std::net::{IpAddr, UdpSocket};

const SETUP_PORT: u16 = 3401;
const STREAM_PORT: u16 = 3400;
const BUFFER_LEN: usize = 80;

pub fn setup() {
    let ip = local_ip().expect("Couldn't get own ip address");
    let mut buffer = [0; BUFFER_LEN];
    let setup_socket = UdpSocket::bind((ip, SETUP_PORT))
        .expect(format!("Couldn't bind to port {SETUP_PORT}").as_str());
    let (_, player_address) = setup_socket.recv_from(&mut buffer).expect("UDP recv error");
    let player_ip = player_address.ip();

    stream(player_ip);
}

fn stream(player_ip: IpAddr) {
    let ip = local_ip().expect("Couldn't get own ip address");
    let mut buffer = [0; BUFFER_LEN];
    let stream_socket = UdpSocket::bind((ip, STREAM_PORT))
        .expect(format!("Couldn't bind to port {STREAM_PORT}").as_str());
    stream_socket
        .connect((player_ip, STREAM_PORT))
        .expect("Connection to player stream port failed");
    for i in 0..20 {
        buffer[0] = i;
        stream_socket
            .send(&buffer)
            .expect("Datagram creation error");
    }
}

pub fn print_ip() {
    match local_ip() {
        Ok(ip) => println!("Hello from broadcaster at ip {ip}"),
        Err(_) => println!("Hello from broadcaster at unknown ip"),
    }
}
