use std::collections::HashMap;

use crate::input_controller::keyboard::KeyStroke;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Action {
    ExitActionMode,
    WriteToFile,
    Quite,
}

impl Action {
    pub fn from_description(desc: &str) -> Option<Action> {
        match desc {
            "quite" => Some(Action::Quite),
            "write_to_file" => Some(Action::WriteToFile),
            "exit_action_mode" => Some(Action::ExitActionMode),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct Actions(HashMap<KeyStroke, Action>);

impl Default for Actions {
    fn default() -> Self {
        let mut actions = HashMap::with_capacity(1);

        actions.insert(KeyStroke::KeyEscape, Action::ExitActionMode);
        actions.insert(KeyStroke::Char('q'), Action::Quite);
        actions.insert(KeyStroke::Char('w'), Action::WriteToFile);

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
        assert_eq!(Some(Action::Quite), actions.get(KeyStroke::Char('q')));
    }

    #[test]
    fn a_config_value_override_the_defaults() {
        // The default for "key_up" is "move_up" but the config should modifiate
        // the behavior.
        let config: HashMap<String, String> = [(String::from("quite"), String::from("<key_up>"))]
            .iter()
            .cloned()
            .collect();

        let actions = Actions::from(&config);

        assert_eq!(Some(Action::Quite), actions.get(KeyStroke::KeyUp));
    }

    #[test]
    fn an_invalid_config_value_doesnt_change_the_default_config() {
        // The default for "key_up" is "move_up" but the config should modifiate
        // the behavior.
        let config: HashMap<String, String> =
            [(String::from("quite"), String::from("invalid_key"))]
                .iter()
                .cloned()
                .collect();

        let actions = Actions::from(&config);

        assert_eq!(Some(Action::Quite), actions.get(KeyStroke::Char('q')));
    }
}
