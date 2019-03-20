use std::process::ChildStdin;

use ncurses::*;
use xi_rpc::RpcPeer;

#[derive(Default)]
pub struct Controller {
    view_id: String,
}

impl Controller {
    pub fn open_file(&mut self, core: &RpcPeer<ChildStdin>, file_path: &str) {
        core.send_rpc_notification("client_started", &json!({}));

        let view_id = core
            .send_rpc_request("new_view", &json!({ "file_path": file_path }))
            .expect("failed to create the new view");

        self.view_id = view_id.as_str().unwrap().to_string();

        let mut size_y: i32 = 0;
        let mut size_x: i32 = 0;
        getmaxyx(stdscr(), &mut size_y, &mut size_x);
        core.send_rpc_notification(
            "edit",
            &json!({
                "method": "resize",
                "view_id": self.view_id,
                "params": {
                    "width": size_y,
                    "height": size_x,
                }
            }),
        );
    }

    pub fn start_keyboard_event_loop(&self, core: &RpcPeer<ChildStdin>) {
        loop {
            match getch() {
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
        }
    }
}
