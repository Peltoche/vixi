use std::collections::HashMap;

use crate::devices::keyboard::keys::*;
use crate::devices::keyboard::KeyStroke;

use failure::Error;

#[derive(Debug, Copy, Clone)]
pub enum Action {
    Exit,
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
        let mut actions = HashMap::with_capacity(11);

        actions.insert(KEY_F1, Action::Exit);

        // The classic arrow keys.
        actions.insert(KEY_UP, Action::MoveUp);
        actions.insert(KEY_DOWN, Action::MoveDown);
        actions.insert(KEY_LEFT, Action::MoveLeft);
        actions.insert(KEY_RIGHT, Action::MoveRight);
        actions.insert(KEY_PPAGE, Action::PageUp);
        actions.insert(KEY_NPAGE, Action::PageDown);

        // The "vim like" keys.
        actions.insert(
            KeyStroke::from_char('k').unwrap() as KeyStroke,
            Action::MoveUp,
        );
        actions.insert(
            KeyStroke::from_char('j').unwrap() as KeyStroke,
            Action::MoveDown,
        );
        actions.insert(
            KeyStroke::from_char('h').unwrap() as KeyStroke,
            Action::MoveLeft,
        );
        actions.insert(
            KeyStroke::from_char('l').unwrap() as KeyStroke,
            Action::MoveRight,
        );

        Self(actions)
    }
}

impl Actions {
    pub fn from_config(config_map: &Config) -> Result<Self, Error> {
        let mut actions = HashMap::with_capacity(config_map.len());

        for (key_desc, action_name) in config_map.iter() {
            let keystroke = KeyStroke::from_description(&key_desc)
                .ok_or_else(|| format_err!("failed to parse the key {}", key_desc))?;

            let action = match action_name.as_str() {
                "move_up" => Action::MoveUp,
                "move_down" => Action::MoveDown,
                "move_left" => Action::MoveLeft,
                "move_right" => Action::MoveRight,
                "exit" => Action::Exit,
                "page_up" => Action::PageUp,
                "page_down" => Action::PageDown,
                _ => return Err(format_err!("unknown action {}", action_name)),
            };

            actions.insert(keystroke, action);
        }

        Ok(Self(actions))
    }

    pub fn get(&self, key: KeyStroke) -> Option<Action> {
        if let Some(res) = self.0.get(&key) {
            Some(*res)
        } else {
            None
        }
    }
}
