use crate::ffmpeg;
use crate::screen::Screen;
use crate::settings::Settings;
use egui::{Pos2, Rect};
use refbox::Ref;
use std::io::{self, Read};
use std::path::PathBuf;
use std::process::Child;
use std::thread::spawn;

#[derive(Default)]
pub struct Capturer {
    capture_devices: Vec<Screen>, // Elenco dei dispositivi di cattura disponibili
    selected_device: Option<String>, // Dispositivo selezionato
    is_recording: bool,           // Stato della registrazione
    ffmpeg_process: Option<Child>, // Processo di registrazione
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

    // pub fn set_capture_devices(&mut self) -> io::Result<()> {
    //     self.capture_devices = ffmpeg::list_screen_capture_devices()?;
    //     if self.capture_devices.is_empty() {
    //         return Err(io::Error::new(
    //             io::ErrorKind::NotFound,
    //             "No screen capture devices found",
    //         ));
    //     }
    //     Ok(())
    // }

    /// entries are like: (index of device, device name)
    // pub fn get_capture_devices(&self) -> HashMap<String, String> {
    //     self.capture_devices.clone()
    // }

    pub fn get_capture_devices(&self) -> &Vec<Screen> {
        &self.capture_devices
    }

    /// Avvia la registrazione dello schermo
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
            self.is_recording = true;
            let crop = self.selected_area.map(|rect| ScreenCrop::from(rect));
            _start_recording(crop, device.clone(), save_dir);
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "No device selected for recording",
            ))
        }
    }

    /// Interrompe la registrazione dello schermo
    pub fn stop_recording(&mut self) -> io::Result<()> {
        if let Some(process) = self.ffmpeg_process.take() {
            ffmpeg::stop_screen_capture(process)?;
            self.is_recording = false;
            println!("Recording stopped.");
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::NotFound,
                "No active recording to stop",
            ))
        }
    }

    // Getter to retrieve the selected device
    pub fn get_selected_device(&self) -> Option<String> {
        self.selected_device.clone() // Clone and return the selected device (if any)
    }

    // Setter to set the selected device
    pub fn set_selected_device(&mut self, device: String) -> io::Result<()> {
        let device_exists = self
            .capture_devices
            .iter()
            .any(|screen| screen.handle().eq(&device));
        if device_exists {
            self.selected_device = Some(device);
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Device index not found",
            ))
        }
    }
    pub fn get_is_recording(&self) -> bool {
        self.is_recording
    }
}

/**
 * Rect is used for portion of the application window, ScreenCrop is used to identify a portion of the screen
 */
pub struct ScreenCrop {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl From<egui::Rect> for ScreenCrop {
    //TODO: this conversion doesn't take into account the window offset
    fn from(value: egui::Rect) -> Self {
        Self {
            x: value.left() as u32,
            y: value.top() as u32,
            width: value.width() as u32,
            height: value.height() as u32,
        }
    }
}

impl ScreenCrop {}

fn _start_recording(crop: Option<ScreenCrop>, device: String, save_dir: PathBuf) {
    spawn(move || -> ! {
        let child = ffmpeg::start_screen_capture(crop, &device, &save_dir)
            .expect("Should start screen capture");

        let mut buffer = vec![0u8; 3840 * 2160 * 4]; // TODO: this must be set dynamically
        let mut out = child.stdout.expect("Couldn't get stdout");
        loop {
            match out.read_exact(&mut buffer) {
                Ok(_) => (),
                Err(_) => todo!(),
            }
        }
    });
}
