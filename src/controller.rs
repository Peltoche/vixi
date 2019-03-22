use std::char;
use std::collections::HashMap;

use ncurses::*;
use xi_rpc::Peer;

#[derive(Default)]
pub struct Controller {
    view_id: String,
    size_y: i32,
    size_x: i32,
}

#[derive(Deserialize, Debug)]
pub struct ConfigMap(HashMap<String, String>);

lazy_static! {
    static ref DEFAULT_CONFIG_MAP: HashMap<String, String> = {
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

impl Controller {
    pub fn open_file(&mut self, core: Box<dyn Peer>, file_path: &str) {
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

        core.send_rpc_notification(
            "edit",
            &json!({
            "method": "scroll",
            "view_id": self.view_id,
            "params": [0 , self.size_y]
            }),
        );
    }

    pub fn start_keyboard_event_loop(&self, core: Box<dyn Peer>) {
        loop {
            let ch = getch();
            match ch {
                KEY_F1 => break,
                KEY_UP => {
                    core.send_rpc_notification(
                        "edit",
                        &json!({ "method": "move_up", "view_id": self.view_id}),
                    );
                }
                KEY_DOWN => {
                    core.send_rpc_notification(
                        "edit",
                        &json!({ "method": "move_down", "view_id": self.view_id}),
                    );
                }
                KEY_LEFT => {
                    core.send_rpc_notification(
                        "edit",
                        &json!({ "method": "move_left", "view_id": self.view_id}),
                    );
                }
                KEY_RIGHT => {
                    core.send_rpc_notification(
                        "edit",
                        &json!({ "method": "move_right", "view_id": self.view_id}),
                    );
                }
                _ => (),
            }

            match char::from_u32(ch as u32).expect("Invalid char") {
                'i' => {
                    core.send_rpc_notification(
                        "edit",
                        &json!({
                            "method": "insert",
                            "params": {
                                "chars": "!",
                            },
                            "view_id": self.view_id}),
                    );
                }
                _ => (),
            }
        }
    }
}
