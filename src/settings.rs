use std::env;
use std::path::PathBuf;

pub struct Settings {
    pub save_dir: PathBuf,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            save_dir: env::current_dir().expect("Couldn't get the current working directory"),
        }
    }
}
