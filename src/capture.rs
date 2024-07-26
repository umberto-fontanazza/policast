use std::io::Write;
use std::process::{Command, Stdio, Child, ChildStdin};
use scap::{
    capturer::{Point, Area, Size, Capturer, Options},
    frame::Frame,
};

pub struct CaptureContext {
    capturer: Capturer,
    ffmpeg_stdin: ChildStdin,
    ffmpeg_process: Child,
}

impl CaptureContext {
    pub fn init_capture(x: f64, y: f64, width: f64, height: f64) -> Result<Self, Box<dyn std::error::Error>> {
        if !scap::is_supported() {
            println!("❌ Platform not supported");
            return Err("Platform not supported".into());
        }
        println!("✅ Platform supported");

        if !scap::has_permission() {
            println!("❌ Permission not granted. Requesting permission...");
            if !scap::request_permission() {
                println!("❌ Permission denied");
                return Err("Permission denied".into());
            }
        }
        println!("✅ Permission granted");

        let targets = scap::get_targets();
        println!("🎯 Targets: {:?}", targets);

        let options = Options {
            fps: 25,
            targets,
            show_cursor: true,
            show_highlight: true,
            excluded_targets: None,
            output_type: scap::frame::FrameType::BGRAFrame,
            output_resolution: scap::capturer::Resolution::Captured,
            source_rect: Some(Area {
                origin: Point { x, y },
                size: Size {
                    width: width / 2.0,
                    height: height / 2.0,
                },
            }),
            ..Default::default()
        };

        let capturer = Capturer::new(options);

        let mut ffmpeg = Command::new("ffmpeg")
            .args(&[
                "-f", "rawvideo",
                "-pix_fmt", "bgra",
                "-s", &format!("{}x{}", width as u32, height as u32),
                "-r", "25",
                "-i", "-",
                "-c:v", "libx264",
                "-pix_fmt", "yuv420p",
                "-b:v", "1M",
                "target/output.mp4",
            ])
            .stdin(Stdio::piped())
            .spawn()?;

        let ffmpeg_stdin = ffmpeg.stdin.take().expect("Failed to open stdin");

        Ok(Self {
            capturer,
            ffmpeg_stdin,
            ffmpeg_process: ffmpeg,
        })
    }

    pub fn stop_capture(mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.capturer.stop_capture();
        self.ffmpeg_stdin.flush()?;
        drop(self.ffmpeg_stdin);
        let output = self.ffmpeg_process.wait_with_output()?;

        if !output.status.success() {
            return Err(format!("FFmpeg error: {}", String::from_utf8_lossy(&output.stderr)).into());
        }

        println!("Video saved to target/output.mp4");
        Ok(())
    }

    pub fn capture_frames(&mut self, stop_signal: std::sync::mpsc::Receiver<()>) -> Result<(), Box<dyn std::error::Error>> {
        self.capturer.start_capture();
        loop {
            if let Ok(_) = stop_signal.try_recv() {
                break;
            }
            match self.capturer.get_next_frame() {
                Ok(Frame::BGRA(frame)) => {
                    self.ffmpeg_stdin.write_all(&frame.data)?;
                }
                Err(e) => eprintln!("Failed to capture frame: {}", e),
                _ => eprintln!("Unsupported frame type"),
            }
        }
        Ok(())
    }

}

pub fn start_capture_thread(x: f64, y: f64, width: f64, height: f64) -> Result<(std::thread::JoinHandle<()>, std::sync::mpsc::Sender<()>), Box<dyn std::error::Error>> {
    let (tx, rx) = std::sync::mpsc::channel();

    let handle = std::thread::spawn(move || {
        let mut context = CaptureContext::init_capture(x, y, width, height).unwrap();
        context.capture_frames(rx).unwrap();
        context.stop_capture().expect("Failed to stop capture");
    });

    Ok((handle, tx))
}

pub fn send_stop_signal(tx: std::sync::mpsc::Sender<()>) {
    tx.send(()).unwrap();
}