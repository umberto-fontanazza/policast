use egui::Context;

use super::{Gui, Route};

impl Gui {
    pub fn check_keyboard(&mut self, ctx: &Context) {
        let actions = self.hotkey.check_keyboard(ctx);
        actions.iter().for_each(|action| match action {
            crate::hotkey::HotkeyAction::StopPlayback => self._action_stop_playback(),
            crate::hotkey::HotkeyAction::PlayPlayback => self._action_play_playback(),
            crate::hotkey::HotkeyAction::BackToRoot => self._action_route_to_root(),
            crate::hotkey::HotkeyAction::RouteBack => self._action_route_back(),
            crate::hotkey::HotkeyAction::OpenSettings => self._action_open_settings(),
            crate::hotkey::HotkeyAction::SelectArea => self._action_select_area(),
        });
    }

    fn _action_stop_playback(&mut self) {
        if self._route != Route::PlayerControls {
            return ();
        }
        self.playback.stop();
    }

    fn _action_play_playback(&mut self) {
        if self._route != Route::PlayerControls {
            return ();
        }
        self.playback.play();
    }

    fn _action_route_to_root(&mut self) {
        while self._route != Route::SelectRole {
            self.route_back();
        }
    }

    fn _action_route_back(&mut self) {
        return self.route_back();
    }

    fn _action_open_settings(&mut self) {
        self.route_to(Route::Settings);
    }

    fn _action_select_area(&mut self) {
        if self._route != Route::CasterControls {
            return ();
        }
        self.capturer.select_area();
    }
}
