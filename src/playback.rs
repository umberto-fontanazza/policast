use crate::decoder::Decoder;
use eframe::egui;
use egui::{ColorImage, TextureHandle, Ui};
use image::{ImageBuffer, Rgba};
use replace_with::replace_with_or_abort;

pub const WIDTH: usize = 1280;
pub const HEIGHT: usize = 720;
pub const FPS: usize = 30;

pub type Frame = ImageBuffer<Rgba<u8>, Vec<u8>>;

#[derive(PartialEq)]
pub enum PlaybackStatus {
    Stopped,
    Playing,
    Paused,
}

enum Status {
    Stopped,
    Playing(Decoder),
    Paused(Decoder),
}

pub struct Playback {
    status: Status,
    video_link: Option<String>, // Private variable to store the video link
    texture: Option<TextureHandle>,
}

impl Playback {
    pub fn new(ctx: &egui::Context) -> Self {
        Self {
            status: Status::Stopped,
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

    pub fn status(&self) -> PlaybackStatus {
        match self.status {
            Status::Stopped => PlaybackStatus::Stopped,
            Status::Playing(_) => PlaybackStatus::Playing,
            Status::Paused(_) => PlaybackStatus::Paused,
        }
    }

    pub fn play(&mut self) {
        replace_with_or_abort(&mut self.status, |status| match status {
            Status::Stopped => Status::Playing(Decoder::new(
                self.video_link
                    .as_ref()
                    .expect("video_url must be set before playing")
                    .clone(),
            )),
            Status::Paused(decoder) => Status::Playing(decoder),
            playing => playing,
        });
    }

    pub fn stop(&mut self) {
        match self.status {
            Status::Stopped => {
                println!("Playback is already stopped")
            }
            _ => {
                self.status = Status::Stopped;
            }
        }
    }

    pub fn pause(&mut self) {
        replace_with_or_abort(&mut self.status, |status| match status {
            Status::Playing(decoder) => Status::Paused(decoder),
            s => s,
        });
    }

    // Function to display the current video frame in the GUI
    pub fn display_video_frame(&mut self, ui: &mut Ui, ctx: &egui::Context) {
        match self.status {
            Status::Stopped => {
                ui.label("Video is not playing");
            }
            Status::Playing(ref decoder) => {
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
            Status::Paused(_) => {
                ui.image(self.texture.as_ref().unwrap());
            }
        }
    }
}
