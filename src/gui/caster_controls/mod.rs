mod area_selector;
mod device_selector;
mod preview;
use egui::{Button, Context};

use super::{Gui, Route};
use crate::server::Server;

impl Gui {
    pub fn caster_controls(&mut self, ui: &mut egui::Ui, ctx: &Context) {
        ui.heading("Caster root");
        if self.first_route_render & self.hls_server.is_none() {
            let serve_path = self.settings.borrow().get_save_dir();
            self.hls_server = Some(Server::new(serve_path));
        }

        if ui.button("Go to settings").clicked() {
            return self.route_to(Route::CasterSettings);
        }

        if ui.button("Back to device selection").clicked() {
            self.capturer.stop_recording();
            self.capturer
                .set_selected_device(None)
                .expect("Couldn't clear device selection");
            return self.route_to(Route::CasterDeviceSelection);
        }

        let preview_rectangle = self.preview(ui, ctx);

        if ui
            .add_enabled(
                !self.capturer.selecting_area,
                Button::new(if !self.capturer.selecting_area {
                    "Start Area Selection"
                } else {
                    "Click and drag"
                }),
            )
            .clicked()
        {
            self.capturer.selecting_area = true;
            self.capturer.start_point = None;
            self.capturer.end_point = None;
            self.capturer.selected_area = None;
        }

        self.area_selector(ui, &preview_rectangle);
    }
}
