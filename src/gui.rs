use crate::hotkey::HotkeyManager;
use eframe;

pub struct Gui;

impl eframe::App for Gui {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello there");
            // ui.paragraph("General Kenobi");
            let hkm = HotkeyManager::default();
            let modifiers = ctx.input(|state| state.modifiers.to_owned());
            let keys = ctx.input(|state| state.keys_down.to_owned());
            let action = hkm.check_hotkey_actions(modifiers, keys);
            println!("{:?}", action);
        });
    }
}
