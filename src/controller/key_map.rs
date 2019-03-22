use std::char;
use std::collections::HashMap;

use crate::controller::config_map::ConfigMap;

use xi_rpc::Peer;

pub type KeyHandler = fn(view_id: &str, &Box<dyn Peer>);

pub struct KeyMap(HashMap<char, KeyHandler>);

impl KeyMap {
    pub fn from_config(config_map: &ConfigMap) -> Result<Self, ()> {
        let mut key_map = HashMap::with_capacity(config_map.len());

        for (input, method) in config_map {
            if input.len() == 1 {
                key_map.insert(
                    input
                        .chars()
                        .nth(0)
                        .expect("failed to retrieve the first char of an input keymap"),
                    get_key_handler(&method)
                        //.ok_or(|| format!("method {} invalid", method))
                        .expect("failed to retrieve the keymap key handler"),
                );
            }
        }

        Ok(KeyMap(key_map))
    }

    pub fn get_handler_for_key(&self, key: char) -> Option<&KeyHandler> {
        self.0.get(&key)
    }
}

fn get_key_handler(name: &str) -> Option<KeyHandler> {
    match name {
        "move_up" => Some(move_up),
        "move_down" => Some(move_down),
        "move_left" => Some(move_left),
        "move_right" => Some(move_right),
        _ => None,
    }
}

fn move_up(view_id: &str, core: &Box<dyn Peer>) {
    core.send_rpc_notification("edit", &json!({ "method": "move_up", "view_id": view_id}));
}

fn move_down(view_id: &str, core: &Box<dyn Peer>) {
    core.send_rpc_notification("edit", &json!({ "method": "move_down", "view_id": view_id}));
}

fn move_left(view_id: &str, core: &Box<dyn Peer>) {
    core.send_rpc_notification("edit", &json!({ "method": "move_left", "view_id": view_id}));
}

fn move_right(view_id: &str, core: &Box<dyn Peer>) {
    core.send_rpc_notification(
        "edit",
        &json!({ "method": "move_right", "view_id": view_id}),
    );
}
