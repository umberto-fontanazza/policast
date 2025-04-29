use crate::alias::{Frame, StopSignal};
use crate::crop::{CropFilter, RelativeScreenCrop};
use crate::screen::Screen;
use crate::settings::{Settings, CAPTURE_HEIGHT};
use crate::{ffmpeg, util};
use egui::{Pos2, Rect};
use std::cell::RefCell;
use std::io::{self, Read};
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread::{spawn, JoinHandle};

#[derive(Default)]
pub struct Capturer {
    capture_devices: Vec<Screen>,
    selected_device: Option<String>,
    is_recording: bool,
    helper_handle: Option<(JoinHandle<()>, Receiver<Frame>, Sender<StopSignal>)>,
    settings: Option<Rc<RefCell<Settings>>>,
    pub selecting_area: bool, // Flag per la selezione dell'area
    pub selected_area: Option<RelativeScreenCrop>,
    pub start_point: Option<Pos2>, // Punto iniziale della selezione
    pub end_point: Option<Pos2>,   // Punto finale della selezione
}

impl Capturer {
    pub fn new(settings: Rc<RefCell<Settings>>) -> Self {
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
        let save_dir = self.settings.as_ref().unwrap().borrow().get_save_dir();
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
            let crop = self.selected_area.clone();
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

    pub fn get_selected_device(&mut self) -> Option<&mut Screen> {
        self.selected_device.as_mut().map(|selected_device_handle| {
            self.capture_devices
                .iter_mut()
                .find(|screen| screen.handle() == *selected_device_handle)
                .expect("Inconsistent state on capturer, self.selected_device is set but the respective device handle was not found among the devices")
        })
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

    pub fn set_selected_area(&mut self, preview_rect: &Rect, crop: &Rect) {
        self.selected_area = Some(RelativeScreenCrop::new(preview_rect, crop));
    }

    pub fn is_recording(&self) -> bool {
        self.is_recording
    }

    pub fn frame_receiver(&mut self) -> &mut Receiver<Frame> {
        let (_, rx, __) = self.helper_handle.as_mut().unwrap();
        rx
    }
}

fn _start_recording(
    relative_crop: Option<RelativeScreenCrop>,
    device: Screen,
    save_dir: PathBuf,
) -> (JoinHandle<()>, Receiver<Frame>, Sender<StopSignal>) {
    let mut device = device;
    let height = CAPTURE_HEIGHT;
    let width = device.width() * height / device.height();
    let crop = relative_crop
        .as_ref()
        .map(|rel| CropFilter::from(rel, width, height));
    let (width, height) = match crop {
        Some(ref c) => (c.width, c.height),
        None => (width, height),
    };
    let (sender, receiver) = channel::<StopSignal>();
    let (frame_sender, frame_receiver) = channel::<Frame>();
    let handle = spawn(move || {
        let mut subprocess =
            ffmpeg::start_screen_capture(Some(CAPTURE_HEIGHT), crop, &device.handle(), &save_dir)
                .expect("Should start screen capture");

        let mut buffer = vec![0u8; width * height * 4];
        let stdout = subprocess
            .stdout
            .as_mut()
            .expect("Should borrow mutably stdout");
        while stdout.read_exact(&mut buffer).is_ok() {
            match receiver.try_recv() {
                Ok(_) => {
                    ffmpeg::stop_screen_capture(subprocess, &mut buffer);
                    return;
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
    });
    (handle, frame_receiver, sender)
}
