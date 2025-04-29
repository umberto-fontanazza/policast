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
            self.capturer.stop();
            self.capturer
                .set_selected_device(None)
                .expect("Couldn't clear device selection");
            return self.route_to(Route::CasterDeviceSelection);
        }

        let preview_rectangle = self.preview(ui, ctx);

        ui.horizontal(|ui| {
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
                self.capturer.select_area();
            }

            let area_is_selected = self
                .capturer
                .get_selected_device()
                .expect("Device should be selected")
                .selected_area
                .is_some();

            if area_is_selected && ui.button("Clear selected area").clicked() {
                self.capturer.get_selected_device().unwrap().selected_area = None;
                self.capturer.restart().unwrap();
            }
        });

        self.area_selector(ui, &preview_rectangle);
    }
}
