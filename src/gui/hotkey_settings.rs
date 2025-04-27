use egui::Ui;

use crate::hotkey::ManagerState;
use crate::util::modifiers_to_string;

use super::{Gui, Role};

impl Gui {
    pub fn hotkey_settings(&mut self, ui: &mut Ui, _role: Role) {
        ui.label("Hotkey settings");
        self.hotkey
            .bindings()
            .iter()
            .for_each(|(action, (modifiers, key))| {
                ui.horizontal(|ui| {
                    ui.label(format!("{}", action));
                    ui.label(format!("{} + {:?}", modifiers_to_string(modifiers), key));
                    if ui.button("Clear binding").clicked() {
                        self.hotkey
                            .try_unbind(*action)
                            .expect("Should unbind the action");
                    }
                });
            });
        let unbound_actions = self.hotkey.unbound_actions();
        unbound_actions.iter().for_each(|action| {
            ui.horizontal(|ui| {
                ui.label(format!("{}", action));
                match self.hotkey.state {
                    ManagerState::Binding(hotkey_action) if hotkey_action == *action => {
                        ui.label("Press key combination to bind");
                    }
                    _ => {
                        ui.label(format!("unbound"));
                        if ui.button("Click to bind").clicked() {
                            self.hotkey.new_binding_mode(*action);
                        }
                    }
                }
            });
        });
    }
}
