use super::{Gui, Route};

impl Gui {
    pub fn select_role(&mut self, ui: &mut egui::Ui) {
        ui.heading("Select your role");

        if ui.button("Caster").clicked() {
            self.capturer.set_capture_devices();
            return self.route_to(Route::CasterDeviceSelection);
        }
        if ui.button("Player").clicked() {
            return self.route_to(Route::PlayerControls);
        }
    }
}
