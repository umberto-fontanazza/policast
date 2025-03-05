use super::Gui;

impl Gui {
    pub fn recording_controls(&mut self, ui: &mut egui::Ui) {
        if !self.video_caster.get_is_recording() {
            // Start recording when the button is pressed
            if ui.button("Start Recording").clicked() {
                if let Some((x, y, width, height)) = self.selected_area {
                    println!("x: {} y: {} width: {} height: {} ", x, y, width, height);
                    // Avvia la registrazione solo se è stata selezionata un'area
                    if let Err(e) = self.video_caster.start_recording(x, y, width, height) {
                        ui.label(format!("Error: {}", e)); // Show error if starting recording fails
                    }
                } else {
                    ui.label("Please select an area to record.");
                }
            }
        } else {
            // Stop recording when the button is pressed
            if ui.button("Stop Recording").clicked() {
                if let Err(e) = self.video_caster.stop_recording() {
                    ui.label(format!("Error: {}", e)); // Show error if stopping recording fails
                }
            }
        }
    }
}
