use super::Gui;
use crate::ffmpeg::take_screenshot;
use crate::gui::Route;
use egui::TextureHandle;

impl Gui {
    pub fn device_selector(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        let devices = self.capturer.get_capture_devices().clone();
        let selected_device = self.capturer.get_selected_device();

        if self.first_route_render & self.thumbnail_textures.is_none() {
            let textures = devices
                .iter()
                .map(|device| {
                    ctx.load_texture(
                        format!("thumb-device-{}", device.name()),
                        take_screenshot(&device.handle()),
                        Default::default(),
                    )
                })
                .collect::<Vec<TextureHandle>>();
            self.thumbnail_textures = Some(textures);
        }

        ui.horizontal(|ui| {
            devices.iter().for_each(|device| {
                let parsed_index = device
                    .handle()
                    .parse::<usize>()
                    .expect("Should parse an usize")
                    - 1; // -1 to get it 0 based
                let t = &(self.thumbnail_textures.as_ref().unwrap()[parsed_index]);
                let selected = selected_device
                    .as_ref()
                    .is_some_and(|d1| d1.eq(&device.handle()));
                let img_button = egui::ImageButton::new(t).selected(selected);
                let button = egui::Button::new(device.name()).selected(selected);

                ui.vertical(|ui| {
                    if ui.add(img_button).clicked() || ui.add(button).clicked() {
                        self.capturer
                            .set_selected_device(device.handle().to_string())
                            .expect("Couldn't set the selected device");
                        self.route_to(Route::CasterControls);
                    }
                });
            });
        });
    }
}
