mod caster_controls;
mod caster_settings;
mod player_controls;
mod select_role;
use crate::playback::Playback;
use crate::settings::Settings;
use crate::videocaster::VideoCaster;
use eframe;
use egui::Pos2;
use refbox::{Ref, RefBox};

#[derive(Default, Clone, Copy, PartialEq)]
enum Route {
    #[default]
    SelectRole,
    CasterRoot,
    CasterSettings,
    PlayerRoot,
}

pub struct Gui {
    settings: Ref<Settings>,
    _route: Route, // don't set this, use self.route_to() instead. This is used to reuse calculations between renders.
    first_route_render: bool, // to avoid repeated calculation for each render
    video_link: String,
    playback: Playback,
    video_caster: VideoCaster,
    selecting_area: bool,      // Flag per la selezione dell'area
    start_point: Option<Pos2>, // Punto iniziale della selezione
    end_point: Option<Pos2>,   // Punto finale della selezione
    selected_area: Option<(u32, u32, u32, u32)>, // Area selezionata (x, y, width, height)
    text_buffer: String,
}

impl Gui {
    pub fn new(cc: &eframe::CreationContext<'_>, s: &RefBox<Settings>) -> Self {
        // egui_extras::install_image_loaders(ctx);
        Self {
            settings: s.create_ref(),
            video_caster: VideoCaster::new(s.create_ref()),
            _route: Route::default(),
            first_route_render: true,
            video_link: "".to_string(),
            playback: Playback::new(&cc.egui_ctx),
            selecting_area: false,
            start_point: None,
            end_point: None,
            selected_area: None,
            text_buffer: "Text goes here".to_owned(),
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
                Route::CasterRoot => {
                    ui.heading("Caster root");
                    self.caster_controls(ui);
                }
                Route::CasterSettings => {
                    ui.heading("Caster settings");
                    self.caster_settings(ui);
                }
                Route::PlayerRoot => {
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
