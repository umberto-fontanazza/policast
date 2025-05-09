use egui::{Context, TextEdit, Ui};

use crate::server::DiscoveryService;
use crate::settings::SERVER_PORT;

use super::Gui;

impl Gui {
    pub fn caster_selector(&mut self, ui: &mut Ui, _ctx: &Context) {
        if self.first_route_render & self.playback.discovery_service.is_none() {
            self.playback.discovery_service = Some(DiscoveryService::new());
            let casters = self
                .playback
                .discovery_service
                .as_mut()
                .map(|ds| ds.get_casters())
                .unwrap();
            self.playback.sources = casters;
        }

        self.playback
            .sources
            .iter()
            .find(|ip| {
                let mut return_value = false;
                ui.horizontal(|ui| {
                    ui.label(format!("{ip}"));
                    return_value = ui.button("Watch this caster").clicked();
                });
                return_value
            })
            .map(|ip| format!("http://{ip}:{SERVER_PORT}/hls/output.m3u8"))
            .and_then(|url| {
                self.playback.video_link = url;
                self.playback.play();
                None as Option<()>
            });

        if ui.button("Refresh sources").clicked() {
            let casters = self
                .playback
                .discovery_service
                .as_mut()
                .map(|ds| ds.get_casters())
                .unwrap();
            self.playback.sources = casters;
        }

        ui.label("Enter the M3U link to play the video:");

        // Text field to input the M3U link
        ui.add(
            TextEdit::singleline(&mut self.playback.video_link)
                .hint_text("Enter M3U playlist link"),
        );
    }
}
