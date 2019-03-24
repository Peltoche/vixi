use std::collections::HashMap;

use crate::controller::actions::Action;
use crate::devices::keyboard::keys::*;
use crate::devices::keyboard::KeyStroke;

#[derive(Debug)]
pub enum Verb {
    Delete,
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
        let mut c= Config::default();

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
    pub fn from_config(config_map: &Config) -> Result<Self, ()> {
        let mut key_map = KeyMap {
            actions: HashMap::new(),
            verbs: HashMap::new(),
            modifiers: HashMap::new(),
            nouns: HashMap::new(),
        };

        for (key_desc, name) in config_map.verbs.iter() {
            let keystroke =
                convert_description_to_keystroke(&key_desc).expect("invalid verb keystroke");

            let verb = match name.as_str() {
                "delete" => Some(Verb::Delete),
                _ => None,
            }
            .expect("invalid verb name");

            key_map.verbs.insert(keystroke, verb);
        }

        for (key_desc, name) in config_map.nouns.iter() {
            let keystroke =
                convert_description_to_keystroke(&key_desc).expect("invalid noun keystroke");

            let noun = match name.as_str() {
                "line" => Some(Noun::Line),
                _ => None,
            }
            .expect("invalid noun name");

            key_map.nouns.insert(keystroke, noun);
        }

        for (key_desc, name) in config_map.actions.iter() {
            let keystroke =
                convert_description_to_keystroke(&key_desc).expect("invalid action keystroke");

            let action = match name.as_str() {
                "move_up" => Some(Action::MoveUp),
                "move_down" => Some(Action::MoveDown),
                "move_left" => Some(Action::MoveLeft),
                "move_right" => Some(Action::MoveRight),
                "exit" => Some(Action::Exit),
                "page_up" => Some(Action::PageUp),
                "page_down" => Some(Action::PageDown),
                _ => None,
            }
            .expect("invalid action name");

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
