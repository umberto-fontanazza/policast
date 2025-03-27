use super::Gui;
use egui::{ColorImage, Context};

impl Gui {
    pub fn recording_controls(&mut self, ui: &mut egui::Ui, ctx: &Context) {
        if self.first_route_render {
            self.preview_texture =
                Some(ctx.load_texture("preview", ColorImage::default(), Default::default()))
        }
        if !self.capturer.is_recording() {
            if ui.button("Start Recording").clicked() {
                //TODO: error management
                self.capturer.start_recording();
            }
        } else {
            if ui.button("Stop Recording").clicked() {
                self.capturer.stop_recording();
            }
        }
        self.capturer
            .render(ui, ctx, self.preview_texture.as_mut().unwrap());
    }
}
