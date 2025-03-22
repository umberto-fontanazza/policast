use super::Gui;

impl Gui {
    pub fn recording_controls(&mut self, ui: &mut egui::Ui) {
        if !self.capturer.get_is_recording() {
            if ui.button("Start Recording").clicked() {
                //TODO: error management
                self.capturer.start_recording();
            }
        } else {
            if ui.button("Stop Recording").clicked() {
                if let Err(e) = self.capturer.stop_recording() {
                    ui.label(format!("Error: {}", e)); // Show error if stopping recording fails
                }
            }
        }
    }
}
