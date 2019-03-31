use std::collections::HashMap;

use crate::devices::keyboard::KeyStroke;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Noun {
    Line,
}

impl Noun {
    pub fn from_description(desc: &str) -> Option<Noun> {
        match desc {
            "line" => Some(Noun::Line),
            _ => None,
        }
    }
}

pub struct Nouns(HashMap<KeyStroke, Noun>);

impl Default for Nouns {
    fn default() -> Self {
        let mut nouns = HashMap::with_capacity(1);

        nouns.insert(KeyStroke::Char('l'), Noun::Line);

        Self(nouns)
    }
}

impl Nouns {
    #[allow(dead_code)]
    pub fn get(&self, key: KeyStroke) -> Option<Noun> {
        if let Some(key) = self.0.get(&key) {
            Some(*key)
        } else {
            None
        }
    }

    pub fn insert(&mut self, key: KeyStroke, noun: Noun) {
        self.0.insert(key, noun);
    }
}

impl From<&HashMap<String, String>> for Nouns {
    fn from(config_map: &HashMap<String, String>) -> Self {
        let mut nouns = Nouns::default();

        for (noun_name, key_desc) in config_map.iter() {
            let key_res = KeyStroke::from_description(&key_desc);
            if key_res.is_none() {
                continue;
            }

            let noun_res = Noun::from_description(noun_name);
            if noun_res.is_none() {
                continue;
            }

            nouns.insert(key_res.unwrap(), noun_res.unwrap());
        }

        nouns
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::{Noun, Nouns};
    use crate::devices::keyboard::KeyStroke;

    #[test]
    fn created_with_an_empty_config_keeps_the_default_values() {
        // The config is empty
        let nouns = Nouns::from(&HashMap::new());

        // It load the default configs.
        assert_eq!(Some(Noun::Line), nouns.get(KeyStroke::Char('l')));
    }
}
