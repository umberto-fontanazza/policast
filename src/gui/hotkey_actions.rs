use egui::Context;

use super::{Gui, Route};

impl Gui {
    pub fn check_keyboard(&mut self, ctx: &Context) {
        let actions = self.hotkey.check_keyboard(ctx);
        actions.iter().for_each(|action| match action {
            crate::hotkey::HotkeyAction::StopPlayback => self._action_stop_playback(),
            crate::hotkey::HotkeyAction::PlayPlayback => self._action_play_playback(),
            crate::hotkey::HotkeyAction::BackToRoot => self._action_route_to_root(),
            crate::hotkey::HotkeyAction::PrintHello => {
                println!("Hello there")
            }
        });
    }

    fn _action_stop_playback(&mut self) {
        self.playback.stop();
    }

    fn _action_play_playback(&mut self) {
        self.playback.play(None);
    }

    fn _action_route_to_root(&mut self) {
        return self.route_to(Route::SelectRole);
    }
}
