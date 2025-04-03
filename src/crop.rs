/**
 * Rect is used for portion of the application window, ScreenCrop is used to identify a portion of the screen
 */
#[derive(Clone)]
pub struct ScreenCrop {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}
impl ScreenCrop {
    pub fn new(x: usize, y: usize, width: usize, height: usize) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

impl ScreenCrop {}
