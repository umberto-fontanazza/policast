use std::{cell::RefCell, net::Ipv4Addr, rc::Rc, time::Instant};

use crate::{
    alias::Frame,
    decoder::Decoder,
    server::DiscoveryService,
    settings::{Settings, CAPTURE_PERIOD},
    util,
};
use eframe::egui;
use egui::{ColorImage, TextureHandle, Ui};
use replace_with::replace_with_or_abort;

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
    pub video_link: String, // Private variable to store the video link
    texture: Option<TextureHandle>,
    refresh_timestamp: Option<Instant>,
    settings: Rc<RefCell<Settings>>,
    pub discovery_service: Option<DiscoveryService>,
    pub sources: Vec<Ipv4Addr>,
}

impl Playback {
    pub fn new(ctx: &egui::Context, settings: Rc<RefCell<Settings>>) -> Self {
        Self {
            status: Status::Stopped,
            video_link: "".to_string(),
            texture: Some(ctx.load_texture(
                "video-frame",
                ColorImage::example(),
                Default::default(),
            )),
            refresh_timestamp: None,
            settings,
            discovery_service: None,
            sources: vec![],
        }
    }

    pub fn status(&self) -> PlaybackStatus {
        match self.status {
            Status::Stopped => PlaybackStatus::Stopped,
            Status::Playing(_) => PlaybackStatus::Playing,
            Status::Paused(_) => PlaybackStatus::Paused,
        }
    }

    pub fn play(&mut self) {
        let s = self.settings.borrow();
        let save_path = if s.player_save_enabled {
            s.player_save_dir.clone()
        } else {
            None
        };
        replace_with_or_abort(&mut self.status, |status| match status {
            Status::Stopped => Status::Playing(Decoder::new(self.video_link.clone(), save_path)),
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
    pub fn render(&mut self, ui: &mut Ui, ctx: &egui::Context) {
        match self.status {
            Status::Stopped => (),
            Status::Playing(_) => {
                let now = Instant::now();
                match self.refresh_timestamp {
                    Some(t) => {
                        if now.duration_since(t) > CAPTURE_PERIOD {
                            self.refresh_timestamp = Some(t + CAPTURE_PERIOD);
                            if self.next_frame(ui, ctx).is_err() {
                                self.stop();
                            }
                        } else {
                            ui.image(self.texture.as_ref().unwrap());
                            ctx.request_repaint();
                        }
                    }
                    None => {
                        self.refresh_timestamp = Some(now);
                        self.next_frame(ui, ctx).expect("Decoder should send the frame to be rendered on the frame sender before closing the channel")
                    }
                }
            }
            Status::Paused(_) => {
                ui.image(self.texture.as_ref().unwrap());
            }
        }
    }

    fn next_frame(&mut self, ui: &mut Ui, ctx: &egui::Context) -> Result<(), ()> {
        if let Status::Playing(decoder) = &mut self.status {
            let frame: Frame = match decoder.recv() {
                Ok(frame) => frame,
                Err(_) => {
                    return Err(());
                }
            };
            let texture = self.texture.as_mut().expect("Missing texture handle");
            util::update_texture(texture, frame);
            ui.image(&(*texture));
            ctx.request_repaint();
            Ok(())
        } else {
            Err(())
        }
    }
}
