use image::ImageBuffer;

use crate::playback::{Frame, FPS, HEIGHT, WIDTH};
use std::{
    io::Read,
    process::{Command, Stdio},
    sync::mpsc::{channel, Receiver, RecvError, Sender},
    thread::{self, JoinHandle},
};

type StopSignal = ();

pub struct Decoder {
    sender: Sender<StopSignal>,
    receiver: Receiver<Frame>,
    handle: Option<JoinHandle<()>>,
}

impl Decoder {
    pub fn new(video_url: String) -> Self {
        let (sender, receiver) = channel::<()>();
        let (frame_sender, frame_receiver) = channel::<Frame>();

        let handle = thread::spawn(move || {
            let mut process = Command::new("ffmpeg")
                .args(get_ffmpeg_decoder_args(video_url.as_str()))
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()
                .expect("Failed to start FFmpeg");

            let mut stdout = process.stdout.take().expect("Failed to take stdout");
            let mut buffer = vec![0u8; WIDTH * HEIGHT * 4];

            // Continuously read the video frames from stdout
            while stdout.read_exact(&mut buffer).is_ok() {
                match receiver.try_recv() {
                    Ok(_) => {
                        break; // received signal to stop
                    }
                    Err(e) => match e {
                        std::sync::mpsc::TryRecvError::Empty => {}
                        std::sync::mpsc::TryRecvError::Disconnected => {
                            println!("This shouldn't happen");
                            break;
                        }
                    },
                }
                let frame: Frame = ImageBuffer::from_raw(
                    u32::try_from(WIDTH).unwrap(),
                    u32::try_from(HEIGHT).unwrap(),
                    buffer.clone(),
                )
                .expect("Couldn't create image buffer");
                frame_sender
                    .send(frame)
                    .expect("Couldn't send frame over channel");
            }
            process.kill().expect("Couldn't kill process");
        });
        Self {
            sender,
            receiver: frame_receiver,
            handle: Some(handle),
        }
    }

    pub fn recv(&self) -> Result<Frame, RecvError> {
        self.receiver.recv()
    }
}

impl Drop for Decoder {
    fn drop(&mut self) {
        match self.sender.send(() as StopSignal) {
            _ => (), // we don't care, the error case happens when the decoder finished before we stopped the video and was no longer needed.
        };
        self.handle
            .take()
            .unwrap()
            .join()
            .expect("Couldn't join decoder helper thread");
    }
}

pub fn get_ffmpeg_decoder_args(video_link: &str) -> Vec<String> {
    vec![
        "-hide_banner".to_owned(),
        "-loglevel".to_owned(),
        "error".to_owned(),
        "-i".to_owned(),
        video_link.to_owned(), // Input the video link
        "-r".to_owned(),
        format!("{FPS}"),
        "-vf".to_owned(),
        format!("fps={FPS},scale={WIDTH}:{HEIGHT},format=rgba"), // Set resolution and format
        "-pix_fmt".to_owned(),
        "rgba".to_owned(), // Set pixel format
        "-f".to_owned(),
        "rawvideo".to_owned(), // Set raw video output format
        "-".to_owned(),        // Output to stdout
    ]
}
