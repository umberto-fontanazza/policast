use crate::hotkey::ManagerState;
use crate::util::modifiers_to_string;
use egui::Ui;
use egui_extras::{Column, TableBuilder};

use super::Gui;

impl Gui {
    pub fn hotkey_settings(&mut self, ui: &mut Ui) {
        ui.label("Hotkey settings");
        let table = TableBuilder::new(ui)
            .striped(true)
            .column(Column::auto().at_least(100.0))
            .column(Column::auto())
            .column(Column::auto());

        table
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.strong("Action");
                });
                header.col(|ui| {
                    ui.strong("Current binding");
                });
                header.col(|ui| {
                    ui.strong("Edit");
                });
            })
            .body(|mut body| {
                self.hotkey
                    .bindings()
                    .iter()
                    .for_each(|(action, (modifiers, key))| {
                        body.row(20.0, |mut row| {
                            row.col(|ui| {
                                ui.label(format!("{}", action));
                            });
                            row.col(|ui| {
                                let mut key_combo = Vec::<String>::new();
                                if !modifiers.is_none() {
                                    key_combo.push(modifiers_to_string(modifiers));
                                }
                                key_combo.push(format!("{:?}", key));
                                ui.label(key_combo.join(" + "));
                            });
                            row.col(|ui| {
                                if ui.button("Clear binding").clicked() {
                                    self.hotkey
                                        .try_unbind(*action)
                                        .expect("Should unbind the action");
                                }
                            });
                        });
                    });

                let unbound_actions = self.hotkey.unbound_actions();
                unbound_actions.iter().for_each(|action| {
                    body.row(20.0, |mut row| {
                        row.col(|ui| {
                            ui.label(format!("{}", action));
                        });
                        row.col(|ui| match self.hotkey.state {
                            ManagerState::Binding(hotkey_action) if hotkey_action == *action => {
                                ui.label("Press key combination to bind");
                            }
                            _ => {
                                ui.label(format!("unbound"));
                            }
                        });
                        row.col(|ui| {
                            if ui.button("Click to bind").clicked() {
                                self.hotkey.new_binding_mode(*action);
                            }
                        });
                    });
                });
            });
    }
}
