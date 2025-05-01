use crate::alias::Frame;
use crate::ffmpeg;
use crate::util::fallback_filename;
use std::{io::Write, path::PathBuf, process::Child};

const DEFAULT_SAVENAME: &str = "capture";

pub struct Save {
    subprocess: Child,
}

impl Save {
    pub fn new(save_dir: PathBuf, width: usize, height: usize) -> Self {
        if !save_dir.is_dir() {
            std::fs::create_dir_all(&save_dir).expect("Should create save dir if missing");
        }
        let output_file_path = save_dir.join(fallback_filename(&save_dir, DEFAULT_SAVENAME, "mp4"));
        let subprocess = ffmpeg::spawn_raw_encoder(
            width,
            height,
            output_file_path
                .to_str()
                .expect("Should convert the path to a str"),
        );
        Self { subprocess }
    }

    pub fn frame(&mut self, frame: &Frame) {
        self.subprocess
            .stdin
            .as_mut()
            .expect("Should unwrap stdin")
            .write(frame.as_raw())
            .expect("Should write frame to encoder stdin");
    }
}
