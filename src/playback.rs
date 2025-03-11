use crate::decoder::Decoder;
use eframe::egui;
use egui::{ColorImage, TextureHandle, Ui};
use image::{ImageBuffer, Rgba};

pub const WIDTH: usize = 1280;
pub const HEIGHT: usize = 720;
pub const FPS: usize = 30;

pub type Frame = ImageBuffer<Rgba<u8>, Vec<u8>>;

enum PlaybackStatus {
    Stopped,
    Playing(Decoder),
}

pub struct Playback {
    status: PlaybackStatus,
    video_link: Option<String>, // Private variable to store the video link
    texture: Option<TextureHandle>,
}

impl Playback {
    pub fn new(ctx: &egui::Context) -> Self {
        Self {
            status: PlaybackStatus::Stopped,
            video_link: None,
            texture: Some(ctx.load_texture(
                "video-frame",
                ColorImage::example(),
                Default::default(),
            )),
        }
    }

    pub fn set_video_link(&mut self, link: String) {
        self.video_link = Some(link);
    }

    pub fn start_playback(&mut self) {
        match self.status {
            PlaybackStatus::Stopped => {
                let video_url = self
                    .video_link
                    .as_ref()
                    .expect("video_url must be set before playing")
                    .clone();
                self.status = PlaybackStatus::Playing(Decoder::new(video_url));
            }
            PlaybackStatus::Playing(_) => {
                println!("Playback is already playing")
            }
        }
    }

    pub fn stop_playback(&mut self) {
        match self.status {
            PlaybackStatus::Stopped => {
                println!("Playback is already stopped")
            }
            PlaybackStatus::Playing(_) => {
                self.status = PlaybackStatus::Stopped;
            }
        }
    }

    // Function to display the current video frame in the GUI
    pub fn display_video_frame(&mut self, ui: &mut Ui, ctx: &egui::Context) {
        match self.status {
            PlaybackStatus::Stopped => {
                ui.label("Video is not playing");
            }
            PlaybackStatus::Playing(ref decoder) => {
                let frame: Frame = decoder.recv().expect("Failed to receive frame");
                let texture = self.texture.as_mut().expect("Missing texture handle");
                let image = ColorImage::from_rgba_unmultiplied(
                    [frame.width() as usize, frame.height() as usize],
                    frame.as_raw(),
                );
                texture.set(image, Default::default());
                ui.image(&(*texture));
                ctx.request_repaint();
            }
        }
    }
}
