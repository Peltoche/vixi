use std::collections::HashMap;

use crate::devices::keyboard::KeyStroke;

use failure::Error;

pub type Config = HashMap<String, String>;

#[derive(Debug, Clone, Copy)]
pub enum Verb {
    Delete,
}

pub struct Verbs(HashMap<KeyStroke, Verb>);

impl Default for Verbs {
    fn default() -> Self {
        let mut verbs = HashMap::with_capacity(1);

        verbs.insert(KeyStroke::Char('d'), Verb::Delete);

        Self(verbs)
    }
}

impl Verbs {
    #[allow(dead_code)]
    pub fn from_config(config_map: &Config) -> Result<Self, Error> {
        let mut verbs = HashMap::with_capacity(config_map.len());

        for (key_desc, verb_name) in config_map.iter() {
            let keystroke = KeyStroke::from_description(&key_desc)
                .ok_or_else(|| format_err!("failed to parse the key {}", key_desc))?;

            let verb = match verb_name.as_str() {
                "delete" => Verb::Delete,
                _ => return Err(format_err!("unknown verb {}", verb_name)),
            };

            verbs.insert(keystroke, verb);
        }

        Ok(Self(verbs))
    }

    //pub fn get(&self, key: KeyStroke) -> Option<Verb> {
    //if let Some(key) = self.0.get(&key) {
    //Some(*key)
    //} else {
    //None
    //}
    //}
}
