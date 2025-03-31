/**
 * Rect is used for portion of the application window, ScreenCrop is used to identify a portion of the screen
 */
pub struct ScreenCrop {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

impl From<egui::Rect> for ScreenCrop {
    //TODO: this conversion doesn't take into account the window offset
    fn from(value: egui::Rect) -> Self {
        Self {
            x: value.left() as usize,
            y: value.top() as usize,
            width: value.width() as usize,
            height: value.height() as usize,
        }
    }
}

impl ScreenCrop {}
