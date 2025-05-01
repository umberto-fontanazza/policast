use std::{io::Read, path::Path, process::ChildStdout};

use egui::{ColorImage, Modifiers, TextureHandle};
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
    ImageBuffer::from_raw(
        u32::try_from(width).unwrap(),
        u32::try_from(height).unwrap(),
        buffer.clone(),
    )
    .expect("Failed to create frame from buffer")
}

pub fn read_while_full(stdout: &mut ChildStdout, buffer: &mut [u8]) {
    loop {
        let read_result = stdout.read(buffer);
        match read_result {
            Ok(bytes_read) if bytes_read == 0 => break,
            Ok(_) => (),
            Err(_) => panic!("Failed to read from subprocess stdout"),
        }
    }
}

pub fn modifiers_to_string(modifiers: &Modifiers) -> String {
    vec![
        (String::from("ALT"), modifiers.alt),
        (String::from("CTRL"), modifiers.ctrl),
        (String::from("SHIFT"), modifiers.shift),
        (String::from("CMD"), modifiers.mac_cmd),
    ]
    .into_iter()
    .filter(|(_, flag)| *flag)
    .map(|(str, _)| str)
    .collect::<Vec<String>>()
    .join(" + ")
}

pub fn fallback_filename(dir_path: &Path, filename: &str, extension: &str) -> String {
    _fallback_filename(dir_path, filename, extension, 0)
}

fn _fallback_filename(
    dir_path: &Path,
    filename: &str,
    extension: &str,
    iteration: usize,
) -> String {
    let suffix = if iteration == 0 {
        "".to_string()
    } else {
        format!("_{iteration}")
    };
    let file_path = dir_path.join(format!("{filename}{suffix}.{extension}"));
    if !file_path.is_file() {
        format!("{filename}{suffix}")
    } else {
        _fallback_filename(dir_path, filename, extension, iteration + 1)
    }
}
