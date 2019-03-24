use std::collections::HashMap;

#[derive(Default)]
pub struct ConfigMap {
    pub actions: HashMap<String, String>,
    pub verbs: HashMap<String, String>,
    #[allow(dead_code)]
    pub modifiers: HashMap<String, String>,
    pub nouns: HashMap<String, String>,
}

lazy_static! {
    pub static ref DEFAULT_CONFIG_MAP: ConfigMap = {
        let mut c= ConfigMap::default();

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
