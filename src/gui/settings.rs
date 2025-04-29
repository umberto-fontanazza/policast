use super::{Gui, Role};

impl Gui {
    pub fn settings(&mut self, ui: &mut egui::Ui) {
        if self.first_route_render {
            let path = self.settings.borrow().get_save_dir();
            self.text_buffer = path.to_str().unwrap().into();
        }
        ui.horizontal(|ui| {
            ui.heading("Caster settings");
            if ui.button("Back").clicked() {
                return self.route_back();
            }
        });

        if self.role.as_ref().is_some_and(|role| *role == Role::Caster) {
            ui.label("Edit save location: ");
            ui.horizontal(|ui| {
                ui.add(egui::TextEdit::singleline(&mut self.text_buffer));
                if ui.button("Apply changes").clicked() {
                    self.settings.borrow_mut().set_save_dir(&self.text_buffer);
                }
            });
        }
        self.hotkey_settings(ui);
    }
}
