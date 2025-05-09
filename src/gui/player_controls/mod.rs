mod caster_selector;
use super::{Gui, Route};
use crate::playback::PlaybackStatus;

impl Gui {
    pub fn player_controls(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.horizontal(|ui| {
            ui.heading("Player root");
            if ui.button("Settings").clicked() {
                self.route_to(Route::Settings);
            }
        });

        if self.playback.status() == PlaybackStatus::Stopped {
            self.caster_selector(ui, ctx);
        }

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
