use crate::ffmpeg::take_screenshot;

use super::Gui;
use egui::TextureHandle;

impl Gui {
    pub fn device_selector(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        let devices = self.video_caster.get_capture_devices();
        let selected_device = self.video_caster.get_selected_device();

        if self.first_route_render & self.thumbnail_textures.is_none() {
            let textures = devices
                .iter()
                .map(|(index, _)| {
                    ctx.load_texture(
                        format!("thumb-device-{index}"),
                        take_screenshot(&index),
                        Default::default(),
                    )
                })
                .collect::<Vec<TextureHandle>>();
            self.thumbnail_textures = Some(textures);
        }

        ui.horizontal(|ui| {
            devices.into_iter().for_each(|(index, name)| {
                let parsed_index = index.parse::<usize>().expect("Should parse an usize") - 1; // -1 to get it 0 based
                let t = &(self.thumbnail_textures.as_ref().unwrap()[parsed_index]);
                let selected = selected_device
                    .as_ref()
                    .is_some_and(|device| device.eq(&index));
                let img_button = egui::ImageButton::new(t).selected(selected);
                let button = egui::Button::new(&name).selected(selected);

                ui.vertical(|ui| {
                    if ui.add(img_button).clicked() || ui.add(button).clicked() {
                        self.video_caster
                            .set_selected_device(index)
                            .expect("Couldn't set the selected device");
                    }
                });
            });
        });
    }
}
