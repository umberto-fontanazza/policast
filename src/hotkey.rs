use crate::{
    alias::KeyCombo,
    settings::{APP_NAME, APP_ORGANIZATION, APP_QUALIFIER},
};
use directories::ProjectDirs;
use egui::{Context, Event, Key, Modifiers};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string_pretty};
use serde_json_any_key::any_key_map;
use std::{
    collections::{HashMap, HashSet},
    fs::{create_dir_all, File},
    io::{Read, Write},
};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

const SAVED_CONFIG_FILENAME: &str = "hotkey-bindings.json";

#[derive(Clone, Copy, PartialEq, Eq, Hash, EnumIter, Display, Debug, Serialize, Deserialize)]
pub enum HotkeyAction {
    // Caster actions
    OpenSettings,
    SelectArea,
    // Player actions
    StopPlayback,
    PlayPlayback,
    // Common
    BackToRoot,
    RouteBack,
}

#[derive(Debug)]
pub enum BindError {
    KeyComboAlreadyAssigned(HotkeyAction),
    ActionAlreadyAssigned(KeyCombo),
    ActionAlreadyUnbinded,
}

#[derive(Display, Serialize, Deserialize)]
pub enum ManagerState {
    Default,
    Binding(HotkeyAction),
}

#[derive(Serialize, Deserialize)]
pub struct HotkeyManager {
    pub state: ManagerState,
    #[serde(with = "any_key_map")]
    bindings: HashMap<KeyCombo, HotkeyAction>,
    reverse_bindings_cache: Option<Vec<(HotkeyAction, KeyCombo)>>,
    unbound_actions_cache: Option<Vec<HotkeyAction>>,
}

impl Default for HotkeyManager {
    fn default() -> Self {
        let mut bindings = HashMap::new();
        bindings.insert((Modifiers::MAC_CMD, Key::S), HotkeyAction::StopPlayback);
        bindings.insert((Modifiers::NONE, Key::Space), HotkeyAction::PlayPlayback);
        bindings.insert((Modifiers::NONE, Key::Backspace), HotkeyAction::RouteBack);
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

    fn try_load() -> Result<Self, ()> {
        let dirs = ProjectDirs::from(APP_QUALIFIER, APP_ORGANIZATION, APP_NAME).unwrap();
        let file_path = dirs.config_dir().join(SAVED_CONFIG_FILENAME);
        if !file_path.is_file() {
            return Err(());
        }
        let mut file = File::open(&file_path).map_err(|_| ())?;
        let mut file_content = String::new();
        file.read_to_string(&mut file_content).map_err(|_| ())?;
        from_str::<HotkeyManager>(&file_content).map_err(|_| ())
    }

    pub fn save(&self) {
        let dirs = ProjectDirs::from(APP_QUALIFIER, APP_ORGANIZATION, APP_NAME).unwrap();
        let config_dir = dirs.config_dir();
        let json = to_string_pretty(self).expect("Should serialize settings to json");
        create_dir_all(config_dir).expect("Should make sure the settings save dir exists");
        File::create(config_dir.join(SAVED_CONFIG_FILENAME))
            .unwrap()
            .write_all(json.as_bytes())
            .expect("Should write settings to file");
    }

    pub fn load_or_default() -> Self {
        Self::try_load().unwrap_or(Self::default())
    }
}
