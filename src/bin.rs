use mpsc::{capture, caster, player};
use std::env;
use std::thread;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let peer_role = &args[1];
    if peer_role == "caster" {
        // caster::setup();
        setup_stream();
    } else if peer_role == "player" {
        player::setup();
    } else {
        println!("Unrecognised command line arg");
    }
}

fn setup_stream() {
    let video_width = 800;
    let video_height = 600;
    let x = 200;
    let y = 200;

    let (capture_process, stop_sender) =
        capture::start_screen_capture(video_width, video_height, x, y);
    println!("Screen capture started successfully.");

    //TODO: STOP NON FUNZIONA
    thread::sleep(std::time::Duration::from_secs(2));
    capture::stop_screen_capture(capture_process, stop_sender);
}
