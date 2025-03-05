use super::Gui;
use egui::{Align, Layout};

impl Gui {
    pub fn device_selector(&mut self, ui: &mut egui::Ui) {
        let selected_device = self.video_caster.get_selected_device();
        ui.with_layout(Layout::left_to_right(Align::LEFT), |ui| {
            self.video_caster
                .get_capture_devices()
                .into_iter()
                .for_each(|(index, name)| {
                    let button = egui::Button::new(&name).selected(
                        selected_device
                            .as_ref()
                            .is_some_and(|device| device.eq(&index)),
                    );
                    if ui.add(button).clicked() {
                        self.video_caster
                            .set_selected_device(index)
                            .expect("Couldn't set the selected device");
                    }
                });
        });
    }
}
