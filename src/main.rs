mod capture;
use capture::{start_screen_capture, stop_screen_capture};
use std::thread;

fn main() {
    let video_width = 800;
    let video_height = 600;
    let x = 200;
    let y = 200;

    let (capture_process, stop_sender) = start_screen_capture(video_width, video_height, x, y);
    println!("Screen capture started successfully.");

    //TODO: STOP NON FUNZIONA
    thread::sleep(std::time::Duration::from_secs(2));
    stop_screen_capture(capture_process, stop_sender);
}