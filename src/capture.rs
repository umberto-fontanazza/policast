use std::process::{Command, Child, Stdio};
use std::io::{self, BufRead, BufReader};
use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{self, Sender, Receiver};

fn get_ffmpeg_args(video_width: u32, video_height: u32, x: u32, y: u32) -> Vec<String> {
    let crop_filter = format!("crop={}:{}:{}:{}", video_width, video_height, x, y);
    let video_size = format!("{}x{}", video_width, video_height);

    let args = if cfg!(target_os = "macos") {
        vec![
            "-f", "avfoundation",
            "-r", "30",
            "-i", "2",
            "-video_size", &video_size,
            "-vf", &crop_filter,
            "-pix_fmt", "yuv420p",
            "-c:v", "libx264",
            "-t","10", //TODO: RIMUOVERE
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
            "-pix_fmt", "yuv420p",
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
            "-pix_fmt", "yuv420p",
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

fn capture_screen_area(video_width: u32, video_height: u32, x: u32, y: u32, stop_receiver: Receiver<()>) -> io::Result<Arc<Mutex<Child>>> {
    let ffmpeg_args = get_ffmpeg_args(video_width, video_height, x, y);

    let ffmpeg_command = Command::new("ffmpeg")
        .args(&ffmpeg_args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let ffmpeg_command = Arc::new(Mutex::new(ffmpeg_command));
    let ffmpeg_command_clone = Arc::clone(&ffmpeg_command);

    thread::spawn(move || {
        let stdout = ffmpeg_command_clone.lock().unwrap().stdout.take().unwrap();
        let stderr = ffmpeg_command_clone.lock().unwrap().stderr.take().unwrap();
        let stdout_reader = BufReader::new(stdout);
        let stderr_reader = BufReader::new(stderr);

        let mut stdout_lines = stdout_reader.lines();
        let mut stderr_lines = stderr_reader.lines();

        loop {
            if let Ok(_) = stop_receiver.try_recv() {
                println!("Stop signal received.");
                ffmpeg_command_clone.lock().unwrap().kill().unwrap();
                break;
            }

            if let Some(line) = stdout_lines.next() {
                if let Ok(line) = line {
                    println!("{}", line);
                }
            }

            if let Some(line) = stderr_lines.next() {
                if let Ok(line) = line {
                    eprintln!("{}", line);
                }
            }
        }
    });

    Ok(ffmpeg_command)
}

pub fn start_screen_capture(video_width: u32, video_height: u32, x: u32, y: u32) -> (Arc<Mutex<Child>>, Arc<Mutex<Sender<()>>>) {
    let (stop_sender, stop_receiver) = mpsc::channel();
    let stop_sender = Arc::new(Mutex::new(stop_sender));

    let capture_process = capture_screen_area(video_width, video_height, x, y, stop_receiver).unwrap();

    (capture_process, stop_sender)
}

pub fn stop_screen_capture(capture_process: Arc<Mutex<Child>>, stop_sender: Arc<Mutex<Sender<()>>>) {
    stop_sender.lock().unwrap().send(()).unwrap();
    capture_process.lock().unwrap().wait().unwrap();
    println!("Screen capture stopped.");
}
