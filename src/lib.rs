pub mod capture;
pub mod decoder;
pub mod gui;
pub mod hotkey;
pub mod old_caster;
pub mod old_player;
pub mod playback;
pub mod server;
pub mod settings;
pub mod videocaster;

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
