use regex::Regex;
use std::collections::HashMap;
use std::io::{self, Write};
use std::process::{Child, Command, Stdio};

fn list_screen_capture_devices_macos() -> io::Result<HashMap<String, String>> {
    let output = Command::new("ffmpeg")
        .args(["-f", "avfoundation", "-list_devices", "true", "-i", ""])
        .stderr(Stdio::piped())
        .spawn()?
        .wait_with_output()?;

    let stderr = String::from_utf8_lossy(&output.stderr);
    let mut devices = HashMap::new();

    let re = Regex::new(r"\[(\d+)\]\s*(.*Capture screen.*)").unwrap();

    for line in stderr.lines() {
        if let Some(caps) = re.captures(line) {
            if let Some(index) = caps.get(1).map(|i| i.as_str().to_string()) {
                let name = caps.get(2).map_or("", |m| m.as_str()).to_string();
                devices.insert(index, name);
            }
        }
    }

    if devices.is_empty() {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "No screen capture devices found",
        ))
    } else {
        Ok(devices)
    }
}

fn list_screen_capture_devices_windows() -> io::Result<HashMap<String, String>> {
    unimplemented!()
}

fn list_screen_capture_devices_linux() -> io::Result<HashMap<String, String>> {
    unimplemented!()
}

// Funzione wrapper per elencare i dispositivi di cattura dello schermo su qualsiasi OS
pub fn list_screen_capture_devices() -> io::Result<HashMap<String, String>> {
    if cfg!(target_os = "macos") {
        list_screen_capture_devices_macos()
    } else if cfg!(target_os = "windows") {
        list_screen_capture_devices_windows()
    } else if cfg!(target_os = "linux") {
        list_screen_capture_devices_linux()
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            "Unsupported operating system",
        ))
    }
}

fn get_ffmpeg_args(
    video_width: u32,
    video_height: u32,
    x: u32,
    y: u32,
    target: &str,
) -> Vec<String> {
    let crop_filter = format!("crop={}:{}:{}:{}", video_width, video_height, x, y);
    let video_size = format!("{}x{}", video_width, video_height);

    let args = if cfg!(target_os = "macos") {
        vec![
            "-f",
            "avfoundation",
            "-r",
            "25",
            "-i",
            target,
            "-video_size",
            &video_size,
            "-vf",
            &crop_filter,
            "-c:v",
            "libx264",
            "-f",
            "hls",
            "-hls_time",
            "2",
            "-hls_list_size",
            "0",
            "-hls_flags",
            "delete_segments",
            "-hls_segment_filename",
            "target/output_%03d.ts",
            "target/output.m3u8",
        ]
    } else if cfg!(target_os = "windows") {
        vec![
            "-f",
            "gdigrab",
            "-framerate",
            "30",
            "-i",
            target,
            "-vf",
            &crop_filter,
            "-c:v",
            "libx264",
            "-f",
            "hls",
            "-hls_time",
            "2",
            "-hls_list_size",
            "0",
            "-hls_flags",
            "delete_segments",
            "-hls_segment_filename",
            "target/output_%03d.ts",
            "target/output.m3u8",
        ]
    } else if cfg!(target_os = "linux") {
        vec![
            "-f",
            "x11grab",
            "-r",
            "30",
            "-s",
            &video_size,
            "-i",
            target,
            "-vf",
            &crop_filter,
            "-c:v",
            "libx264",
            "-f",
            "hls",
            "-hls_time",
            "2",
            "-hls_list_size",
            "0",
            "-hls_flags",
            "delete_segments",
            "-hls_segment_filename",
            "target/output_%03d.ts",
            "target/output.m3u8",
        ]
    } else {
        panic!("Unsupported operating system");
    };

    args.into_iter().map(String::from).collect()
}

pub fn start_screen_capture(
    video_width: u32,
    video_height: u32,
    x: u32,
    y: u32,
    target: &str,
) -> io::Result<Child> {
    let ffmpeg_args = get_ffmpeg_args(video_width, video_height, x, y, target);

    let ffmpeg_command = Command::new("ffmpeg")
        .args(&ffmpeg_args)
        .stdin(Stdio::piped())
        .stderr(Stdio::null()) // Redirect stderr to null to prevent ffmpeg messages from appearing in the terminal
        .spawn()?;

    println!("Screen capture started successfully.");

    Ok(ffmpeg_command)
}

pub fn stop_screen_capture(mut ffmpeg_command: Child) -> io::Result<()> {
    if let Some(stdin) = ffmpeg_command.stdin.as_mut() {
        writeln!(stdin, "q").expect("Failed to write to stdin");
        ffmpeg_command.wait()?;
        println!("Screen capture stopped successfully.");
    }
    Ok(())
}
