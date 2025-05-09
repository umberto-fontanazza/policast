use super::{Gui, Route};
use crate::{playback::PlaybackStatus, server::DiscoveryService};
use egui::TextEdit;

impl Gui {
    pub fn player_controls(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        if self.first_route_render & self.playback.discovery_service.is_none() {
            self.playback.discovery_service = Some(DiscoveryService::new());
            let casters = self
                .playback
                .discovery_service
                .as_mut()
                .map(|ds| ds.get_casters())
                .unwrap();
            self.playback.sources = casters;
        }

        ui.horizontal(|ui| {
            ui.heading("Player root");
            if ui.button("Settings").clicked() {
                self.route_to(Route::Settings);
            }
        });

        let mut to_be_player_url = self
            .playback
            .sources
            .iter()
            .map(|source| {
                let mut return_value: Option<String> = None;
                ui.horizontal(|ui| {
                    ui.label(format!("{source}"));
                    if ui.button("Watch this caster").clicked() {
                        return_value = Some(format!("http://{source}:3000/hls/output.m3u8"))
                    }
                });
                return_value
            })
            .collect::<Vec<Option<String>>>();
        let to_be_played_url = to_be_player_url
            .iter_mut()
            .find(|opt| opt.is_some())
            .map(|opt| opt.take().unwrap())
            .take();
        if to_be_played_url.is_some() {
            self.video_link = to_be_played_url.unwrap();
            self._playback_play();
        }

        if ui.button("Refresh sources").clicked() {
            let casters = self
                .playback
                .discovery_service
                .as_mut()
                .map(|ds| ds.get_casters())
                .unwrap();
            self.playback.sources = casters;
        }

        ui.label("Enter the M3U link to play the video:");

        // Text field to input the M3U link
        ui.add(TextEdit::singleline(&mut self.video_link).hint_text("Enter M3U playlist link"));

        ui.horizontal(|ui| {
            match self.playback.status() {
                PlaybackStatus::Stopped => {
                    if ui.button("Play").clicked() {
                        self._playback_play();
                    };
                }
                PlaybackStatus::Playing => {
                    if ui.button("Stop").clicked() {
                        self.playback.stop();
                    }
                    if ui.button("Pause").clicked() {
                        self.playback.pause();
                    }
                }
                PlaybackStatus::Paused => {
                    if ui.button("Stop").clicked() {
                        self.playback.stop();
                    }
                    if ui.button("Resume").clicked() {
                        self.playback.play();
                    }
                }
            };
        });

        self.playback.render(ui, ctx);
    }

    fn _playback_play(&mut self) {
        self.playback.set_video_link(self.video_link.clone());
        self.playback.play();
    }
}
