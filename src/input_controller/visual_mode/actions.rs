use std::collections::HashMap;

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

        actions.insert(KeyStroke::KeyEscape, Action::SwitchToNormalMode);

        // The classic arrow keys.
        actions.insert(KeyStroke::KeyUp, Action::MoveUp);
        actions.insert(KeyStroke::KeyDown, Action::MoveDown);
        actions.insert(KeyStroke::KeyLeft, Action::MoveLeft);
        actions.insert(KeyStroke::KeyRight, Action::MoveRight);
        actions.insert(KeyStroke::KeyPreviousPage, Action::PageUp);
        actions.insert(KeyStroke::KeyNextPage, Action::PageDown);

        // The "vim like" keys.
        actions.insert(KeyStroke::Char('k'), Action::MoveUp);
        actions.insert(KeyStroke::Char('j'), Action::MoveDown);
        actions.insert(KeyStroke::Char('h'), Action::MoveLeft);
        actions.insert(KeyStroke::Char('l'), Action::MoveRight);

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
                "switch_to_normal_mode" => Action::SwitchToNormalMode,
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
