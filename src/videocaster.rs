use crate::capture;
use std::collections::HashMap;
use std::io;
use std::process::Child;

#[derive(Default)]
pub struct VideoCaster {
    available_devices: HashMap<String, String>, // Elenco dei dispositivi di cattura disponibili
    selected_device: Option<String>,            // Dispositivo selezionato
    is_recording: bool,                         // Stato della registrazione
    ffmpeg_process: Option<Child>,              // Processo di registrazione
}

impl VideoCaster {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// Elenca i dispositivi di cattura disponibili
    pub fn list_devices(&mut self) -> io::Result<()> {
        self.available_devices = capture::list_screen_capture_devices()?;
        if self.available_devices.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "No screen capture devices found",
            ));
        }
        Ok(())
    }

    /// Avvia la registrazione dello schermo
    pub fn start_recording(
        &mut self,
        video_width: u32,
        video_height: u32,
        x: u32,
        y: u32,
    ) -> io::Result<()> {
        if let Some(device) = &self.selected_device {
            self.ffmpeg_process = Some(capture::start_screen_capture(
                video_width,
                video_height,
                x,
                y,
                device,
            )?);
            self.is_recording = true;
            println!("Recording started on device: {}", device);
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
            capture::stop_screen_capture(process)?;
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

    /// Restituisce una lista dei dispositivi disponibili come stringa leggibile
    pub fn get_device_list(&self) -> String {
        self.available_devices
            .iter()
            .map(|(index, name)| format!("[{}] {}", index, name))
            .collect::<Vec<_>>()
            .join("\n")
    }

    // Getter to retrieve the selected device
    pub fn get_selected_device(&self) -> Option<String> {
        self.selected_device.clone() // Clone and return the selected device (if any)
    }

    // Setter to set the selected device
    pub fn set_selected_device(&mut self, device: String) -> io::Result<()> {
        // Check if the device exists in the available devices
        if self.available_devices.contains_key(&device) {
            self.selected_device = Some(device); // Set the selected device
            Ok(()) // Return Ok if the device is found
        } else {
            // Return an error if the device is not found
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Device index not found",
            ))
        }
    }
    pub fn get_status(&self) -> bool {
        self.is_recording
    }

    // FUNZIONE MOMENTANEA DI TEST
    pub fn get_first_device(&self) -> Option<String> {
        self.available_devices.keys().next().cloned()
    }
}
