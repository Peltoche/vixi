mod insert_mode;
mod normal_mode;
mod visual_mode;

mod rpc;

use std::sync::{Arc, Mutex};

use self::insert_mode::InsertMode;
use self::normal_mode::NormalMode;
use self::visual_mode::VisualMode;
use crate::core::ClientToClientWriter;
use crate::devices::keyboard::Keyboard;
use crate::devices::terminal::Terminal;

use failure::Error;
use xi_rpc::Peer;

lazy_static! {
    static ref PAST_BUFFER: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));
}

#[derive(Debug, Default, Deserialize)]
pub struct Config {
    #[serde(default)]
    normal_mode: normal_mode::Config,
    #[serde(default)]
    insert_mode: insert_mode::Config,
    #[serde(default)]
    visual_mode: visual_mode::Config,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Mode {
    Normal,
    Insert,
    Visual,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Response {
    Continue,
    Stop,
    SwitchToInsertMode,
    SwitchToNormalMode,
    SwitchToVisualMode,
}

impl Mode {
    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        match self {
            Mode::Normal => String::from("NORMAL"),
            Mode::Insert => String::from("INSERT"),
            Mode::Visual => String::from("VISUAL"),
        }
    }
}

pub struct InputController {
    terminal: Terminal,
    keyboard: Keyboard,
    view_id: String,
    normal_mode: NormalMode,
    insert_mode: InsertMode,
    visual_mode: VisualMode,
    mode: Mode,
    front_event_writer: ClientToClientWriter,
}

impl InputController {
    pub fn new(
        terminal: Terminal,
        keyboard: Keyboard,
        client_to_client_writer: ClientToClientWriter,
        config: &Config,
    ) -> Self {
        Self {
            terminal,
            keyboard,
            view_id: String::new(),
            normal_mode: NormalMode::from(&config.normal_mode),
            insert_mode: InsertMode::from(&config.insert_mode),
            visual_mode: VisualMode::from(&config.visual_mode),
            mode: Mode::Normal,
            front_event_writer: client_to_client_writer,
        }
    }

    pub fn open_file(&mut self, core: &dyn Peer, file_path: &str) -> Result<(), Error> {
        let view_id = core
            .send_rpc_request("new_view", &json!({ "file_path": file_path }))
            .expect("failed to create the new view");

        self.view_id = view_id.as_str().unwrap().to_string();

        let (size_y, _) = self.terminal.get_size();
        core.send_rpc_notification(
            "edit",
            &json!({
                "method": "scroll",
                "view_id": self.view_id,
                "params": [0 ,size_y + 1] // + 1 bc range not inclusive
            }),
        );

        core.send_rpc_notification("set_theme", &json!({"theme_name": "Solarized (light)" }));
        self.front_event_writer.send_rpc_notification(
            "add_status_item",
            &json!({
                "key": "change-mode",
                "value": self.mode.to_string(),
                "alignment": "left",
            }),
        );

        Ok(())
    }

    pub fn start_keyboard_event_loop(&mut self, core: &dyn Peer) -> Result<(), Error> {
        loop {
            let key_res = self.keyboard.get_next_keystroke();

            if let Some(key) = key_res {
                let res = match self.mode {
                    Mode::Normal => self.normal_mode.handle_keystroke(key, &self.view_id, core),
                    Mode::Insert => self.insert_mode.handle_keystroke(key, &self.view_id, core),
                    Mode::Visual => self.visual_mode.handle_keystroke(key, &self.view_id, core),
                };

                match res {
                    Response::Continue => continue,
                    Response::Stop => break,
                    Response::SwitchToInsertMode => self.mode = Mode::Insert,
                    Response::SwitchToNormalMode => self.mode = Mode::Normal,
                    Response::SwitchToVisualMode => self.mode = Mode::Visual,
                }

                self.front_event_writer.send_rpc_notification(
                    "add_status_item",
                    &json!({
                        "key": "change-mode",
                        "value": self.mode.to_string(),
                        "alignment": "left",
                    }),
                );
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Config;

    #[test]
    fn test_config_deserialization() {
        let config: Config = toml::from_str(
            r#"
        visual_mode.actions.move_down  = "key_up"
         "#,
        )
        .unwrap();

        assert_eq!(
            String::from("key_up"),
            config.visual_mode.actions[&String::from("move_down")]
        );
    }

}
