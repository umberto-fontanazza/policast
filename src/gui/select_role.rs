use super::{Gui, Route};

impl Gui {
    pub fn select_role(&mut self, ui: &mut egui::Ui) {
        if ui.button("Caster").clicked() {
            self.route_to(Route::CasterRoot);
            // Carica i dispositivi di cattura all'ingresso
            if let Err(e) = self.capturer.set_capture_devices() {
                ui.label(format!("Error: {}", e));
            }
        }
        if ui.button("Player").clicked() {
            self.route_to(Route::PlayerRoot);
        }
    }
}
