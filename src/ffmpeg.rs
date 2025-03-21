use egui::ColorImage;
use image::{load_from_memory_with_format, RgbImage};
use regex::Regex;
use std::collections::HashMap;
use std::io::{self, Write};
use std::path::Path;
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
    Ok(HashMap::from([(
        "desktop".to_string(),
        "desktop".to_string(),
    )]))
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
    save_dir: &Path,
) -> Vec<String> {
    let crop_filter = format!("crop={}:{}:{}:{}", video_width, video_height, x, y);
    let video_size = format!("{}x{}", video_width, video_height);
    let segment_path = save_dir.join("output_%03d.ts");
    let playlist_path = save_dir.join("output.m3u8");

    let os_args = if cfg!(target_os = "macos") {
        vec![
            "-f",
            "avfoundation",
            "-r",
            "25",
            "-i",
            target,
            "-video_size",
            &video_size,
        ]
    } else if cfg!(target_os = "windows") {
        vec!["-f", "gdigrab", "-framerate", "30", "-i", target]
    } else if cfg!(target_os = "linux") {
        vec!["-f", "x11grab", "-r", "30", "-s", &video_size, "-i", target]
    } else {
        panic!("Unsupported operating system");
    };
    let args = vec![
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
        segment_path.to_str().expect("Couldn't stringify path"),
        playlist_path.to_str().expect("Couldn't stringify path"),
    ];

    os_args
        .into_iter()
        .chain(args.into_iter())
        .map(String::from)
        .collect()
}

pub fn start_screen_capture(
    video_width: u32,
    video_height: u32,
    x: u32,
    y: u32,
    target: &str,
    save_dir: &Path,
) -> io::Result<Child> {
    let ffmpeg_args = get_ffmpeg_args(video_width, video_height, x, y, target, save_dir);

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

pub fn ffmpeg_is_installed() -> bool {
    let out = Command::new("ffmpeg")
        .arg("-version")
        .output()
        .expect("Error in running child process");
    out.status.success()
}

pub fn take_screenshot(source: &str) -> ColorImage {
    let downsample_factor = "10";
    if cfg!(target_os = "macos") {
        let o = Command::new("ffmpeg")
            .args([
                "-f",
                "avfoundation",
                "-framerate",
                "1",
                "-i",
                source,
                "-frames:v",
                "1",
                "-vf",
                &format!("scale=iw/{downsample_factor}:ih/{downsample_factor}"),
                "-pix_fmt",
                "rgba",
                "-f",
                "image2pipe",
                "-vcodec",
                "bmp",
                "-",
            ])
            .output()
            .unwrap()
            .stdout;
        bmp_to_image(o)
    } else {
        unimplemented!();
    }
}

fn bmp_to_image(bmp_data: Vec<u8>) -> ColorImage {
    let img = load_from_memory_with_format(&bmp_data, image::ImageFormat::Bmp)
        .expect("Couldn't parse BPM to image");
    let rgb_img: RgbImage = img.to_rgb8();
    let (width, height) = rgb_img.dimensions();
    let pixels = rgb_img.into_raw();
    let color_image = ColorImage::from_rgb([width as usize, height as usize], &pixels);
    color_image
}
