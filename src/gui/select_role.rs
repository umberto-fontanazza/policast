use super::{Gui, Route};

impl Gui {
    pub fn select_role(&mut self, ui: &mut egui::Ui) {
        if ui.button("Caster").clicked() {
            self.route_to(Route::CasterDeviceSelection);
            // Carica i dispositivi di cattura all'ingresso
            self.capturer.set_capture_devices();
        }
        if ui.button("Player").clicked() {
            self.route_to(Route::PlayerControls);
        }
    }
}
