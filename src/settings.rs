use directories::UserDirs;
use std::env;
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
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            caster_save_dir: env::current_dir()
                .expect("Couldn't get the current working directory")
                .join("capture"),
            player_save_dir: Some(UserDirs::new().unwrap().video_dir().unwrap().to_path_buf()),
            player_save_enabled: true,
        }
    }
}
