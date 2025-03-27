use egui::{ColorImage, TextureHandle};
use image::ImageBuffer;

use crate::alias::Frame;

pub fn update_texture(texture: &mut TextureHandle, frame: Frame) {
    let image = ColorImage::from_rgba_unmultiplied(
        [frame.width() as usize, frame.height() as usize],
        frame.as_raw(),
    );
    texture.set(image, Default::default());
}

pub fn frame_from_buffer(width: usize, height: usize, buffer: Vec<u8>) -> Frame {
    // TODO: check cast failure cases
    ImageBuffer::from_raw(
        u32::try_from(width).unwrap(),
        u32::try_from(height).unwrap(),
        buffer.clone(),
    )
    .expect("Failed to create frame from buffer")
}
