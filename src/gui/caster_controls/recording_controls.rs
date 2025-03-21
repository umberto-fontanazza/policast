use super::Gui;

impl Gui {
    pub fn recording_controls(&mut self, ui: &mut egui::Ui) {
        if !self.capturer.get_is_recording() {
            if ui.button("Start Recording").clicked() {
                if let Some((x, y, width, height)) = self.capturer.selected_area {
                    println!("x: {} y: {} width: {} height: {} ", x, y, width, height);
                    // Avvia la registrazione solo se è stata selezionata un'area
                    if let Err(e) = self.capturer.start_recording() {
                        ui.label(format!("Error: {}", e)); // Show error if starting recording fails
                    }
                } else {
                    ui.label("Please select an area to record.");
                }
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
