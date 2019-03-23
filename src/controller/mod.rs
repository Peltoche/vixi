pub mod config_map;
mod key_map;

use self::config_map::ConfigMap;
use self::key_map::KeyMap;

use ncurses::*;
use xi_rpc::Peer;

#[derive(Default)]
pub struct Controller {
    view_id: String,
    size_y: i32,
    size_x: i32,
}

impl Controller {
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

        getmaxyx(stdscr(), &mut self.size_y, &mut self.size_x);
        core.send_rpc_notification(
            "edit",
            &json!({
                "method": "resize",
                "view_id": self.view_id,
                "params": {
                    "width": self.size_x,
                    "height": self.size_y,
                }
            }),
        );

        // Paint all the screen with the black color in order to set an uniform
        // background color.
        //
        // TODO: make the background color configurable.
        bkgd(' ' as chtype | COLOR_PAIR(COLOR_BLACK) as chtype);

        core.send_rpc_notification(
            "edit",
            &json!({
                "method": "scroll",
                "view_id": self.view_id,
                "params": [0 , self.size_y + 1] // + 1 bc range not inclusive
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

    pub fn start_keyboard_event_loop(&self, core: &dyn Peer, config_map: &ConfigMap) {
        let key_map = KeyMap::from_config(config_map).expect("failed to create the key map");

        loop {
            let key = getch();

            if let Some(handler) = key_map.get_handler_for_key(key) {
                let should_continue = handler(&self.view_id, core.clone());

                if !should_continue {
                    break;
                }
            }
        }
    }
}
