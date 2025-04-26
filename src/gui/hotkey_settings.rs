use egui::Ui;

use super::{Gui, Role};

impl Gui {
    pub fn hotkey_settings(&mut self, ui: &mut Ui, _role: Role) {
        ui.label("Hotkey settings");
        self.hotkey.bindings().iter().for_each(|(action, combo)| {
            ui.horizontal(|ui| {
                ui.label(format!("{}", action));
                ui.label(format!("{:?} + {:?}", combo.0, combo.1));
                if ui.button("Clear binding").clicked() {
                    self.hotkey
                        .try_unbind(*action)
                        .expect("Should unbind the action");
                }
            });
        });
        self.hotkey.unbound_actions().iter().for_each(|action| {
            ui.horizontal(|ui| {
                ui.label(format!("{}", action));
                ui.label(format!("Unbinded"));
                if ui.button("Click to bind").clicked() {
                    self.hotkey.new_binding_mode(*action);
                }
            });
        });
    }
}
