use std::collections::HashMap;

use crate::devices::keyboard::KeyStroke;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Action {
    ExitSelectionMode,
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Yank,
    Paste,
    //PageUp,
    //PageDown,
}

impl Action {
    pub fn from_description(desc: &str) -> Option<Action> {
        match desc {
            "exit_selection_mode" => Some(Action::ExitSelectionMode),
            "move_up" => Some(Action::MoveUp),
            "move_down" => Some(Action::MoveDown),
            "move_left" => Some(Action::MoveLeft),
            "move_right" => Some(Action::MoveRight),
            //"page_up" => Action::PageUp,
            //"page_down" => Action::PageDown,
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct Actions(HashMap<KeyStroke, Action>);

impl Default for Actions {
    fn default() -> Self {
        let mut actions = HashMap::with_capacity(1);

        actions.insert(KeyStroke::KeyEscape, Action::ExitSelectionMode);
        actions.insert(KeyStroke::Char('y'), Action::Yank);
        actions.insert(KeyStroke::Char('p'), Action::Paste);

        // The classic arrow keys.
        actions.insert(KeyStroke::KeyUp, Action::MoveUp);
        actions.insert(KeyStroke::KeyDown, Action::MoveDown);
        actions.insert(KeyStroke::KeyLeft, Action::MoveLeft);
        actions.insert(KeyStroke::KeyRight, Action::MoveRight);

        // The current Core implementation doesn't fail of the buffer is not
        // already available
        //
        //actions.insert(KeyStroke::KeyPreviousPage, Action::PageUp);
        //actions.insert(KeyStroke::KeyNextPage, Action::PageDown);

        // The "vim like" keys.
        actions.insert(KeyStroke::Char('k'), Action::MoveUp);
        actions.insert(KeyStroke::Char('j'), Action::MoveDown);
        actions.insert(KeyStroke::Char('h'), Action::MoveLeft);
        actions.insert(KeyStroke::Char('l'), Action::MoveRight);

        Self(actions)
    }
}

impl Actions {
    pub fn get(&self, key: KeyStroke) -> Option<Action> {
        if let Some(key) = self.0.get(&key) {
            Some(*key)
        } else {
            None
        }
    }

    pub fn insert(&mut self, key: KeyStroke, action: Action) {
        self.0.insert(key, action);
    }
}

impl From<&HashMap<String, String>> for Actions {
    fn from(config_map: &HashMap<String, String>) -> Self {
        let mut actions = Actions::default();

        for (action_name, key_desc) in config_map.iter() {
            let key_res = KeyStroke::from_description(&key_desc);
            if key_res.is_none() {
                continue;
            }

            let action_res = Action::from_description(action_name);
            if action_res.is_none() {
                continue;
            }

            actions.insert(key_res.unwrap(), action_res.unwrap());
        }

        actions
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::{Action, Actions};
    use crate::devices::keyboard::KeyStroke;

    #[test]
    fn created_with_an_empty_config_keeps_the_default_values() {
        // The config is empty
        let actions = Actions::from(&HashMap::new());

        // It load the default configs.
        assert_eq!(Some(Action::MoveUp), actions.get(KeyStroke::KeyUp));
    }

    #[test]
    fn a_config_value_override_the_defaults() {
        // The default for "key_up" is "move_up" but the config should modifiate
        // the behavior.
        let config: HashMap<String, String> = [(String::from("move_down"), String::from("key_up"))]
            .iter()
            .cloned()
            .collect();

        let actions = Actions::from(&config);

        assert_eq!(Some(Action::MoveDown), actions.get(KeyStroke::KeyUp));
    }

    #[test]
    fn an_invalid_config_value_doesnt_change_the_default_config() {
        // The default for "key_up" is "move_up" but the config should modifiate
        // the behavior.
        let config: HashMap<String, String> = [
            (String::from("move_up"), String::from("key_up")),
            (String::from("move_down"), String::from("invalid_key")),
        ]
        .iter()
        .cloned()
        .collect();

        let actions = Actions::from(&config);

        assert_eq!(Some(Action::MoveDown), actions.get(KeyStroke::KeyDown));
        assert_eq!(Some(Action::MoveUp), actions.get(KeyStroke::KeyUp));
    }
}
