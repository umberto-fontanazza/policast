use mpsc::{capture, caster, player};
use std::env;
use std::process::Command;
use std::thread;
use mpsc::capture::{start_screen_capture, stop_screen_capture};

pub fn main() {
    let args: Vec<String> = env::args().collect();
    // if args.len() == 1 {
    //     ffmpeg_list_devices();
    //     return;
    // }
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
        let x = 0;
        let y = 0;

        let ffmpeg_command = start_screen_capture(video_width, video_height, x, y)
            .expect("Failed to start screen capture");

        // Simulate some work
        std::thread::sleep(std::time::Duration::from_secs(10));

        stop_screen_capture(ffmpeg_command).expect("Failed to stop screen capture");
}

fn ffmpeg_is_installed() -> bool {
    let out = Command::new("ffmpeg")
        .arg("-version")
        .output()
        .expect("Error in running child process");
    out.status.success()
}

fn ffmpeg_list_devices() -> Result<Vec<String>, ()> {
    if cfg!(target_os = "macos") {
        let out = Command::new("ffmpeg")
            // .args(["-version"])
            .args([
                // "-hide_banner",
                // "-loglevel",
                // "error",
                "-f",
                "avfoundation",
                "-list_devices",
                "true",
                "-i",
                "\"\"",
            ])
            .output()
            .expect("Couldn't run command");
        println!("{}", String::from_utf8(out.stderr).expect("Parse error"));
    } else if cfg!(target_os = "windows") {
        unimplemented!();
    } else if cfg!(target_os = "linux") {
        unimplemented!();
    } else {
        println!("Platform not supported!");
    }
    Err(())
}
