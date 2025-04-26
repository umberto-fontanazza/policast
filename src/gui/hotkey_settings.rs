use egui::Ui;
use strum::IntoEnumIterator;

use super::{Gui, Role};
use crate::hotkey::HotkeyAction;

impl Gui {
    pub fn hotkey_settings(&mut self, ui: &mut Ui, _role: Role) {
        ui.label("Hotkey settings");
        HotkeyAction::iter().for_each(|variant| {
            ui.horizontal(|ui| {
                ui.label(format!("{}", variant));
                ui.label("Change me button");
            });
        });
    }
}
