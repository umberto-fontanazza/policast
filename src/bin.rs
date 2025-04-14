use eframe::run_native;
use mpsc::gui;

pub fn main() {
    run_native(
        "ciao",
        eframe::NativeOptions::default(),
        Box::new(|cc| Ok(Box::new(gui::Gui::new(cc)))),
    )
    .expect("something wrong");
}
