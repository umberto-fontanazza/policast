use super::Gui;
use egui::{Align, Layout};

impl Gui {
    pub fn device_selector(&mut self, ui: &mut egui::Ui) {
        ui.with_layout(Layout::left_to_right(Align::LEFT), |ui| {
            self.video_caster
                .get_capture_devices()
                .into_iter()
                .for_each(|(index, name)| {
                    if ui.button(name).clicked() {
                        self.video_caster
                            .set_selected_device(index)
                            .expect("Couldn't set the selected device");
                    }
                });
        });
    }
}
