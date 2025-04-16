use crate::alias::Frame;
use crate::ffmpeg;
use std::{io::Write, path::PathBuf, process::Child};

const DEFAULT_SAVENAME: &str = "capture";

pub struct Save {
    subprocess: Child,
}

impl Save {
    pub fn new(output_path: PathBuf, width: usize, height: usize) -> Self {
        //TODO: make sure dir exists
        let output_file_path = output_path.join(DEFAULT_SAVENAME);
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
