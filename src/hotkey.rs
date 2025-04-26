use crate::alias::KeyCombo;
use egui::{Context, Event, Key, Modifiers};
use std::collections::{HashMap, HashSet};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

#[derive(Clone, Copy, PartialEq, Eq, Hash, EnumIter, Display)]
pub enum HotkeyAction {
    StopPlayback,
    PlayPlayback,
    PrintHello,
    BackToRoot,
}

pub enum BindError {
    KeyComboAlreadyAssigned(HotkeyAction),
    ActionAlreadyAssigned(KeyCombo),
    ActionAlreadyUnbinded,
}

pub struct HotkeyManager {
    pub enabled: bool,
    bindings: HashMap<KeyCombo, HotkeyAction>,
}

impl Default for HotkeyManager {
    fn default() -> Self {
        let mut bindings = HashMap::new();
        bindings.insert((Modifiers::MAC_CMD, Key::P), HotkeyAction::PrintHello);
        bindings.insert((Modifiers::MAC_CMD, Key::S), HotkeyAction::StopPlayback);
        bindings.insert((Modifiers::NONE, Key::Space), HotkeyAction::PlayPlayback);
        Self {
            bindings,
            enabled: true,
        }
    }
}

impl HotkeyManager {
    pub fn check_keyboard(&self, ctx: &Context) -> Vec<HotkeyAction> {
        if !self.enabled {
            return vec![];
        }
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

    pub fn bindings(&self) -> HashMap<HotkeyAction, KeyCombo> {
        self.bindings
            .iter()
            .map(|(k, v)| (v.clone(), k.clone()))
            .collect::<HashMap<HotkeyAction, KeyCombo>>()
    }

    pub fn unbinded_actions(&self) -> HashSet<HotkeyAction> {
        let mut all_actions = HotkeyAction::iter().collect::<HashSet<_>>();
        self.bindings
            .iter()
            .map(|(_, action)| action.clone())
            .for_each(|action| {
                all_actions.remove(&action);
            });
        all_actions
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
