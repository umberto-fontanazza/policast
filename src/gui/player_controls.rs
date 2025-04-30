use super::{Gui, Route};
use crate::playback::PlaybackStatus;
use egui::TextEdit;

impl Gui {
    pub fn player_controls(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.horizontal(|ui| {
            ui.heading("Player root");
            if ui.button("Settings").clicked() {
                self.route_to(Route::Settings);
            }
        });

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
                        self.playback.play(None);
                    }
                }
            };
        });

        self.playback.render(ui, ctx);
    }

    fn _playback_play(&mut self) {
        self.playback.set_video_link(self.video_link.clone());
        let save_path = self
            .settings
            .borrow()
            .player_save_dir
            .as_ref()
            .map(|path| path.clone());
        self.playback.play(save_path);
    }
}
