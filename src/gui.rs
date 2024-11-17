use crate::hotkey::HotkeyManager;
use eframe::{self};
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
                    }
                    if ui.button("Player").clicked() {
                        self.route = Route::PlayerRoot;
                    }
                }
                Route::CasterRoot => {
                    ui.heading("Caster root");
                    // let hkm = HotkeyManager::default();
                    // let modifiers = ctx.input(|state| state.modifiers.to_owned());
                    // let keys = ctx.input(|state| state.keys_down.to_owned());
                    // let action = hkm.check_hotkey_actions(modifiers, keys);
                    // println!("{:?}", action);
                }
                Route::PlayerRoot => {
                    ui.heading("Player root");
                    let pixels = vec![Color32::BLACK; VIDEO_SIZE[0] * VIDEO_SIZE[1]];
                    self.render_video_frame(ctx, ui, pixels);
                }
            }
        });
    }
}
