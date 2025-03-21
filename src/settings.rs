use std::env;
use std::fs;
use std::path::PathBuf;

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
