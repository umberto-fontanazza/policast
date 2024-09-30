use egui::{Key, Modifiers};
use std::collections::{HashMap, HashSet};

const HOTKEY_ACTIONS_COUNT: usize = 4;
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum HotkeyAction {
    PauseTransmission,
    ResumeTransmission,
    BlankScreen,
    TerminateSession,
}

pub struct HotkeyManager {
    bindings: HashMap<HotkeyAction, (Modifiers, Vec<Key>)>,
}

impl Default for HotkeyManager {
    fn default() -> Self {
        let mut bindings = HashMap::with_capacity(HOTKEY_ACTIONS_COUNT);
        bindings.insert(
            HotkeyAction::PauseTransmission,
            (Modifiers::CTRL, vec![Key::P]),
        );
        bindings.insert(
            HotkeyAction::ResumeTransmission,
            (Modifiers::default(), vec![Key::R]),
        );
        bindings.insert(
            HotkeyAction::BlankScreen,
            (Modifiers::default(), vec![Key::R]),
        );
        bindings.insert(
            HotkeyAction::TerminateSession,
            (Modifiers::default(), vec![Key::T]),
        );
        Self { bindings }
    }
}

impl HotkeyManager {
    pub fn get(&self, action: HotkeyAction) -> (Modifiers, Vec<Key>) {
        let panic_message: &str =
            &format!("The hotkey for the action {:?} was not configured", action);

        self.bindings.get(&action).expect(panic_message).clone()
    }

    pub fn set(&mut self, action: HotkeyAction, modifiers: Modifiers, keys: Vec<Key>) {
        self.bindings.insert(action, (modifiers, keys));
    }

    pub fn check_hotkey_actions(
        &self,
        modifiers: Modifiers,
        keys: HashSet<Key>,
    ) -> Option<HotkeyAction> {
        self.bindings
            .iter()
            .filter(|(_, mod_n_keys)| {
                let (saved_modifiers, saved_keys) = mod_n_keys;
                let modifiers_match = modifiers.matches_logically(saved_modifiers.clone());
                let keys_match = saved_keys.iter().all(|key| keys.contains(key));
                modifiers_match && keys_match
            })
            .map(|(action, mod_n_keys)| {
                let (saved_modifiers, saved_keys) = mod_n_keys;
                (
                    action.clone(),
                    count_active_modifiers(&saved_modifiers) + saved_keys.len(),
                )
            })
            .max_by_key(|(_, saved_keys_count)| saved_keys_count.to_owned())
            .map(|(action, _)| action)
    }
}

fn count_active_modifiers(modifiers: &Modifiers) -> usize {
    let mut count = if modifiers.ctrl { 1 } else { 0 };
    count += if modifiers.alt { 1 } else { 0 };
    count += if modifiers.shift { 1 } else { 0 };
    count += if modifiers.mac_cmd { 1 } else { 0 };
    count
}
