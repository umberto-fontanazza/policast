use directories::{ProjectDirs, UserDirs};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string_pretty};
use std::env;
use std::fs::{create_dir_all, File};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::time::Duration;

pub const CAPTURE_HEIGHT: usize = 1080;
pub const CAPTURE_FPS: usize = 25;
pub const CAPTURE_PERIOD: Duration = Duration::from_millis(1000 / CAPTURE_FPS as u64);
pub const SERVER_PORT: u16 = 3000;
/** Duration in seconds of a single HLS segment */
pub const HLS_SEGMENT_DURATION: usize = 2;
/** Number of segments available at max at any time */
pub const HLS_LIST_SIZE: usize = 4;

// playback settings
pub const DECODER_WIDTH: usize = 1280;
pub const DECODER_HEIGHT: usize = 720;

pub const APP_QUALIFIER: &str = "com";
pub const APP_ORGANIZATION: &str = "polito";
pub const APP_NAME: &str = "PoliCast";

#[derive(Serialize, Deserialize)]
pub struct Settings {
    caster_save_dir: PathBuf, // segment files and playlist manifest are stored here
    pub player_save_dir: Option<PathBuf>,
    pub player_save_enabled: bool,
}

impl Settings {
    pub fn set_save_dir(&mut self, path: &str) {
        self.caster_save_dir = PathBuf::from(path);
    }

    pub fn get_save_dir(&self) -> PathBuf {
        self.caster_save_dir.clone()
    }

    pub fn try_load() -> Result<Self, ()> {
        let dirs = ProjectDirs::from(APP_QUALIFIER, APP_ORGANIZATION, APP_NAME).unwrap();
        let file_path = dirs.config_dir().join("settings.json");
        if !file_path.is_file() {
            return Err(());
        }
        let mut file = File::open(&file_path).map_err(|_| ())?;
        let mut file_content = String::new();
        file.read_to_string(&mut file_content).map_err(|_| ())?;
        from_str::<Settings>(&file_content).map_err(|_| ())
    }

    pub fn save(&self) {
        let dirs = ProjectDirs::from(APP_QUALIFIER, APP_ORGANIZATION, APP_NAME).unwrap();
        let config_dir = dirs.config_dir();
        let json = to_string_pretty(self).expect("Should serialize settings to json");
        create_dir_all(config_dir).expect("Should make sure the settings save dir exists");
        File::create(config_dir.join("settings.json"))
            .unwrap()
            .write_all(json.as_bytes())
            .expect("Should write settings to file");
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self::try_load().unwrap_or(Self {
            caster_save_dir: env::current_dir()
                .expect("Couldn't get the current working directory")
                .join("capture"),
            player_save_dir: Some(UserDirs::new().unwrap().video_dir().unwrap().to_path_buf()),
            player_save_enabled: true,
        })
    }
}
