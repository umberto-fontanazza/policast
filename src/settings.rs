use std::env;
use std::path::PathBuf;

pub struct Settings {
    save_dir: PathBuf,
}

impl Settings {
    pub fn set_save_dir(&mut self, path: &str) -> Result<(), ()> {
        let path = PathBuf::from(path);
        match path.is_dir() {
            true => {
                self.save_dir = path;
                Ok(())
            }
            false => Err(()),
        }
    }

    pub fn get_save_dir(&self) -> PathBuf {
        self.save_dir.clone()
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            save_dir: env::current_dir().expect("Couldn't get the current working directory"),
        }
    }
}
