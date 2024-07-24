use local_ip_address::local_ip;
use scap::{
    capturer::{Area, Capturer, Options, Point, Size},
    frame::Frame,
};
use std::net::UdpSocket;

pub fn print_ip() {
    match local_ip() {
        Ok(ip) => println!("Hello from broadcaster at ip {ip}"),
        Err(_) => println!("Hello from broadcaster at unknown ip"),
    }
}

pub fn listen() {
    let mut buffer = [0u8; 500];
    let socket = UdpSocket::bind("192.168.1.52:3400").expect("couldn't bind to address");
    println!("Listening!");
    let (len, sender_address) = socket.recv_from(&mut buffer).expect("Fuck");
    println!("Done!");
    println!("{:?}", buffer);
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
