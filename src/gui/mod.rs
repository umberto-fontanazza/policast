mod caster_controls;
mod caster_settings;
mod player_controls;
mod select_role;
use crate::capturer::Capturer;
use crate::playback::Playback;
use crate::server::Server;
use crate::settings::Settings;
use eframe;
use egui::TextureHandle;
use refbox::{Ref, RefBox};

#[derive(Default, Clone, Copy, PartialEq)]
enum Route {
    #[default]
    SelectRole,
    CasterDeviceSelection,
    CasterControls,
    CasterSettings,
    PlayerControls,
}

pub struct Gui {
    settings: Ref<Settings>,
    thumbnail_textures: Option<Vec<TextureHandle>>, //used to preview the capture devices
    _route: Route, // don't set this, use self.route_to() instead. This is used to reuse calculations between renders.
    first_route_render: bool, // to avoid repeated calculation for each render
    video_link: String,
    playback: Playback,
    capturer: Capturer,
    text_buffer: String,
    hls_server: Option<Server>,
}

impl Gui {
    pub fn new(cc: &eframe::CreationContext<'_>, s: &RefBox<Settings>) -> Self {
        // egui_extras::install_image_loaders(ctx);
        Self {
            settings: s.create_ref(),
            thumbnail_textures: None,
            capturer: Capturer::new(s.create_ref()),
            _route: Route::default(),
            first_route_render: true,
            video_link: "".to_string(),
            playback: Playback::new(&cc.egui_ctx),
            text_buffer: "Text goes here".to_owned(),
            hls_server: None,
        }
    }

    fn route_to(&mut self, destination: Route) {
        self.first_route_render = true;
        self._route = destination;
    }
}

impl eframe::App for Gui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let rendered_route = self._route;
        egui::CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {
            match self._route {
                Route::SelectRole => {
                    ui.heading("Select your role");
                    self.select_role(ui);
                }
                Route::CasterDeviceSelection => {
                    ui.heading("Capture device selection");
                    self.device_selector(ui, ctx);
                }
                Route::CasterControls => {
                    ui.heading("Caster root");
                    self.caster_controls(ui, ctx);
                }
                Route::CasterSettings => {
                    ui.heading("Caster settings");
                    self.caster_settings(ui);
                }
                Route::PlayerControls => {
                    ui.heading("Player root");
                    self.player_controls(ui, ctx); // Calling the new function here
                }
            }
        });
        if self._route == rendered_route {
            self.first_route_render = false;
        }
    }
}
