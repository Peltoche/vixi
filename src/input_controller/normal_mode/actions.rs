use std::collections::HashMap;

use crate::input_controller::keyboard::KeyStroke;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Action {
    SwitchToInsertMode,
    SwitchToVisualMode,
    Exit,
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    PageUp,
    PageDown,
    Paste,
    InsertLineBellow,
    InsertLineAbove,
}

impl Action {
    pub fn from_description(desc: &str) -> Option<Action> {
        match desc {
            "swtich_to_insert_mode" => Some(Action::SwitchToInsertMode),
            "swtich_to_visual_mode" => Some(Action::SwitchToVisualMode),
            "move_up" => Some(Action::MoveUp),
            "move_down" => Some(Action::MoveDown),
            "move_left" => Some(Action::MoveLeft),
            "move_right" => Some(Action::MoveRight),
            "exit" => Some(Action::Exit),
            "page_up" => Some(Action::PageUp),
            "page_down" => Some(Action::PageDown),
            "paste" => Some(Action::Paste),
            "insert_line_bellow" => Some(Action::InsertLineBellow),
            "insert_line_above" => Some(Action::InsertLineAbove),
            _ => None,
        }
    }
}

pub struct Actions(HashMap<KeyStroke, Action>);

impl Default for Actions {
    fn default() -> Self {
        let mut actions = HashMap::with_capacity(1);

        actions.insert(KeyStroke::Char('i'), Action::SwitchToInsertMode);
        actions.insert(KeyStroke::Char('v'), Action::SwitchToVisualMode);
        actions.insert(KeyStroke::Char('q'), Action::Exit);
        actions.insert(KeyStroke::Char('p'), Action::Paste);
        actions.insert(KeyStroke::Char('o'), Action::InsertLineBellow);
        actions.insert(KeyStroke::Char('O'), Action::InsertLineAbove);

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
    use crate::input_controller::keyboard::KeyStroke;

    #[test]
    fn created_with_an_empty_config_keeps_the_default_values() {
        // The config is empty
        let actions = Actions::from(&HashMap::new());

        // It load the default configs.
        assert_eq!(Some(Action::MoveUp), actions.get(KeyStroke::KeyUp));
    }

    #[test]
    fn a_config_value_override_the_defaults() {
        // The default for "key_up" is "move_up" but the config should modify
        // the behavior.
        let config: HashMap<String, String> =
            [(String::from("move_down"), String::from("<key_up>"))]
                .iter()
                .cloned()
                .collect();

        let actions = Actions::from(&config);

        assert_eq!(Some(Action::MoveDown), actions.get(KeyStroke::KeyUp));
    }

    #[test]
    fn an_invalid_config_value_doesnt_change_the_default_config() {
        // The default for "key_up" is "move_up" but the config should modify
        // the behavior.
        let config: HashMap<String, String> = [
            (String::from("move_up"), String::from("<key_up>")),
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
