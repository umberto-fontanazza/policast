use crate::playback::Playback;
use crate::settings::Settings;
use crate::videocaster::VideoCaster;
use eframe;
use egui::{Color32, TextBuffer, TextureHandle};
use egui::{Pos2, TextEdit};
use refbox::{Ref, RefBox};

const VIDEO_SIZE: [usize; 2] = [1920, 1080];

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
    _route: Route,
    first_route_render: bool, // to avoid repeated calculation for each render
    video_link: String,
    playback: Playback,
    video_caster: VideoCaster,
    video_texture: TextureHandle,
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
            playback: Default::default(),
            video_texture: cc.egui_ctx.load_texture(
                "video-tex",
                egui::ColorImage {
                    size: VIDEO_SIZE,
                    pixels: vec![Color32::BLACK; VIDEO_SIZE[0] * VIDEO_SIZE[1]],
                },
                egui::TextureOptions::NEAREST,
            ),
            selecting_area: false,
            start_point: None,
            end_point: None,
            selected_area: None,
            text_buffer: "Text goes here".to_owned(),
        }
    }

    fn render_video_frame(
        &mut self,
        _ctx: &egui::Context,
        ui: &mut egui::Ui,
        pixels: Vec<Color32>,
    ) {
        self.video_texture.set(
            egui::ColorImage {
                size: VIDEO_SIZE,
                pixels,
            },
            egui::TextureOptions::NEAREST,
        );
        let size = self.video_texture.size_vec2();
        let sized_texture = egui::load::SizedTexture::new(&self.video_texture, size);
        ui.add(egui::Image::new(sized_texture).fit_to_exact_size(size));
    }

    fn route_to(&mut self, destination: Route) {
        self.first_route_render = true;
        self._route = destination;
    }
}

impl eframe::App for Gui {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
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

impl Gui {
    fn select_role(&mut self, ui: &mut egui::Ui) {
        if ui.button("Caster").clicked() {
            self.route_to(Route::CasterRoot);
            // Carica i dispositivi di cattura all'ingresso
            if let Err(e) = self.video_caster.list_devices() {
                ui.label(format!("Error: {}", e));
            }
        }
        if ui.button("Player").clicked() {
            self.route_to(Route::PlayerRoot);
        }
    }

    // Separate function to manage video playback controls
    fn player_controls(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.label("Enter the M3U link to play the video:");

        // Text field to input the M3U link
        ui.add(TextEdit::singleline(&mut self.video_link).hint_text("Enter M3U link"));

        // Button to start playback
        if ui.button("Play").clicked() {
            if !self.video_link.is_empty() {
                self.playback.set_video_link(self.video_link.clone()); // Set the video link in the playback instance
                self.playback.start_playback(); // Start the video playback
            } else {
                ui.label("Enter a valid link!");
            }
        }

        // Button to stop playback
        if ui.button("Stop").clicked() {
            self.playback.stop_playback();
        }

        // Show the video status and current frame
        self.playback.display_video_frame(ui, ctx); // Use the persistent playback instance to display the frame
    }

    fn caster_controls(&mut self, ui: &mut egui::Ui) {
        if ui.button("Go to settings").clicked() {
            self.route_to(Route::CasterSettings);
        }
        ui.label("Available screen capture devices:");

        // Display the list of available devices
        let device_list = self.video_caster.get_device_list();
        ui.label(&device_list);

        // Automatically select the first device if none is selected
        if self.video_caster.get_selected_device().is_none() {
            if let Some(first_device) = self.video_caster.get_first_device() {
                if let Err(e) = self.video_caster.set_selected_device(first_device.clone()) {
                    ui.label(format!("Error: {}", e));
                } else {
                    ui.label(format!("Automatically selected device: {}", first_device));
                }
            } else {
                ui.label("No screen capture devices found.");
            }
        }

        // Start recording when the button is pressed
        if ui.button("Start Recording").clicked() {
            if let Some((x, y, width, height)) = self.selected_area {
                println!("x: {} y: {} width: {} height: {} ", x, y, width, height);
                // Avvia la registrazione solo se è stata selezionata un'area
                if let Err(e) = self.video_caster.start_recording(x, y, width, height) {
                    ui.label(format!("Error: {}", e)); // Show error if starting recording fails
                }
            } else {
                ui.label("Please select an area to record.");
            }
        }

        // Stop recording when the button is pressed
        if ui.button("Stop Recording").clicked() {
            if let Err(e) = self.video_caster.stop_recording() {
                ui.label(format!("Error: {}", e)); // Show error if stopping recording fails
            }
        }

        // Display the recording status
        ui.label(if self.video_caster.get_status() {
            "Recording in progress..." // Show if recording is in progress
        } else {
            "Not recording" // Show if not recording
        });

        // Area selection UI
        if ui.button("Start Area Selection").clicked() {
            self.selecting_area = true;
            self.start_point = None;
            self.end_point = None;
            self.selected_area = None;
        }

        // Handle the area selection
        self.handle_area_selection(ui);
    }

    fn handle_area_selection(&mut self, ui: &mut egui::Ui) {
        let pointer = ui.input(|i| i.pointer.clone()); // Usa un closure per accedere al pointer

        if self.selecting_area {
            // Cattura il clic iniziale
            if pointer.any_pressed() && self.start_point.is_none() {
                if let Some(pos) = pointer.interact_pos() {
                    self.start_point = Some(pos);
                }
            }

            // Aggiorna il punto finale mentre si trascina
            if pointer.primary_down() {
                if let Some(pos) = pointer.interact_pos() {
                    self.end_point = Some(pos);
                }
            }

            // Rilascia il mouse per confermare la selezione
            if pointer.any_released() && self.start_point.is_some() && self.end_point.is_some() {
                if let (Some(start), Some(end)) = (self.start_point, self.end_point) {
                    // Calcola l'area selezionata
                    let x = start.x.min(end.x) as u32;
                    let y = start.y.min(end.y) as u32;
                    let width = (start.x - end.x).abs() as u32;
                    let height = (start.y - end.y).abs() as u32;

                    self.selected_area = Some((x, y, width, height));
                    self.selecting_area = false; // Disabilita la selezione
                }
            }

            // Disegna un rettangolo durante la selezione
            if let (Some(start), Some(end)) = (self.start_point, self.end_point) {
                let rect = egui::Rect::from_two_pos(start, end);
                ui.painter().rect(
                    rect,
                    0.0,
                    egui::Color32::from_rgba_premultiplied(150, 150, 200, 100),
                    egui::Stroke::new(1.0, egui::Color32::WHITE),
                );
            }
        }

        // Mostra l'area selezionata se esiste
        if let Some((x, y, width, height)) = self.selected_area {
            ui.label(format!(
                "Selected Area: Position ({}, {}), Size ({}, {})",
                x, y, width, height
            ));
        }
    }

    fn caster_settings(&mut self, ui: &mut egui::Ui) {
        if self.first_route_render {
            println!("First render of route settings!");
        };
        let b = self
            .settings
            .try_borrow_mut()
            .expect("Cannot borrow settings")
            .save_dir
            .clone();
        ui.label("Select save directory:");
        ui.label(format!(
            "{}",
            b.to_str().expect("Couldn't stringyfy pathbuf")
        ));
        ui.add(egui::TextEdit::singleline(&mut self.text_buffer));
        if ui.button("Apply").clicked() {
            ui.label(format!("Applied ",));
        }
    }
}
