use eframe::egui;
use egui::{ColorImage, TextureHandle, Ui};
use image::{ImageBuffer, Rgba};
use std::io::Read;
use std::process::{Command, Stdio};
use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;
const FPS: usize = 30;

#[derive(Default)]
pub struct Playback {
    pub is_playing: bool, // Playback status
    pub frame_buffer: Arc<Mutex<Option<ImageBuffer<Rgba<u8>, Vec<u8>>>>>, // Buffer for video frames
    video_link: Option<String>, // Private variable to store the video link
    texture: Option<TextureHandle>,
    decoder_stop: Option<(Sender<()>, JoinHandle<()>)>, // send a unit to stop the raw video decoder
}

impl Playback {
    pub fn new(ctx: &egui::Context) -> Self {
        Self {
            frame_buffer: Arc::new(Mutex::new(None)),
            texture: Some(ctx.load_texture(
                "video-frame",
                ColorImage::example(),
                Default::default(),
            )),
            ..Default::default()
        }
    }

    pub fn set_video_link(&mut self, link: String) {
        self.video_link = Some(link);
    }

    pub fn start_playback(&mut self) {
        if self.is_playing || self.video_link.is_none() {
            return;
        }

        self.is_playing = true;
        let frame_buffer = Arc::clone(&self.frame_buffer);
        let video_link = self.video_link.clone().unwrap(); // Use the set video link
        let (sender, receiver) = channel::<()>();

        let handle = thread::spawn(move || {
            let mut process = Command::new("ffmpeg")
                .args(&[
                    "-hide_banner",
                    "-loglevel",
                    "error",
                    "-i",
                    &video_link, // Input the video link
                    "-r",
                    format!("{FPS}").as_str(),
                    "-vf",
                    format!("fps={FPS},scale={WIDTH}:{HEIGHT},format=rgba").as_str(), // Set resolution and format
                    "-pix_fmt",
                    "rgba", // Set pixel format
                    "-f",
                    "rawvideo", // Set raw video output format
                    "-",        // Output to stdout
                ])
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()
                .expect("Failed to start FFmpeg");

            let mut stdout = process.stdout.take().expect("Failed to take stdout");
            let mut buffer = vec![0u8; WIDTH * HEIGHT * 4];

            // Continuously read the video frames from stdout
            while stdout.read_exact(&mut buffer).is_ok() {
                match receiver.try_recv() {
                    Ok(_) => {
                        break; // received signal to stop
                    }
                    Err(e) => match e {
                        std::sync::mpsc::TryRecvError::Empty => {}
                        std::sync::mpsc::TryRecvError::Disconnected => {
                            println!("This shouldn't happen");
                            break;
                        }
                    },
                }
                if let Ok(mut lock) = frame_buffer.lock() {
                    if let Some(frame) = ImageBuffer::from_raw(
                        u32::try_from(WIDTH).unwrap(),
                        u32::try_from(HEIGHT).unwrap(),
                        buffer.clone(),
                    ) {
                        *lock = Some(frame);
                    }
                }
            }
            process.kill().expect("Couldn't kill process");
        });
        self.decoder_stop = Some((sender, handle));
    }

    pub fn stop_playback(&mut self) {
        if self.is_playing {
            self.is_playing = false;
            self.frame_buffer
                .lock()
                .expect("Failed to lock frame buffer")
                .take(); // Clear the frame buffer
            let (sender, handle) = self.decoder_stop.take().unwrap();
            sender.send(()).unwrap();
            handle.join().unwrap();
            println!("Helper thread joined");
        }
    }

    // Function to display the current video frame in the GUI
    pub fn display_video_frame(&mut self, ui: &mut Ui, ctx: &egui::Context) {
        // Check if a frame is available and display it
        if let Some(frame) = self
            .frame_buffer
            .lock()
            .expect("Failed to lock frame buffer")
            .as_ref()
        {
            let texture = self.texture.as_mut().expect("Missing texture handle");
            let image = ColorImage::from_rgba_unmultiplied(
                [frame.width() as usize, frame.height() as usize],
                frame.as_raw(),
            );
            texture.set(image, Default::default());
            ui.image(&(*texture));
            ctx.request_repaint();
        } else {
            ui.label("No frame available");
        }
    }
}
