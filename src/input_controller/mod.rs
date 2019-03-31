mod insert_mode;
mod normal_mode;
mod visual_mode;

mod rpc;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use self::insert_mode::InsertMode;
use self::normal_mode::NormalMode;
use self::visual_mode::VisualMode;
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
    normal_mode: HashMap<String, String>,
    #[serde(default)]
    insert_mode: HashMap<String, String>,
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
}

impl InputController {
    pub fn new(terminal: Terminal, keyboard: Keyboard) -> Self {
        Self {
            terminal,
            keyboard,
            view_id: String::new(),
            normal_mode: NormalMode::default(),
            insert_mode: InsertMode::default(),
            visual_mode: VisualMode::default(),
            mode: Mode::Normal,
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
        //let res = core.send_rpc_request(
        //"plugin",
        //&json!({
        //"command": "plugin_rpc",
        //"view_id": self.view_id,
        //"receiver": "",
        //"rpc": {
        //"method": "add_status_item",
        //"rpc_type": "request",
        //"params": {
        //"alignment": "left",
        //"key": "change-mode",
        //"value": self.mode.to_string(),
        //}
        //}
        //}),
        //);

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
                    Response::Continue => {}
                    Response::Stop => break,
                    Response::SwitchToInsertMode => self.mode = Mode::Insert,
                    Response::SwitchToNormalMode => self.mode = Mode::Normal,
                    Response::SwitchToVisualMode => self.mode = Mode::Visual,
                }
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
