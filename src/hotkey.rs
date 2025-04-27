use crate::alias::KeyCombo;
use egui::{Context, Event, Key, Modifiers};
use std::collections::{HashMap, HashSet};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

#[derive(Clone, Copy, PartialEq, Eq, Hash, EnumIter, Display, Debug)]
pub enum HotkeyAction {
    StopPlayback,
    PlayPlayback,
    PrintHello,
    BackToRoot,
}

#[derive(Debug)]
pub enum BindError {
    KeyComboAlreadyAssigned(HotkeyAction),
    ActionAlreadyAssigned(KeyCombo),
    ActionAlreadyUnbinded,
}

pub enum ManagerState {
    Default,
    Binding(HotkeyAction),
}

pub struct HotkeyManager {
    pub state: ManagerState,
    bindings: HashMap<KeyCombo, HotkeyAction>,
    reverse_bindings_cache: Option<Vec<(HotkeyAction, KeyCombo)>>,
    unbound_actions_cache: Option<Vec<HotkeyAction>>,
}

impl Default for HotkeyManager {
    fn default() -> Self {
        let mut bindings = HashMap::new();
        bindings.insert((Modifiers::MAC_CMD, Key::P), HotkeyAction::PrintHello);
        bindings.insert((Modifiers::MAC_CMD, Key::S), HotkeyAction::StopPlayback);
        bindings.insert((Modifiers::NONE, Key::Space), HotkeyAction::PlayPlayback);
        Self {
            bindings,
            state: ManagerState::Default,
            reverse_bindings_cache: None,
            unbound_actions_cache: None,
        }
    }
}

impl HotkeyManager {
    pub fn new_binding_mode(&mut self, action: HotkeyAction) {
        self.state = ManagerState::Binding(action);
    }

    pub fn check_keyboard(&mut self, ctx: &Context) -> Vec<HotkeyAction> {
        match self.state {
            ManagerState::Default => self.default_behavior(ctx),
            ManagerState::Binding(action) => self.new_binding(ctx, action),
        }
    }

    fn new_binding(&mut self, ctx: &Context, action: HotkeyAction) -> Vec<HotkeyAction> {
        let opt_key_combo = ctx.input(|i| {
            i.events
                .iter()
                .filter_map(|event| match event {
                    Event::Key {
                        key,
                        pressed,
                        repeat,
                        modifiers,
                        ..
                    } if *pressed && !*repeat => Some((modifiers.clone(), key.clone())),
                    _ => None,
                })
                .next()
        });
        if let Some(key_combo) = opt_key_combo {
            let _ = self.try_bind(key_combo, action); //TODO: handle error
        }
        vec![]
    }

    fn default_behavior(&self, ctx: &Context) -> Vec<HotkeyAction> {
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

    pub fn bindings(&mut self) -> Vec<(HotkeyAction, KeyCombo)> {
        match &mut self.reverse_bindings_cache {
            Some(cache) => cache.clone(),
            None => {
                let reverse_bindings = self
                    .bindings
                    .iter()
                    .map(|(k, v)| (v.clone(), k.clone()))
                    .collect::<Vec<_>>();
                self.reverse_bindings_cache = Some(reverse_bindings.clone());
                reverse_bindings
            }
        }
    }

    pub fn unbound_actions(&mut self) -> Vec<HotkeyAction> {
        if self.unbound_actions_cache.is_none() {
            let bound_actions = self
                .bindings
                .iter()
                .map(|(_, action)| action.clone())
                .collect::<HashSet<HotkeyAction>>();
            let unbound_actions = HotkeyAction::iter()
                .collect::<HashSet<HotkeyAction>>()
                .difference(&bound_actions)
                .collect::<Vec<&HotkeyAction>>()
                .into_iter()
                .map(|reference| reference.clone())
                .collect::<Vec<_>>();
            self.unbound_actions_cache = Some(unbound_actions);
        }
        self.unbound_actions_cache.clone().unwrap()
    }

    fn reverse_search(&self, action: HotkeyAction) -> Option<&KeyCombo> {
        self.bindings
            .iter()
            .filter(|(_, act)| *act == &action)
            .map(|(key_combo, _)| key_combo)
            .next()
    }

    fn try_bind(&mut self, combo: KeyCombo, action: HotkeyAction) -> Result<(), BindError> {
        match self.bindings.get(&combo) {
            Some(action) => return Err(BindError::KeyComboAlreadyAssigned(*action)),
            None => (),
        };
        match self.reverse_search(action) {
            Some(key_combo) => return Err(BindError::ActionAlreadyAssigned(*key_combo)),
            None => (),
        };
        self.reverse_bindings_cache = None;
        self.unbound_actions_cache = None;
        self.bindings.insert(combo, action);
        Ok(())
    }

    pub fn try_unbind(&mut self, action: HotkeyAction) -> Result<(), BindError> {
        let opt_key_combo = self
            .reverse_search(action)
            .map(|opt_content| opt_content.clone());
        match opt_key_combo {
            Some(combo) => {
                self.reverse_bindings_cache = None;
                self.unbound_actions_cache = None;
                self.bindings.remove(&combo);
            }
            None => return Err(BindError::ActionAlreadyUnbinded),
        }
        Ok(())
    }
}
