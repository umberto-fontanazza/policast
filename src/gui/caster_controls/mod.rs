mod area_selector;
mod recording_controls;
use super::{Gui, Route};

impl Gui {
    pub fn caster_controls(&mut self, ui: &mut egui::Ui) {
        if ui.button("Go to settings").clicked() {
            self.route_to(Route::CasterSettings);
        }
        ui.label("Available screen capture devices:");

        // Display the list of available devices
        let device_list = self.video_caster.get_device_list();
        ui.label(&device_list);

        // Automatically select the first device if none is selected
        if self.video_caster.get_selected_device().is_none() {
            if let Some(first_device) = self.video_caster.get_first_device() {
                if let Err(e) = self.video_caster.set_selected_device(first_device.clone()) {
                    ui.label(format!("Error: {}", e));
                } else {
                    ui.label(format!("Automatically selected device: {}", first_device));
                }
            } else {
                ui.label("No screen capture devices found.");
            }
        }

        self.recording_controls(ui);

        // Area selection UI
        if ui.button("Start Area Selection").clicked() {
            self.selecting_area = true;
            self.start_point = None;
            self.end_point = None;
            self.selected_area = None;
        }

        // Handle the area selection
        self.area_selector(ui);
    }
}
