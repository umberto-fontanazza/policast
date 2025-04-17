use crate::crop::CropFilter;
use crate::settings::{CAPTURE_FPS, HLS_LIST_SIZE, HLS_SEGMENT_DURATION};
use crate::util;
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

pub fn spawn_raw_encoder(width: usize, height: usize, filename: &str) -> Child {
    let size = format!("{width}x{height}");
    let filename = format!("{filename}.mp4");
    let framerate = CAPTURE_FPS.to_string();
    Command::new("ffmpeg")
        .args([
            "-f", "rawvideo", "-pix_fmt", "rgba", "-s", &size, "-r", &framerate, "-i", "-",
            &filename,
        ])
        .stdin(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .expect("Should spawn an ffmpeg subprocess")
}

fn get_ffmpeg_args(
    resolution: Option<usize>,
    crop: Option<CropFilter>,
    target: &str,
    save_dir: &Path,
) -> Vec<String> {
    let framerate = CAPTURE_FPS.to_string();
    let size_filter = match resolution {
        Some(height) => format!("scale=trunc(oh*a/2)*2:{},", height),
        None => "".to_string(),
    };
    let crop_filter = match crop {
        Some(ref crop) => format!("crop={}:{}:{}:{},", crop.width, crop.height, crop.x, crop.y),
        None => "".to_string(),
    };
    // let video_size = format!("{}x{}", crop.width, crop.height);
    let segment_path = save_dir.join("output_%03d.ts");
    let playlist_path = save_dir.join("output.m3u8");
    let frames_per_segment = (HLS_SEGMENT_DURATION * CAPTURE_FPS).to_string();
    let hls_list_size = HLS_LIST_SIZE.to_string();

    let input_args = if cfg!(target_os = "macos") {
        vec![
            "-f",
            "avfoundation",
            "-r",
            &framerate,
            "-i",
            target,
            // "-video_size",
            // &video_size,
        ]
    } else if cfg!(target_os = "windows") {
        vec!["-f", "gdigrab", "-framerate", &framerate, "-i", target]
    } else if cfg!(target_os = "linux") {
        vec![
            "-f", "x11grab", "-r", &framerate, // "-s", &video_size,
            "-i", target,
        ]
    } else {
        panic!("Unsupported operating system");
    };
    let complex_filter = format!("[0:v]{}{}split=2[out1][out2]", size_filter, crop_filter);
    let args = vec![
        "-filter_complex",
        &complex_filter,
        "-map",
        "[out1]",
        "-c:v",
        "libx264",
        // EXPERIMENTAL
        "-preset",
        "ultrafast",
        "-tune",
        "zerolatency",
        // END EXPERIMENTAL
        "-g",
        &frames_per_segment,
        "-keyint_min",
        &frames_per_segment,
        "-sc_threshold",
        "0",
        "-f",
        "hls",
        "-hls_time",
        "2",
        "-hls_list_size",
        &hls_list_size,
        "-hls_flags",
        "delete_segments",
        "-hls_segment_filename",
        segment_path.to_str().expect("Couldn't stringify path"),
        playlist_path.to_str().expect("Couldn't stringify path"),
        "-map",
        "[out2]",
        "-pix_fmt",
        "rgba",
        "-f",
        "rawvideo",
        "-",
    ];

    input_args
        .into_iter()
        .chain(args.into_iter())
        .filter(|arg| !arg.is_empty())
        .map(String::from)
        .collect()
}

pub fn start_screen_capture(
    resolution: Option<usize>,
    crop: Option<CropFilter>,
    target: &str,
    save_dir: &Path,
) -> io::Result<Child> {
    let ffmpeg_args = get_ffmpeg_args(resolution, crop, target, save_dir);

    let ffmpeg_command = Command::new("ffmpeg")
        .args(&ffmpeg_args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        // .stderr(Stdio::null()) // Redirect stderr to null to prevent ffmpeg messages from appearing in the terminal
        .spawn()?;

    println!("Screen capture started successfully.");

    Ok(ffmpeg_command)
}

pub fn stop_screen_capture(mut subprocess: Child, buffer: &mut [u8]) {
    writeln!(subprocess.stdin.take().unwrap(), "q").expect("Should write \"q\" to stdin");
    util::read_while_full(subprocess.stdout.as_mut().unwrap(), buffer);
    let _ = subprocess.wait();
    println!("ffmpeg subprocess exited gracefully");
}

pub fn ffmpeg_is_installed() -> bool {
    let out = Command::new("ffmpeg")
        .arg("-version")
        .output()
        .expect("Error in running child process");
    out.status.success()
}

pub fn take_screenshot(source: &str) -> ColorImage {
    // let downsample_factor = "10";
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
                // "-vf",
                // &format!("scale=iw/{downsample_factor}:ih/{downsample_factor}"),
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
