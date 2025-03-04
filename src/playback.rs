use eframe::egui;
use egui::{ColorImage, Ui};
use image::{ImageBuffer, Rgba};
use std::io::Read;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Default)]
pub struct Playback {
    pub is_playing: bool, // Playback status
    pub frame_buffer: Arc<Mutex<Option<ImageBuffer<Rgba<u8>, Vec<u8>>>>>, // Buffer for video frames
    video_link: Option<String>, // Private variable to store the video link
}

impl Playback {
    // Constructor to initialize the playback instance
    pub fn new() -> Self {
        Self {
            is_playing: false,
            frame_buffer: Arc::new(Mutex::new(None)),
            video_link: None, // Initialize video link as None
        }
    }

    // Function to set the video link
    pub fn set_video_link(&mut self, link: String) {
        self.video_link = Some(link);
    }

    // Function to start video playback
    pub fn start_playback(&mut self) {
        if self.is_playing || self.video_link.is_none() {
            return; // Do nothing if already playing or if no link is set
        }

        self.is_playing = true;
        let frame_buffer = Arc::clone(&self.frame_buffer);
        let video_link = self.video_link.clone().unwrap(); // Use the set video link

        // Spawn a new thread to run FFmpeg for video processing
        thread::spawn(move || {
            let mut process = Command::new("ffmpeg")
                .args(&[
                    "-i",
                    &video_link, // Input the video link
                    "-r",
                    "30",
                    "-vf",
                    "fps=30,scale=1280:720,format=rgba", // Set resolution and format
                    "-pix_fmt",
                    "rgba", // Set pixel format
                    "-f",
                    "rawvideo", // Set raw video output format
                    "-",        // Output to stdout
                ])
                .stdout(Stdio::piped())
                .spawn()
                .expect("Failed to start FFmpeg");

            let mut stdout = process.stdout.take().expect("Failed to take stdout");
            let mut buffer = vec![0u8; 1280 * 720 * 4]; // Assume 1280x720 RGBA format for the frame
            let mut frame_count = 0; // Counter to track number of frames

            // Continuously read the video frames from stdout
            while stdout.read_exact(&mut buffer).is_ok() {
                frame_count += 1; // Increment frame counter
                                  // Lock the frame buffer and update with the new frame
                if let Ok(mut lock) = frame_buffer.lock() {
                    if let Some(frame) = ImageBuffer::from_raw(1280, 720, buffer.clone()) {
                        *lock = Some(frame);
                    }
                }
            }

            println!("Total frames processed: {}", frame_count); // Print total frames at the end
        });
    }

    // Function to stop video playback
    pub fn stop_playback(&mut self) {
        if self.is_playing {
            self.is_playing = false; // Set playback status to false
            self.frame_buffer
                .lock()
                .expect("Failed to lock frame buffer")
                .take(); // Clear the frame buffer
        }
    }

    // Function to display the current video frame in the GUI
    pub fn display_video_frame(&self, ui: &mut Ui, ctx: &egui::Context) {
        // Check if a frame is available and display it
        if let Some(frame) = self
            .frame_buffer
            .lock()
            .expect("Failed to lock frame buffer")
            .as_ref()
        {
            let texture = ctx.load_texture(
                "video_frame",
                ColorImage::from_rgba_unmultiplied(
                    [frame.width() as usize, frame.height() as usize],
                    frame.as_raw(),
                ),
                Default::default(),
            );
            ui.image(&texture);
        } else {
            ui.label("No frame available"); // Display a placeholder message if no frame
        }
    }
}
