use crate::{gui::Route, util};

use super::Gui;
use egui::{ColorImage, Context, Image, Vec2};

impl Gui {
    pub fn preview(&mut self, ui: &mut egui::Ui, ctx: &Context) {
        if self.first_route_render {
            self.preview_texture =
                Some(ctx.load_texture("preview", ColorImage::default(), Default::default()));
            if !self.capturer.is_recording() {
                self.capturer.start_recording();
            }
        }
        if ui.button("Back to device selection").clicked() {
            self.capturer.stop_recording();
            self.capturer
                .set_selected_device(None)
                .expect("Couldn't clear device selection");
            self.route_to(Route::CasterDeviceSelection);
        }

        match self.capturer.is_recording {
            true => {
                let texture = self
                    .preview_texture
                    .as_mut()
                    .expect("Texture should be set");
                let frame_receiver = self.capturer.frame_receiver();
                let frame = frame_receiver.recv().unwrap();
                util::update_texture(texture, frame);
                ui.add(
                    Image::new(&(*texture))
                        .maintain_aspect_ratio(true)
                        .fit_to_fraction(Vec2::new(1.0, 1.0)),
                );
                ctx.request_repaint();
            }
            false => {
                ui.label("Capturer not recoding");
            }
        }
    }
}
