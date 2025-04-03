use crate::util;

use super::Gui;
use egui::{ColorImage, Context, Image, Rect, Vec2};

impl Gui {
    pub fn preview(&mut self, ui: &mut egui::Ui, ctx: &Context) -> Rect {
        if self.first_route_render {
            self.preview_texture =
                Some(ctx.load_texture("preview", ColorImage::default(), Default::default()));
            if !self.capturer.is_recording() {
                self.capturer
                    .start_recording()
                    .expect("Capture should start");
            }
        }

        let texture = self
            .preview_texture
            .as_mut()
            .expect("Texture should be set");
        if self.capturer.is_recording {
            let frame_receiver = self.capturer.frame_receiver();
            let frame = frame_receiver.recv().unwrap();
            util::update_texture(texture, frame);
            ctx.request_repaint();
        }
        ui.add(
            Image::new(&(*texture))
                .maintain_aspect_ratio(true)
                .fit_to_fraction(Vec2::new(1.0, 1.0)),
        )
        .rect
    }
}
