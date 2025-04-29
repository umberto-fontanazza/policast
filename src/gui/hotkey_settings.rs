use crate::hotkey::ManagerState;
use crate::util::modifiers_to_string;
use egui::Ui;

use super::Gui;

impl Gui {
    pub fn hotkey_settings(&mut self, ui: &mut Ui) {
        ui.label("Hotkey settings");
        self.hotkey
            .bindings()
            .iter()
            .for_each(|(action, (modifiers, key))| {
                ui.horizontal(|ui| {
                    ui.label(format!("{}", action));
                    let mut key_combo = Vec::<String>::new();
                    if !modifiers.is_none() {
                        key_combo.push(modifiers_to_string(modifiers));
                    }
                    key_combo.push(format!("{:?}", key));
                    ui.label(key_combo.join(" + "));
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
