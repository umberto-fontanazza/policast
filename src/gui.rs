use crate::hotkey::HotkeyManager;
use eframe;

#[derive(Default)]
enum Route {
    #[default]
    SelectRole,
    CasterRoot,
    PlayerRoot,
}

#[derive(Default)]
pub struct Gui {
    route: Route,
}

impl eframe::App for Gui {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.route {
                Route::SelectRole => {
                    ui.heading("Select your role");
                    if ui.button("Caster").clicked() {
                        self.route = Route::CasterRoot;
                    }
                    if ui.button("Player").clicked() {
                        self.route = Route::PlayerRoot;
                    }
                }
                Route::CasterRoot => {
                    ui.heading("Caster root");
                    // let hkm = HotkeyManager::default();
                    // let modifiers = ctx.input(|state| state.modifiers.to_owned());
                    // let keys = ctx.input(|state| state.keys_down.to_owned());
                    // let action = hkm.check_hotkey_actions(modifiers, keys);
                    // println!("{:?}", action);
                }
                Route::PlayerRoot => {
                    ui.heading("Player root");
                }
            }
        });
    }
}
