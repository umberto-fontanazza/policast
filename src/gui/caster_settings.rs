use super::{Gui, Route};

impl Gui {
    pub fn caster_settings(&mut self, ui: &mut egui::Ui) {
        if self.first_route_render {
            let path = self.settings.try_borrow_mut().unwrap().get_save_dir();
            self.text_buffer = path.to_str().unwrap().into();
        }
        if ui.button("Back").clicked() {
            self.route_to(Route::CasterRoot);
        }
        ui.label("Edit save location: ");
        ui.add(egui::TextEdit::singleline(&mut self.text_buffer));
        if ui.button("Apply changes").clicked() {
            self.settings
                .try_borrow_mut()
                .unwrap()
                .set_save_dir(&self.text_buffer)
        }
    }
}
