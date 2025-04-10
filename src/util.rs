use std::{io::Read, process::ChildStdout};

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

pub fn read_while_full(stdout: &mut ChildStdout, buffer: Option<&mut [u8]>) {
    let mut _buffer = Vec::<u8>::with_capacity(if buffer.is_none() { 1024 } else { 0 });
    let buffer = buffer.or(Some(&mut _buffer)).unwrap();
    loop {
        let read_result = stdout.read(buffer);
        match read_result {
            Ok(bytes_read) if bytes_read == 0 => break,
            Ok(_) => (),
            Err(_) => panic!("Failed to read from subprocess stdout"),
        }
    }
}
