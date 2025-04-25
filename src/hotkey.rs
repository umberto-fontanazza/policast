use egui::{Context, Event, Key, Modifiers};
use std::collections::HashMap;

#[derive(Clone, Copy)]
pub enum HotkeyAction {
    StopPlayback,
    PlayPlayback,
    PrintHello,
}

pub struct HotkeyManager {
    bindings: HashMap<(Modifiers, Key), HotkeyAction>,
}

impl Default for HotkeyManager {
    fn default() -> Self {
        let mut bindings = HashMap::new();
        bindings.insert((Modifiers::CTRL, Key::P), HotkeyAction::PrintHello);
        Self { bindings }
    }
}

impl HotkeyManager {
    pub fn check_keyboard(&self, ctx: &Context) -> Vec<HotkeyAction> {
        ctx.input(|i| {
            i.events
                .iter()
                .filter_map(|event| match event {
                    Event::Key {
                        key,
                        pressed,
                        modifiers,
                        ..
                    } if *pressed => Some((modifiers.clone(), key.clone())),
                    _ => None,
                })
                .filter_map(|ref search_key| self.bindings.get(search_key).copied())
                .collect::<Vec<HotkeyAction>>()
        })
    }
}
