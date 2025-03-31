/**
 * Rect is used for portion of the application window, ScreenCrop is used to identify a portion of the screen
 */
pub struct ScreenCrop {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl From<egui::Rect> for ScreenCrop {
    //TODO: this conversion doesn't take into account the window offset
    fn from(value: egui::Rect) -> Self {
        Self {
            x: value.left() as u32,
            y: value.top() as u32,
            width: value.width() as u32,
            height: value.height() as u32,
        }
    }
}

impl ScreenCrop {}
