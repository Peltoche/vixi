mod actions;
pub mod key_map;

use self::actions::Response;
use self::key_map::{Config, KeyMap, Noun, Verb};
use crate::devices::keyboard::Keyboard;
use crate::devices::terminal::{RGBColor, Terminal};

use xi_rpc::Peer;

pub struct Controller {
    terminal: Terminal,
    keyboard: Keyboard,
    view_id: String,
}

impl Controller {
    pub fn new(terminal: Terminal, keyboard: Keyboard) -> Self {
        Self {
            terminal,
            keyboard,
            view_id: String::new(),
        }
    }

    pub fn open_file(&mut self, core: &dyn Peer, file_path: &str) {
        let mut xi_config_dir = dirs::config_dir().expect("failed to retrieve your config dir");
        xi_config_dir.push("xi");

        core.send_rpc_notification(
            "client_started",
            &json!({ "config_dir": xi_config_dir.to_str().unwrap(), }),
        );

        let view_id = core
            .send_rpc_request("new_view", &json!({ "file_path": file_path }))
            .expect("failed to create the new view");

        self.view_id = view_id.as_str().unwrap().to_string();

        // Paint all the screen with the black color in order to set an uniform
        // background color.
        //
        // TODO: make the background color configurable.
        self.terminal
            .set_background_color(RGBColor { r: 0, g: 0, b: 0 });

        let (size_y, _) = self.terminal.get_size();
        core.send_rpc_notification(
            "edit",
            &json!({
                "method": "scroll",
                "view_id": self.view_id,
                "params": [0 ,size_y + 1] // + 1 bc range not inclusive
            }),
        );

        core.send_rpc_notification(
            "set_language",
            &json!({
            "language_id": "Rust",
            "view_id": self.view_id,
            }),
        );
    }

    pub fn start_keyboard_event_loop(&self, core: &dyn Peer, config_map: &Config) {
        let key_map = KeyMap::from_config(config_map).expect("failed to create the key map");

        loop {
            let key = self.keyboard.get_next_keystroke();

            if let Some(action) = key_map.actions.get(&key) {
                match actions::run(action, self.view_id.as_str(), core) {
                    Response::Continue => continue,
                    Response::Stop => break,
                }
            }

            if let Some(verb) = key_map.verbs.get(&key) {
                let key2 = self.keyboard.get_next_keystroke();

                if let Some(noun) = key_map.nouns.get(&key2) {
                    match verb {
                        Verb::Delete => delete(noun),
                    }
                    continue;
                }

                continue;
            }
        }
    }
}

fn delete(noun: &Noun) {
    info!("delete: {:?}", noun);
}
