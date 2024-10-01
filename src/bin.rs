use eframe::run_native;
use mpsc::capture::{list_screen_capture_devices, start_screen_capture, stop_screen_capture};
use mpsc::{capture, caster, gui, player};
use std::process::Command;
use std::thread;
use std::{env, io};

pub fn main() {
    let my_gui = gui::Gui;
    run_native(
        "ciao",
        eframe::NativeOptions::default(),
        Box::new(|_| Ok(Box::new(my_gui))),
    )
    .expect("something wrong");
}

fn do_something() {
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
    // List available capture devices
    let devices = list_screen_capture_devices().expect("Devices error");

    println!("Available screen capture devices:");
    for (index, name) in &devices {
        println!("[{}] {}", index, name);
    }

    // Select the first available device
    let target = devices.keys().next().expect("No devices found");

    let video_width = 1280;
    let video_height = 720;
    let x = 0;
    let y = 0;

    // Start screen capture
    let ffmpeg_command = start_screen_capture(video_width, video_height, x, y, target)
        .expect("ffmpeg start capture error");

    // Wait for user input to stop the capture
    println!("Press Enter to stop the screen capture...");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    // Stop screen capture
    stop_screen_capture(ffmpeg_command).expect("ffmpeg stop capture error");
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
