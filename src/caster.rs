use local_ip_address::local_ip;
use scap::{
    capturer::{Area, Capturer, Options, Point, Size},
    frame::Frame,
};
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

pub fn test_screencap() {
    let mut targets = scap::get_targets();
    let screen_one = targets.remove(0);

    // Create Options
    let options = Options {
        fps: 30,
        targets: vec![screen_one],
        show_cursor: true,
        show_highlight: true,
        excluded_targets: None,
        output_type: scap::frame::FrameType::BGRAFrame,
        output_resolution: scap::capturer::Resolution::_720p,
        source_rect: Some(Area {
            origin: Point { x: 0.0, y: 0.0 },
            size: Size {
                width: 2000.0,
                height: 1000.0,
            },
        }),
        ..Default::default()
    };

    // Create Recorder
    let mut capturer = Capturer::new(options);
    let mut buffer: Vec<Frame> = Vec::new();

    // Start Capture
    capturer.start_capture();

    for i in 0..(30 * 6) {
        println! {"Loop iter {i}"};
        let frame_or_err = capturer.get_next_frame();
        match frame_or_err {
            Ok(frame) => buffer.push(frame),
            Err(_) => break,
        }
    }

    // Stop Capture
    capturer.stop_capture();

    println!("Capture stopped");
}
