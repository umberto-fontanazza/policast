use crate::util;

use super::Gui;
use crate::settings::SERVER_PORT;
use egui::{ColorImage, Context, Image, Rect};
use local_ip_address::local_ip;

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
        if self.capturer.is_recording() {
            let frame_receiver = self.capturer.frame_receiver();
            let frame = frame_receiver.recv().unwrap();
            util::update_texture(texture, frame);
            ctx.request_repaint();
        }
        let response = ui.add(Image::new(&(*texture)).shrink_to_fit()).rect;
        let ip = local_ip().expect("Should get the local IPv6");
        ui.label(format!("http://{ip}:{SERVER_PORT}/hls/output.m3u8"));
        response
    }
}
