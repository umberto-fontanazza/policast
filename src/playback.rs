use std::time::{Duration, Instant};

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
    refresh_timestamp: Option<Instant>,
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
            refresh_timestamp: None,
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
                self.refresh_timestamp = None;
            }
        }
    }

    pub fn pause(&mut self) {
        replace_with_or_abort(&mut self.status, |status| match status {
            Status::Playing(decoder) => {
                self.refresh_timestamp = None;
                Status::Paused(decoder)
            }
            s => s,
        });
    }

    // Function to display the current video frame in the GUI
    pub fn display_video_frame(&mut self, ui: &mut Ui, ctx: &egui::Context) {
        match self.status {
            Status::Stopped => {
                ui.label("Video is not playing");
            }
            Status::Playing(_) => {
                let now = Instant::now();
                match self.refresh_timestamp {
                    Some(t) => {
                        let frame_period = Duration::from_millis(40); // (1000 ms / (FPS = 25)) = 40
                        if now.duration_since(t) > frame_period {
                            self.refresh_timestamp = Some(t + frame_period);
                            self.next_frame(ui, ctx);
                        } else {
                            ui.image(self.texture.as_ref().unwrap());
                            ctx.request_repaint();
                        }
                    }
                    None => {
                        self.refresh_timestamp = Some(now);
                        self.next_frame(ui, ctx);
                    }
                }
            }
            Status::Paused(_) => {
                ui.image(self.texture.as_ref().unwrap());
            }
        }
    }

    fn next_frame(&mut self, ui: &mut Ui, ctx: &egui::Context) {
        if let Status::Playing(decoder) = &mut self.status {
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
