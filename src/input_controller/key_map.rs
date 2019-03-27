use std::collections::HashMap;

use crate::devices::keyboard::keys::*;
use crate::devices::keyboard::KeyStroke;
use crate::input_controller::actions::Action;
use crate::input_controller::verbs::Verb;

use failure::Error;

#[derive(Debug, Fail)]
pub enum KeyMapError {
    #[fail(display = "Invalid keystroke \"{}\" for \"{}\"", target, key)]
    InvalidKeystroke { key: String, target: String },
    #[fail(display = "Invalid action \"{}\" for key \"{}\"", target, key)]
    InvalidTarget { key: String, target: String },
}

#[derive(Debug)]
pub enum Noun {
    Line,
}

#[derive(Debug)]
pub enum Modifier {}

#[derive(Default)]
pub struct Config {
    pub actions: HashMap<String, String>,
    pub verbs: HashMap<String, String>,
    #[allow(dead_code)]
    pub modifiers: HashMap<String, String>,
    pub nouns: HashMap<String, String>,
}

lazy_static! {
    pub static ref DEFAULT_CONFIG: Config = {
        let mut c = Config::default();

        //
        // Action keys
        //
        c.actions.insert(String::from("f1"), String::from("exit"));

        // The classic arrow keys.
        c.actions.insert(String::from("key_up"), String::from("move_up"));
        c.actions.insert(String::from("key_down"), String::from("move_down"));
        c.actions.insert(String::from("key_left"), String::from("move_left"));
        c.actions.insert(String::from("key_right"), String::from("move_right"));
        c.actions.insert(String::from("page_up"), String::from("page_up"));
        c.actions.insert(String::from("page_down"), String::from("page_down"));

        // The "vim like" keys.
        c.actions.insert(String::from("k"), String::from("move_up"));
        c.actions.insert(String::from("j"), String::from("move_down"));
        c.actions.insert(String::from("h"), String::from("move_left"));
        c.actions.insert(String::from("l"), String::from("move_right"));

        //
        // Verb keys
        //
        c.verbs.insert(String::from("d"), String::from("delete"));

        //
        // Nouns keys
        //
        c.nouns.insert(String::from("l"), String::from("line"));

        c
    };
}

pub struct KeyMap {
    pub actions: HashMap<KeyStroke, Action>,
    pub verbs: HashMap<KeyStroke, Verb>,
    #[allow(dead_code)]
    pub modifiers: HashMap<KeyStroke, Modifier>,
    pub nouns: HashMap<KeyStroke, Noun>,
}

impl KeyMap {
    pub fn from_config(config_map: &Config) -> Result<Self, Error> {
        let mut key_map = KeyMap {
            actions: HashMap::new(),
            verbs: HashMap::new(),
            modifiers: HashMap::new(),
            nouns: HashMap::new(),
        };

        for (key, name) in config_map.verbs.iter() {
            let keystroke =
                convert_description_to_keystroke(&key).ok_or(KeyMapError::InvalidKeystroke {
                    key: key.clone(),
                    target: name.clone(),
                })?;

            let verb = match name.as_str() {
                "delete" => Verb::Delete,
                _ => {
                    return Err(KeyMapError::InvalidTarget {
                        key: key.clone(),
                        target: name.clone(),
                    }
                    .into());
                }
            };

            key_map.verbs.insert(keystroke, verb);
        }

        for (key, name) in config_map.nouns.iter() {
            let keystroke =
                convert_description_to_keystroke(&key).ok_or(KeyMapError::InvalidKeystroke {
                    key: key.clone(),
                    target: name.clone(),
                })?;

            let noun = match name.as_str() {
                "line" => Noun::Line,
                _ => {
                    return Err(KeyMapError::InvalidTarget {
                        key: key.clone(),
                        target: name.clone(),
                    }
                    .into());
                }
            };

            key_map.nouns.insert(keystroke, noun);
        }

        for (key, name) in config_map.actions.iter() {
            let keystroke =
                convert_description_to_keystroke(&key).ok_or(KeyMapError::InvalidKeystroke {
                    key: key.clone(),
                    target: name.clone(),
                })?;

            let action = match name.as_str() {
                "move_up" => Action::MoveUp,
                "move_down" => Action::MoveDown,
                "move_left" => Action::MoveLeft,
                "move_right" => Action::MoveRight,
                "exit" => Action::Exit,
                "page_up" => Action::PageUp,
                "page_down" => Action::PageDown,
                _ => {
                    return Err(KeyMapError::InvalidTarget {
                        key: key.clone(),
                        target: name.clone(),
                    }
                    .into());
                }
            };

            key_map.actions.insert(keystroke, action);
        }

        Ok(key_map)
    }
}

fn convert_description_to_keystroke(description: &str) -> Option<KeyStroke> {
    if description.len() == 1 {
        return Some(description.chars().nth(0).unwrap() as i32);
    }

    match description {
        "f1" => Some(KEY_F1),
        "key_up" => Some(KEY_UP),
        "key_down" => Some(KEY_DOWN),
        "key_left" => Some(KEY_LEFT),
        "key_right" => Some(KEY_RIGHT),
        "page_up" => Some(KEY_PPAGE),
        "page_down" => Some(KEY_NPAGE),
        _ => None,
    }
}
