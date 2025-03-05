use super::Gui;

impl Gui {
    pub fn device_selector(&mut self, ui: &mut egui::Ui) {
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
    }
}
