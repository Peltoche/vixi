mod actions;

use std::collections::HashMap;

use self::actions::{Action, Actions};
use super::keyboard::KeyStroke;
use crate::input_controller::rpc::*;
use crate::input_controller::Response;

use xi_rpc::Peer;

#[derive(Debug, Default, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub actions: HashMap<String, String>,
}

#[derive(Default)]
pub struct ActionMode {
    actions: Actions,
}

impl ActionMode {
    pub fn handle_keystroke(&self, key: KeyStroke, view_id: &str, core: &dyn Peer) -> Response {
        let action = self.actions.get(key);
        if let Some(action) = action {
            return match action {
                Action::Quite => quite(view_id, core),
                Action::WriteToFile => write_to_file(view_id, core),
                Action::ExitActionMode => Response::SwitchToNormalMode,
            };
        }

        Response::SwitchToNormalMode
    }
}

impl From<&Config> for ActionMode {
    fn from(config: &Config) -> Self {
        Self {
            actions: Actions::from(&config.actions),
        }
    }
}
