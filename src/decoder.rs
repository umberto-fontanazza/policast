use crate::save::Save;
use crate::settings::CAPTURE_FPS;
use crate::{
    alias::{Frame, StopSignal},
    playback::{HEIGHT, WIDTH},
};
use crate::{ffmpeg, util};
use std::path::PathBuf;
use std::{
    io::Read,
    process::{Command, Stdio},
    sync::mpsc::{channel, Receiver, RecvError, Sender},
    thread::{self, JoinHandle},
};

pub struct Decoder {
    sender: Sender<StopSignal>,
    receiver: Receiver<Frame>,
    handle: Option<JoinHandle<()>>,
    save: Option<Save>,
}

impl Decoder {
    pub fn new(video_url: String) -> Self {
        let save: bool = true; //TODO: this must be a @param
        let (sender, receiver) = channel::<()>();
        let (frame_sender, frame_receiver) = channel::<Frame>();

        let handle = thread::spawn(move || {
            let mut subprocess = Command::new("ffmpeg")
                .args(get_ffmpeg_decoder_args(video_url.as_str()))
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()
                .expect("Failed to start FFmpeg");

            let stdout = subprocess.stdout.as_mut().expect("Failed to take stdout");
            let mut buffer = vec![0u8; WIDTH * HEIGHT * 4];

            // Continuously read the video frames from stdout
            while stdout.read_exact(&mut buffer).is_ok() {
                match receiver.try_recv() {
                    Ok(_) => {
                        ffmpeg::stop_screen_capture(subprocess, &mut buffer);
                        return; // received signal to stop
                    }
                    Err(e) => match e {
                        std::sync::mpsc::TryRecvError::Empty => {}
                        std::sync::mpsc::TryRecvError::Disconnected => {
                            println!("This shouldn't happen");
                            break;
                        }
                    },
                }
                let frame: Frame = util::frame_from_buffer(WIDTH, HEIGHT, buffer.clone());
                frame_sender
                    .send(frame)
                    .expect("Couldn't send frame over channel");
            }
        });
        Self {
            sender,
            receiver: frame_receiver,
            handle: Some(handle),
            save: Some(Save::new(
                PathBuf::from("/Users/umbertofontanazza/Projects/Polito/api-programming/mpsc/save"), //TODO: directory selection
                WIDTH,
                HEIGHT,
            )),
        }
    }

    pub fn recv(&mut self) -> Result<Frame, RecvError> {
        let res = self.receiver.recv();
        match res {
            // send frame to mp4 encoder subprocess for saving
            Ok(ref frame) => match self.save {
                Some(ref mut save) => save.frame(&frame),
                None => (),
            },
            Err(_) => (),
        }
        res
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
        format!("{CAPTURE_FPS}"),
        "-vf".to_owned(),
        format!("fps={CAPTURE_FPS},scale={WIDTH}:{HEIGHT},format=rgba"), // Set resolution and format
        "-pix_fmt".to_owned(),
        "rgba".to_owned(), // Set pixel format
        "-f".to_owned(),
        "rawvideo".to_owned(), // Set raw video output format
        "-".to_owned(),        // Output to stdout
    ]
}
