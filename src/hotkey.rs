use egui::{Context, Event, Key, Modifiers};
use std::collections::HashMap;

pub struct HotkeyManager {
    bindings: HashMap<(Modifiers, Key), fn()>,
}

impl Default for HotkeyManager {
    fn default() -> Self {
        let mut bindings = HashMap::new();
        bindings.insert(
            (Modifiers::CTRL, Key::P),
            (|| {
                println!("CTRL + P was pressed");
            }) as fn(),
        );
        Self { bindings }
    }
}

impl HotkeyManager {
    pub fn check_keyboard(&self, ctx: &Context) {
        ctx.input(|i| {
            i.events.iter().for_each(|event| {
                if let Event::Key {
                    key,
                    pressed,
                    modifiers,
                    ..
                    // physical_key,
                    // repeat,
                } = event
                {
                    if !*pressed {
                        // TODO: handle multiple presses
                        return;
                    }
                    let query = (modifiers.clone(), key.clone());
                    let action = self.bindings.get(&query);
                    action.inspect(|func| func());
                }
            });
        });
    }
}
