use egui::Ui;

use super::{Gui, Role};

impl Gui {
    pub fn hotkey_settings(&mut self, ui: &mut Ui, _role: Role) {
        ui.label("Hotkey settings");
        //FIXME: deterministic ordering of tuples
        self.hotkey.bindings().iter().for_each(|(action, combo)| {
            ui.horizontal(|ui| {
                ui.label(format!("{}", action));
                ui.label(format!("{:?} + {:?}", combo.0, combo.1));
            });
        });
        self.hotkey.unbinded_actions().iter().for_each(|action| {
            ui.horizontal(|ui| {
                ui.label(format!("{}", action));
                ui.label(format!("Unbinded"));
            });
        });
    }
}
