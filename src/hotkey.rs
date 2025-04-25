use egui::{Context, Event, Key, Modifiers, Ui};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::gui::Gui;

pub struct HotkeyManager {
    bindings: HashMap<(Modifiers, Key), Rc<RefCell<dyn FnMut(&mut Gui, &Context, &Ui) -> ()>>>,
}

impl Default for HotkeyManager {
    fn default() -> Self {
        let mut bindings = HashMap::new();
        bindings.insert(
            (Modifiers::CTRL, Key::P),
            Rc::new(RefCell::new(|gui: &mut Gui, ctx: &Context, ui: &Ui| {
                println!("CTRL + P was pressed, stopping playback");
                gui.action_stop_playback();
            })) as Rc<RefCell<dyn FnMut(&mut Gui, &Context, &Ui) -> ()>>,
        );
        Self { bindings }
    }
}

impl HotkeyManager {
    pub fn check_keyboard(
        &self,
        ctx: &Context,
    ) -> Vec<Rc<RefCell<dyn FnMut(&mut Gui, &Context, &Ui) -> ()>>> {
        ctx.input(|i| {
            i.events
                .iter()
                .filter_map(|event| match event {
                    Event::Key {
                        key,
                        pressed,
                        modifiers,
                        ..
                    } if *pressed == true => {
                        let query = (modifiers.clone(), key.clone());
                        let a = self.bindings.get(&query);
                        a.cloned()
                    }
                    _ => None,
                })
                .map(|rc| rc.clone())
                .collect::<Vec<Rc<_>>>()
        })
    }
}
