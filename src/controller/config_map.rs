use std::collections::HashMap;

pub type ConfigMap = HashMap<String, String>;

lazy_static! {
    pub static ref DEFAULT_CONFIG_MAP: ConfigMap = {
        let mut c = HashMap::new();
        c.insert(String::from("f1"), String::from("exit"));

        // The classic arrow keys
        c.insert(String::from("key_up"), String::from("move_up"));
        c.insert(String::from("key_down"), String::from("move_down"));
        c.insert(String::from("key_left"), String::from("move_left"));
        c.insert(String::from("key_right"), String::from("move_right"));

        // The "vim like" keys
        c.insert(String::from("k"), String::from("move_up"));
        c.insert(String::from("j"), String::from("move_down"));
        c.insert(String::from("h"), String::from("move_left"));
        c.insert(String::from("l"), String::from("move_right"));

        c
    };
}
