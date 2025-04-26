use crate::alias::KeyCombo;
use egui::{Context, Event, Key, Modifiers};
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq)]
pub enum HotkeyAction {
    StopPlayback,
    PlayPlayback,
    PrintHello,
}

pub enum BindError {
    KeyComboAlreadyAssigned(HotkeyAction),
    ActionAlreadyAssigned(KeyCombo),
    ActionAlreadyUnbinded,
}

pub struct HotkeyManager {
    bindings: HashMap<KeyCombo, HotkeyAction>,
}

impl Default for HotkeyManager {
    fn default() -> Self {
        let mut bindings = HashMap::new();
        bindings.insert((Modifiers::CTRL, Key::P), HotkeyAction::PrintHello);
        bindings.insert((Modifiers::MAC_CMD, Key::P), HotkeyAction::StopPlayback);
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

    fn reverse_search(&self, action: HotkeyAction) -> Option<&KeyCombo> {
        self.bindings
            .iter()
            .filter(|(_, act)| *act == &action)
            .map(|(key_combo, _)| key_combo)
            .next()
    }

    pub fn try_bind(&mut self, combo: KeyCombo, action: HotkeyAction) -> Result<(), BindError> {
        match self.bindings.get(&combo) {
            Some(action) => return Err(BindError::KeyComboAlreadyAssigned(*action)),
            None => (),
        };
        match self.reverse_search(action) {
            Some(key_combo) => return Err(BindError::ActionAlreadyAssigned(*key_combo)),
            None => (),
        };
        self.bindings.insert(combo, action);
        Ok(())
    }

    pub fn try_unbind(&mut self, action: HotkeyAction) -> Result<(), BindError> {
        let opt_key_combo = self
            .reverse_search(action)
            .map(|opt_content| opt_content.clone());
        match opt_key_combo {
            Some(combo) => {
                self.bindings.remove(&combo);
            }
            None => return Err(BindError::ActionAlreadyUnbinded),
        }
        Ok(())
    }
}
