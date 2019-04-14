mod commands;
pub mod config;
pub mod keyboard;
mod modes;

use std::sync::{Arc, Mutex};

use self::commands::{Command, Response};
use self::config::Config;
use self::keyboard::{KeyStroke, Keyboard};
use self::modes::ModeCommands;
use crate::core::ClientToClientWriter;

use failure::Error;
use xi_rpc::Peer;

lazy_static! {
    static ref PASTE_BUFFER: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));
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
    normal_mode: ModeCommands,
    insert_mode: ModeCommands,
    visual_mode: ModeCommands,
    action_mode: ModeCommands,
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
            normal_mode: ModeCommands::setup(Mode::Normal, &config.normal_mode),
            insert_mode: ModeCommands::setup(Mode::Insert, &config.insert_mode),
            visual_mode: ModeCommands::setup(Mode::Visual, &config.visual_mode),
            action_mode: ModeCommands::setup(Mode::Action, &config.action_mode),
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
                    Mode::Normal => self.normal_mode.get_command_from_keystroke(key),
                    Mode::Insert => self.insert_mode.get_command_from_keystroke(key),
                    Mode::Visual => self.visual_mode.get_command_from_keystroke(key),
                    Mode::Action => self.action_mode.get_command_from_keystroke(key),
                };

                if action.is_none() && self.mode == Mode::Insert {
                    action = Some(Command::InsertKeyStroke(key));
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
