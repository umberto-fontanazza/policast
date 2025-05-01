use eframe::run_native;
use mpsc::gui;
use mpsc::settings::APP_NAME;

pub fn main() {
    run_native(
        APP_NAME,
        eframe::NativeOptions::default(),
        Box::new(|cc| Ok(Box::new(gui::Gui::new(cc)))),
    )
    .expect("something wrong");
}
