use std::collections::HashMap;

use crate::input_controller::keyboard::KeyStroke;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Verb {
    Delete,
}

impl Verb {
    pub fn from_description(desc: &str) -> Option<Verb> {
        match desc {
            "delete" => Some(Verb::Delete),
            _ => None,
        }
    }
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
    pub fn get(&self, key: KeyStroke) -> Option<Verb> {
        if let Some(key) = self.0.get(&key) {
            Some(*key)
        } else {
            None
        }
    }

    pub fn insert(&mut self, key: KeyStroke, verb: Verb) {
        self.0.insert(key, verb);
    }
}

impl From<&HashMap<String, String>> for Verbs {
    fn from(config_map: &HashMap<String, String>) -> Self {
        let mut verbs = Verbs::default();

        for (verb_name, key_desc) in config_map.iter() {
            let key_res = KeyStroke::from_description(&key_desc);
            if key_res.is_none() {
                continue;
            }

            let verb_res = Verb::from_description(verb_name);
            if verb_res.is_none() {
                continue;
            }

            verbs.insert(key_res.unwrap(), verb_res.unwrap());
        }

        verbs
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::{Verb, Verbs};
    use crate::input_controller::keyboard::KeyStroke;

    #[test]
    fn created_with_an_empty_config_keeps_the_default_values() {
        // The config is empty
        let verbs = Verbs::from(&HashMap::new());

        // It load the default configs.
        assert_eq!(Some(Verb::Delete), verbs.get(KeyStroke::Char('d')));
    }
}
