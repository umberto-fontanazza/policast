use super::Gui;
use crate::ffmpeg::take_screenshot;
use crate::gui::Route;
use egui::{Image, TextureHandle, Vec2};

impl Gui {
    pub fn device_selector(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.heading("Capture device selection");

        let devices = self.capturer.get_capture_devices().clone();

        if self.first_route_render & self.thumbnail_textures.is_none() {
            let textures = devices
                .iter()
                .map(|device| {
                    ctx.load_texture(
                        format!("device-{}", device.handle()),
                        take_screenshot(&device.handle()),
                        Default::default(),
                    )
                })
                .collect::<Vec<TextureHandle>>();
            self.thumbnail_textures = Some(textures);
        }

        ui.horizontal(|ui| {
            devices.iter().for_each(|device| {
                let t = self
                    .thumbnail_textures
                    .as_ref()
                    .unwrap()
                    .iter()
                    .find(|t| t.name().eq(format!("device-{}", device.handle()).as_str()))
                    .expect(format!("Texture with hanlde {} not found", device.handle()).as_str());
                let selected = self
                    .capturer
                    .get_selected_device()
                    .is_some_and(|opt_val| opt_val.handle() == device.handle());
                //TODO: fit to relative size?
                let img_button = egui::ImageButton::new(
                    Image::new(t).fit_to_exact_size(Vec2::new(320.0, 180.0)),
                )
                .selected(selected);
                let button = egui::Button::new(device.name()).selected(selected);

                ui.vertical(|ui| {
                    if ui.add(img_button).clicked() || ui.add(button).clicked() {
                        self.capturer
                            .set_selected_device(Some(device.handle().to_string()))
                            .expect("Couldn't set the selected device");
                        return self.route_to(Route::CasterControls);
                    }
                });
            });
        });
    }
}
