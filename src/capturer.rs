use crate::alias::{Frame, StopSignal};
use crate::crop::ScreenCrop;
use crate::screen::Screen;
use crate::settings::Settings;
use crate::{ffmpeg, util};
use egui::{Context, Image, Pos2, Rect, TextureHandle, Ui, Vec2};
use refbox::Ref;
use std::io::{self, Read};
use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread::{spawn, JoinHandle};

#[derive(Default)]
pub struct Capturer {
    capture_devices: Vec<Screen>,
    selected_device: Option<String>,
    is_recording: bool,
    helper_handle: Option<(JoinHandle<()>, Receiver<Frame>, Sender<StopSignal>)>,
    settings: Option<Ref<Settings>>,
    pub selecting_area: bool, // Flag per la selezione dell'area
    pub selected_area: Option<Rect>,
    pub start_point: Option<Pos2>, // Punto iniziale della selezione
    pub end_point: Option<Pos2>,   // Punto finale della selezione
}

impl Capturer {
    pub fn new(settings: Ref<Settings>) -> Self {
        Self {
            settings: Some(settings),
            ..Default::default()
        }
    }

    pub fn set_capture_devices(&mut self) {
        self.capture_devices = Screen::get_all();
    }

    pub fn get_capture_devices(&self) -> &Vec<Screen> {
        &self.capture_devices
    }

    pub fn start_recording(&mut self) -> io::Result<()> {
        let save_dir = {
            self.settings
                .as_ref()
                .expect("Videocaster should have access to settings")
                .try_borrow_mut()
                .expect("Should be able to access settings")
                .get_save_dir()
        };
        if !save_dir.is_dir() {
            std::fs::create_dir_all(&save_dir).expect("Should create dir if missing");
        }
        if let Some(device) = &self.selected_device {
            let device = self
                .capture_devices
                .iter()
                .find(|screen| screen.handle().eq(device))
                .expect("Selected device hanlde is inconsistent with available devices");
            self.is_recording = true;
            let crop = self.selected_area.map(|rect| ScreenCrop::from(rect));
            let handle = _start_recording(crop, device.clone(), save_dir);
            self.helper_handle = Some(handle);
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "No device selected for recording",
            ))
        }
    }

    pub fn stop_recording(&mut self) {
        if self.helper_handle.is_none() {
            return;
        }
        self.is_recording = false;
        let (_, _, stopper) = self.helper_handle.as_ref().unwrap();
        let _ = stopper.send(() as StopSignal); //TODO: handle error
    }

    pub fn get_selected_device(&self) -> Option<String> {
        self.selected_device.clone()
    }

    pub fn set_selected_device(&mut self, device: Option<String>) -> io::Result<()> {
        if device.is_none() {
            self.selected_device = device;
            return Ok(());
        }
        let device_exists = self
            .capture_devices
            .iter()
            .any(|screen| screen.handle().eq(device.as_ref().unwrap()));
        if device_exists {
            self.selected_device = device;
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Device index not found",
            ))
        }
    }

    pub fn is_recording(&self) -> bool {
        self.is_recording
    }

    pub fn render(&mut self, ui: &mut Ui, ctx: &Context, texture: &mut TextureHandle) {
        match self.is_recording {
            true => {
                let (_, frame_receiver, __) = self.helper_handle.as_ref().unwrap();
                let frame = frame_receiver.recv().unwrap();
                util::update_texture(texture, frame);
                ui.add(
                    Image::new(&(*texture))
                        .maintain_aspect_ratio(true)
                        .fit_to_fraction(Vec2::new(1.0, 2.0)),
                );
                ctx.request_repaint();
            }
            false => {
                ui.label("Capturer not recoding");
            }
        }
    }
}

fn _start_recording(
    crop: Option<ScreenCrop>,
    device: Screen,
    save_dir: PathBuf,
) -> (JoinHandle<()>, Receiver<Frame>, Sender<StopSignal>) {
    let mut device = device;
    let (width, height) = match crop {
        Some(ref crop) => (crop.width, crop.height),
        None => (device.width(), device.height()),
    };
    let (sender, receiver) = channel::<StopSignal>();
    let (frame_sender, frame_receiver) = channel::<Frame>();
    let handle = spawn(move || {
        let mut subprocess = ffmpeg::start_screen_capture(crop, &device.handle(), &save_dir)
            .expect("Should start screen capture");

        let mut buffer = vec![0u8; width * height * 4];
        {
            let out = subprocess.stdout.as_mut().expect("Couldn't get stdout");
            while out.read_exact(&mut buffer).is_ok() {
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
                let frame: Frame = util::frame_from_buffer(width, height, buffer.clone());
                frame_sender
                    .send(frame)
                    .expect("Couldn't send frame over channel");
            }
        }
        ffmpeg::stop_screen_capture(subprocess).unwrap();
    });
    (handle, frame_receiver, sender)
}
