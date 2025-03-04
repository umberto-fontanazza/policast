use eframe::run_native;
use mpsc::gui;
use mpsc::settings::Settings;
use refbox::RefBox;
use std::process::Command;

pub fn main() {
    let settings = RefBox::new(Settings::default());
    run_native(
        "ciao",
        eframe::NativeOptions::default(),
        Box::new(|cc| Ok(Box::new(gui::Gui::new(cc, &settings)))),
    )
    .expect("something wrong");
}
