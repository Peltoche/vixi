use std::collections::HashMap;

use crate::controller::config_map::ConfigMap;

use ncurses::*;
use xi_rpc::Peer;

pub type KeyHandler = fn(view_id: &str, &Box<dyn Peer>) -> bool;

pub struct KeyMap(HashMap<i32, KeyHandler>);

impl KeyMap {
    pub fn from_config(config_map: &ConfigMap) -> Result<Self, ()> {
        let mut key_map = HashMap::with_capacity(config_map.len());

        for (input, method) in config_map {
            if input.len() == 1 {
                let key = input
                    .chars()
                    .nth(0)
                    .expect("failed to retrieve the first char of an input keymap")
                    as i32;

                key_map.insert(
                    key,
                    get_handler_from_name(&method)
                        //.ok_or(|| format!("method {} invalid", method))
                        .expect("failed to retrieve the keymap key handler"),
                );
            } else {
                key_map.insert(
                    get_key_from_name(&input).expect("invalid key name"),
                    get_handler_from_name(&method)
                        .expect("failed to retrieve the keymap key handler"),
                );
            }
        }

        Ok(KeyMap(key_map))
    }

    pub fn get_handler_for_key(&self, key: i32) -> Option<&KeyHandler> {
        self.0.get(&key)
    }
}

fn get_key_from_name(name: &str) -> Option<i32> {
    match name {
        "key_up" => Some(KEY_UP),
        "key_down" => Some(KEY_DOWN),
        "key_left" => Some(KEY_LEFT),
        "key_right" => Some(KEY_RIGHT),
        "f1" => Some(KEY_F1),
        _ => None,
    }
}

fn get_handler_from_name(name: &str) -> Option<KeyHandler> {
    match name {
        "move_up" => Some(move_up),
        "move_down" => Some(move_down),
        "move_left" => Some(move_left),
        "move_right" => Some(move_right),
        "exit" => Some(exit),
        _ => None,
    }
}

fn exit(_view_id: &str, _core: &Box<dyn Peer>) -> bool {
    false
}

fn move_up(view_id: &str, core: &Box<dyn Peer>) -> bool {
    core.send_rpc_notification("edit", &json!({ "method": "move_up", "view_id": view_id}));
    true
}

fn move_down(view_id: &str, core: &Box<dyn Peer>) -> bool {
    core.send_rpc_notification("edit", &json!({ "method": "move_down", "view_id": view_id}));
    true
}

fn move_left(view_id: &str, core: &Box<dyn Peer>) -> bool {
    core.send_rpc_notification("edit", &json!({ "method": "move_left", "view_id": view_id}));
    true
}

fn move_right(view_id: &str, core: &Box<dyn Peer>) -> bool {
    core.send_rpc_notification(
        "edit",
        &json!({ "method": "move_right", "view_id": view_id}),
    );
    true
}
