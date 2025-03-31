pub mod alias;
pub mod capturer;
pub mod crop;
pub mod decoder;
pub mod ffmpeg;
pub mod gui;
pub mod hotkey;
pub mod playback;
pub mod screen;
pub mod server;
pub mod settings;
pub mod util;

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
