mod actions;
pub mod keyboard;
mod mode_actions;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use self::actions::{Action, Response};
use self::keyboard::{KeyStroke, Keyboard};
use self::mode_actions::ModeActions;
use crate::core::ClientToClientWriter;

use failure::Error;
use xi_rpc::Peer;

lazy_static! {
    static ref PASTE_BUFFER: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));
}

#[derive(Debug, Default, Deserialize)]
pub struct Config {
    #[serde(default)]
    normal_mode: HashMap<String, String>,
    #[serde(default)]
    insert_mode: HashMap<String, String>,
    #[serde(default)]
    visual_mode: HashMap<String, String>,
    #[serde(default)]
    action_mode: HashMap<String, String>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Mode {
    Normal,
    Insert,
    Visual,
    Action,
}

impl Mode {
    #[allow(dead_code)]
    pub fn to_string(self) -> String {
        match self {
            Mode::Normal => String::from("NORMAL"),
            Mode::Insert => String::from("INSERT"),
            Mode::Visual => String::from("VISUAL"),
            Mode::Action => String::from("ACTION"),
        }
    }
}

pub struct InputController {
    keyboard: Box<dyn Keyboard>,
    view_id: String,
    normal_mode: ModeActions,
    insert_mode: ModeActions,
    visual_mode: ModeActions,
    action_mode: ModeActions,
    mode: Mode,
    front_event_writer: ClientToClientWriter,
}

impl InputController {
    pub fn new(
        keyboard: Box<dyn Keyboard>,
        client_to_client_writer: ClientToClientWriter,
        config: &Config,
    ) -> Self {
        Self {
            keyboard,
            view_id: String::new(),
            normal_mode: ModeActions::setup(Mode::Normal, &config.normal_mode),
            insert_mode: ModeActions::setup(Mode::Insert, &config.insert_mode),
            visual_mode: ModeActions::setup(Mode::Visual, &config.visual_mode),
            action_mode: ModeActions::setup(Mode::Action, &config.action_mode),
            mode: Mode::Normal,
            front_event_writer: client_to_client_writer,
        }
    }

    pub fn open_file(&mut self, core: &dyn Peer, file_path: &str) -> Result<(), Error> {
        let view_id = core
            .send_rpc_request("new_view", &json!({ "file_path": file_path }))
            .expect("failed to create the new view");

        self.view_id = view_id.as_str().unwrap().to_string();

        self.front_event_writer.send_rpc_notification(
            "set_path_for_view",
            &json!({
                "view_id": self.view_id,
                "path": file_path,
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
                let mut action = match self.mode {
                    Mode::Normal => self.normal_mode.get_action_from_keystroke(key),
                    Mode::Insert => self.insert_mode.get_action_from_keystroke(key),
                    Mode::Visual => self.visual_mode.get_action_from_keystroke(key),
                    Mode::Action => self.action_mode.get_action_from_keystroke(key),
                };

                if action.is_none() && self.mode == Mode::Insert {
                    action = Some(Action::InsertKeyStroke(key));
                } else if action.is_none() {
                    continue;
                }

                let res =
                    action
                        .unwrap()
                        .execute(&self.view_id, core, &mut self.front_event_writer);

                match res {
                    Response::Continue => continue,
                    Response::Stop => break,
                    Response::SwitchToInsertMode => self.mode = Mode::Insert,
                    Response::SwitchToNormalMode => self.mode = Mode::Normal,
                    Response::SwitchToVisualMode => self.mode = Mode::Visual,
                    Response::SwitchToActionMode => self.mode = Mode::Action,
                }

                core.send_rpc_notification(
                    "edit",
                    &json!({ "method": "collapse_selections", "view_id": self.view_id}),
                );

                self.front_event_writer.send_rpc_notification(
                    "update_status_item",
                    &json!({
                        "key": "change-mode",
                        "value": self.mode.to_string(),
                    }),
                );
            }
        }

        self.front_event_writer
            .send_rpc_notification("command", &json!({"method": "exit"}));

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
            [visual_mode]
            move_down  = "<key_up>"
         "#,
        )
        .unwrap();

        assert_eq!(
            String::from("<key_up>"),
            config.visual_mode[&String::from("move_down")]
        );
    }
}
