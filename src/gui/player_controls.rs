use super::Gui;
use crate::playback::PlaybackStatus;
use egui::TextEdit;

impl Gui {
    pub fn player_controls(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.label("Enter the M3U link to play the video:");

        // Text field to input the M3U link
        ui.add(TextEdit::singleline(&mut self.video_link).hint_text("Enter M3U link"));

        if ui
            .button(if self.playback.status() == PlaybackStatus::Paused {
                "Resume"
            } else {
                "Play"
            })
            .clicked()
        {
            if !self.video_link.is_empty() {
                self.playback.set_video_link(self.video_link.clone()); // Set the video link in the playback instance
                self.playback.play(); // Start the video playback
            } else {
                ui.label("Enter a valid link!");
            }
        }

        if ui.button("Stop").clicked() {
            self.playback.stop();
        }

        if ui.button("Pause").clicked() {
            self.playback.pause();
        }

        self.playback.render(ui, ctx);
    }
}
