use std::collections::HashMap;

use crate::devices::keyboard::KeyStroke;

use failure::Error;

#[derive(Debug)]
pub enum Noun {
    Line,
}

pub type Config = HashMap<String, String>;

pub struct Nouns(HashMap<KeyStroke, Noun>);

impl Default for Nouns {
    fn default() -> Self {
        let mut nouns = HashMap::with_capacity(1);

        nouns.insert(KeyStroke('l'), Noun::Line);

        Self(nouns)
    }
}

impl Nouns {
    #[allow(dead_code)]
    pub fn from_config(config_map: &Config) -> Result<Self, Error> {
        let mut nouns = HashMap::with_capacity(config_map.len());

        for (key_desc, noun_name) in config_map.iter() {
            let keystroke = KeyStroke::from_description(&key_desc)
                .ok_or_else(|| format_err!("failed to parse the key {}", key_desc))?;

            let noun = match noun_name.as_str() {
                "line" => Noun::Line,
                _ => return Err(format_err!("unknown noun {}", noun_name)),
            };

            nouns.insert(keystroke, noun);
        }

        Ok(Self(nouns))
    }
}
