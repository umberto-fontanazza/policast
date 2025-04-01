use std::env;
use std::path::PathBuf;
use std::time::Duration;

pub const CAPTURE_HEIGHT: usize = 1080;
pub const CAPTURE_FPS: usize = 25;
pub const CAPTURE_PERIOD: Duration = Duration::from_millis(1000 / CAPTURE_FPS as u64); //TODO: check cast

pub struct Settings {
    save_dir: PathBuf,
}

impl Settings {
    pub fn set_save_dir(&mut self, path: &str) {
        self.save_dir = PathBuf::from(path);
    }

    pub fn get_save_dir(&self) -> PathBuf {
        self.save_dir.clone()
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            save_dir: env::current_dir()
                .expect("Couldn't get the current working directory")
                .join("capture"),
        }
    }
}
