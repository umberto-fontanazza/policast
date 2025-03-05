use super::Gui;
use egui::TextEdit;

impl Gui {
    pub fn player_controls(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.label("Enter the M3U link to play the video:");

        // Text field to input the M3U link
        ui.add(TextEdit::singleline(&mut self.video_link).hint_text("Enter M3U link"));

        // Button to start playback
        if ui.button("Play").clicked() {
            if !self.video_link.is_empty() {
                self.playback.set_video_link(self.video_link.clone()); // Set the video link in the playback instance
                self.playback.start_playback(); // Start the video playback
            } else {
                ui.label("Enter a valid link!");
            }
        }

        // Button to stop playback
        if ui.button("Stop").clicked() {
            self.playback.stop_playback();
        }

        // Show the video status and current frame
        self.playback.display_video_frame(ui, ctx); // Use the persistent playback instance to display the frame
    }
}
