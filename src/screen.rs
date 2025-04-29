use egui::ColorImage;

use crate::{
    crop::RelativeScreenCrop,
    ffmpeg::{list_screen_capture_devices, take_screenshot},
};

#[derive(Clone)]
pub struct Screen {
    handle: String,
    name: String,
    sceenshot: Option<ColorImage>,
    pub selected_area: Option<RelativeScreenCrop>,
}

impl Screen {
    pub fn new(handle: String, name: Option<String>) -> Self {
        Self {
            handle: handle.clone(),
            name: name.or(Some(handle)).unwrap(),
            sceenshot: None,
            selected_area: None,
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
}
