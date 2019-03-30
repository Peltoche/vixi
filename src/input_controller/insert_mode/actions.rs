use std::char::*;
use std::collections::HashMap;

use crate::devices::keyboard::keys::*;
use crate::devices::keyboard::KeyStroke;

use failure::Error;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Action {
    SwitchToNormalMode,
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    PageUp,
    PageDown,
}

pub type Config = HashMap<String, String>;

pub struct Actions(HashMap<KeyStroke, Action>);

impl Default for Actions {
    fn default() -> Self {
        let mut actions = HashMap::with_capacity(1);

        actions.insert(KeyStroke('i'), Action::SwitchToNormalMode);

        // The classic arrow keys.
        actions.insert(KeyStroke(from_u32(KEY_UP).unwrap()), Action::MoveUp);
        actions.insert(KeyStroke(from_u32(KEY_DOWN).unwrap()), Action::MoveDown);
        actions.insert(KeyStroke(from_u32(KEY_LEFT).unwrap()), Action::MoveLeft);
        actions.insert(KeyStroke(from_u32(KEY_RIGHT).unwrap()), Action::MoveRight);
        actions.insert(KeyStroke(from_u32(KEY_PPAGE).unwrap()), Action::PageUp);
        actions.insert(KeyStroke(from_u32(KEY_NPAGE).unwrap()), Action::PageDown);

        Self(actions)
    }
}

impl Actions {
    #[allow(dead_code)]
    pub fn from_config(config_map: &Config) -> Result<Self, Error> {
        let mut actions = HashMap::with_capacity(config_map.len());

        for (key_desc, action_name) in config_map.iter() {
            let keystroke = KeyStroke::from_description(&key_desc)
                .ok_or_else(|| format_err!("failed to parse the key {}", key_desc))?;

            let action = match action_name.as_str() {
                "swtich_to_normal_mode" => Action::SwitchToNormalMode,
                "move_up" => Action::MoveUp,
                "move_down" => Action::MoveDown,
                "move_left" => Action::MoveLeft,
                "move_right" => Action::MoveRight,
                "page_up" => Action::PageUp,
                "page_down" => Action::PageDown,
                _ => return Err(format_err!("unknown action {}", action_name)),
            };

            actions.insert(keystroke, action);
        }

        Ok(Self(actions))
    }

    pub fn get(&self, key: KeyStroke) -> Option<Action> {
        if let Some(key) = self.0.get(&key) {
            Some(*key)
        } else {
            None
        }
    }
}
