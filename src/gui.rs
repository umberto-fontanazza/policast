use crate::videocaster::VideoCaster;
use eframe;
use egui::TextEdit;
use crate::playback::Playback;
use crate::hotkey::HotkeyManager;
use egui::{Color32, TextureHandle};

const VIDEO_SIZE: [usize; 2] = [1920, 1080];

#[derive(Default)]
enum Route {
    #[default]
    SelectRole,
    CasterRoot,
    PlayerRoot,
}

pub struct Gui {
    route: Route,
    video_link: String,
    playback: Playback,
    video_caster: VideoCaster,
    video_texture: TextureHandle,
}

impl Gui {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // egui_extras::install_image_loaders(ctx);
        Self {
            route: Route::default(),
            video_texture: cc.egui_ctx.load_texture(
                "video-tex",
                egui::ColorImage {
                    size: VIDEO_SIZE,
                    pixels: vec![Color32::BLACK; VIDEO_SIZE[0] * VIDEO_SIZE[1]],
                },
                egui::TextureOptions::NEAREST,
            ),
        }
    }

    fn render_video_frame(&mut self, ctx: &egui::Context, ui: &mut egui::Ui, pixels: Vec<Color32>) {
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
}

impl eframe::App for Gui {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {
            match self.route {
                Route::SelectRole => {
                    ui.heading("Select your role");

                    if ui.button("Caster").clicked() {
                        self.route = Route::CasterRoot;
                        // Carica i dispositivi di cattura all'ingresso
                        if let Err(e) = self.video_caster.list_devices() {
                            ui.label(format!("Error: {}", e));
                        }
                    }
                    if ui.button("Player").clicked() {
                        self.route = Route::PlayerRoot;
                    }
                }
                Route::CasterRoot => {
                    ui.heading("Caster root");
                    self.caster_controls(ui);
                }
                Route::PlayerRoot => {
                    ui.heading("Player root");
                    self.player_controls(ui, ctx); // Calling the new function here
                }
            }
        });
    }
}

impl Gui {
    // Separate function to manage video playback controls
    fn player_controls(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.label("Enter the M3U link to play the video:");

        // Text field to input the M3U link
        ui.add(TextEdit::singleline(&mut self.video_link).hint_text("Enter M3U link"));

        // Button to start playback
        if ui.button("Play").clicked() {
            if !self.video_link.is_empty() {
                self.playback.set_video_link(self.video_link.clone()); // Set the video link in the playback instance
                self.playback.start_playback();                        // Start the video playback
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
                    let pixels = vec![Color32::BLACK; VIDEO_SIZE[0] * VIDEO_SIZE[1]];
                    self.render_video_frame(ctx, ui, pixels);
                }
            } else {
                ui.label("No screen capture devices found.");
            }
        }

        // Start recording when the button is pressed
        if ui.button("Start Recording").clicked() {
            if let Err(e) = self.video_caster.start_recording(2560, 1600, 0, 0) {
                ui.label(format!("Error: {}", e));  // Show error if starting recording fails
            }
        }

        // Stop recording when the button is pressed
        if ui.button("Stop Recording").clicked() {
            if let Err(e) = self.video_caster.stop_recording() {
                ui.label(format!("Error: {}", e));  // Show error if stopping recording fails
            }
        }

        // Display the recording status
        ui.label(if self.video_caster.get_status() {
            "Recording in progress..."  // Show if recording is in progress
        } else {
            "Not recording"  // Show if not recording
        });
    }

}
