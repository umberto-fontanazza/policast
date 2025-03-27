use egui::{ColorImage, TextureHandle};

use crate::playback::Frame;

pub fn update_texture(texture: &mut TextureHandle, frame: Frame) {
    let image = ColorImage::from_rgba_unmultiplied(
        [frame.width() as usize, frame.height() as usize],
        frame.as_raw(),
    );
    texture.set(image, Default::default());
}
