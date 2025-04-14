use egui::Rect;
use fraction::Fraction;

/*
 * All the field composing screen crop are relative measures. For example x is the offset from the top left corner of the
 * screen as a percentage of the screen width.
 * The crop height is the percentage of the cropped area with respect to the screen area.
 */
#[derive(Clone)]
pub struct RelativeScreenCrop {
    pub x: Fraction,      // relative distance on the x axis from the top left corner
    pub y: Fraction,      // ... on the y axis ...
    pub width: Fraction,  // relative width
    pub height: Fraction, // relative height
}
impl RelativeScreenCrop {
    pub fn new(container: &Rect, selection: &Rect) -> Self {
        let (x, y) = (container.left(), container.top());
        let (crop_x, crop_y) = (selection.left(), selection.top());
        let (width, height) = (container.width(), container.height());
        let (crop_width, crop_height) = (selection.width(), selection.height());
        // casting to Fraction
        let (x, y) = (Fraction::from(x), Fraction::from(y));
        let (crop_x, crop_y) = (Fraction::from(crop_x), Fraction::from(crop_y));
        let (width, height) = (Fraction::from(width), Fraction::from(height));
        let (crop_width, crop_height) = (Fraction::from(crop_width), Fraction::from(crop_height));
        Self {
            x: (crop_x - x) / width,
            y: (crop_y - y) / height,
            width: crop_width / width,
            height: crop_height / height,
        }
    }
}

pub struct CropFilter {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

impl CropFilter {
    pub fn from(relative_crop: &RelativeScreenCrop, width: usize, height: usize) -> Self {
        let (width, height) = (Fraction::from(width), Fraction::from(height));
        let x: usize = (relative_crop.x * width)
            .round()
            .try_into()
            .expect("Should cast Fraction to usize");
        let y: usize = (relative_crop.y * height)
            .round()
            .try_into()
            .expect("Should cast Fraction to usize");
        let width: usize = (relative_crop.width * width)
            .round()
            .try_into()
            .expect("Should cast Fraction to usize");
        let height: usize = (relative_crop.height * height)
            .round()
            .try_into()
            .expect("Should cast Fraction to usize");
        Self {
            x,
            y,
            width: truncate_to_even(width),
            height: truncate_to_even(height),
        }
    }
}

fn truncate_to_even(number: usize) -> usize {
    (number >> 1) << 1
}
