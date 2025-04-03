use egui::{ColorImage, Rect};

use crate::{
    crop::ScreenCrop,
    ffmpeg::{list_screen_capture_devices, take_screenshot},
};

#[derive(Clone)]
pub struct Screen {
    handle: String,
    name: String,
    sceenshot: Option<ColorImage>,
}

impl Screen {
    pub fn new(handle: String, name: Option<String>) -> Self {
        Self {
            handle: handle.clone(),
            name: name.or(Some(handle)).unwrap(),
            sceenshot: None,
        }
    }

    pub fn load_screenshot(&mut self) {
        self.sceenshot = Some(take_screenshot(&self.handle));
    }

    fn _check_sceenshot(&mut self) {
        match self.sceenshot {
            None => {
                self.load_screenshot();
            }
            Some(_) => (),
        }
    }

    pub fn width(&mut self) -> usize {
        self._check_sceenshot();
        self.sceenshot.as_ref().unwrap().width()
    }

    pub fn height(&mut self) -> usize {
        self._check_sceenshot();
        self.sceenshot.as_ref().unwrap().height()
    }

    pub fn handle(&self) -> String {
        self.handle.clone()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn screenshot(&mut self) -> &ColorImage {
        self._check_sceenshot();
        &self.sceenshot.as_ref().unwrap()
    }

    pub fn get_all() -> Vec<Screen> {
        list_screen_capture_devices()
            .expect("Should list available screens as video input sources")
            .into_iter()
            .map(|(handle, name)| Screen::new(handle, Some(name)))
            .collect::<Vec<Screen>>()
    }

    pub fn crop(&mut self, source: Rect, crop: Rect) -> ScreenCrop {
        let x = crop.left() - source.left();
        let y = crop.top() - source.top();
        let (width, height) = (crop.width(), crop.height());
        let (app_width, app_height) = (source.width(), source.height());
        let x = (x / app_width * self.width() as f32).round() as usize;
        let y = (y / app_height * self.height() as f32).round() as usize;
        let width = (width / app_width * self.width() as f32).round() as usize;
        let height = (height / app_height * self.height() as f32).round() as usize;
        ScreenCrop::new(x, y, width, height)
    }
}
