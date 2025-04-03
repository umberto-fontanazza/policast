mod area_selector;
mod device_selector;
mod preview;
use egui::Context;

use super::{Gui, Route};
use crate::server::Server;

impl Gui {
    pub fn caster_controls(&mut self, ui: &mut egui::Ui, ctx: &Context) {
        if self.first_route_render & self.hls_server.is_none() {
            self.hls_server = Some(Server::new());
        }

        if ui.button("Go to settings").clicked() {
            self.route_to(Route::CasterSettings);
        }

        if ui.button("Back to device selection").clicked() {
            self.capturer.stop_recording();
            self.capturer
                .set_selected_device(None)
                .expect("Couldn't clear device selection");
            self.route_to(Route::CasterDeviceSelection);
        }

        self.preview(ui, ctx);

        // Area selection UI
        if ui.button("Start Area Selection").clicked() {
            self.capturer.selecting_area = true;
            self.capturer.start_point = None;
            self.capturer.end_point = None;
            self.capturer.selected_area = None;
        }

        // Handle the area selection
        self.area_selector(ui);
    }
}
