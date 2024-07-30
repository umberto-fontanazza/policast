use std::process::{Command, Child, Stdio};
use std::io::{self, Write};

fn get_ffmpeg_args(video_width: u32, video_height: u32, x: u32, y: u32) -> Vec<String> {
    let crop_filter = format!("crop={}:{}:{}:{}", video_width, video_height, x, y);
    let video_size = format!("{}x{}", video_width, video_height);

    let args = if cfg!(target_os = "macos") {
        vec![
            "-f", "avfoundation",
            "-r", "25",
            "-i", "2",
            "-video_size", &video_size,
            "-vf", &crop_filter,
            "-c:v", "libx264",
            "-f", "hls",
            "-hls_time", "2",
            "-hls_list_size", "0",
            "-hls_flags", "delete_segments",
            "-hls_segment_filename", "target/output_%03d.ts",
            "target/output.m3u8",
        ]
    } else if cfg!(target_os = "windows") {
        vec![
            "-f", "gdigrab",
            "-framerate", "30",
            "-i", "desktop",
            "-vf", &crop_filter,
            "-c:v", "libx264",
            "-f", "hls",
            "-hls_time", "2",
            "-hls_list_size", "0",
            "-hls_flags", "delete_segments",
            "-hls_segment_filename", "target/output_%03d.ts",
            "target/output.m3u8",
        ]
    } else if cfg!(target_os = "linux") {
        vec![
            "-f", "x11grab",
            "-r", "30",
            "-s", &video_size,
            "-i", ":0.0",
            "-vf", &crop_filter,
            "-c:v", "libx264",
            "-f", "hls",
            "-hls_time", "2",
            "-hls_list_size", "0",
            "-hls_flags", "delete_segments",
            "-hls_segment_filename", "target/output_%03d.ts",
            "target/output.m3u8",
        ]
    } else {
        panic!("Unsupported operating system");
    };

    args.into_iter().map(String::from).collect()
}

pub fn start_screen_capture(video_width: u32, video_height: u32, x: u32, y: u32) -> io::Result<Child> {
    let ffmpeg_args = get_ffmpeg_args(video_width, video_height, x, y);

    let ffmpeg_command = Command::new("ffmpeg")
        .args(&ffmpeg_args)
        .stdin(Stdio::piped())
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
