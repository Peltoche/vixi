mod actions;

use std::collections::HashMap;

use self::actions::{Action, Actions};
use crate::devices::keyboard::KeyStroke;
use crate::input_controller::rpc::*;
use crate::input_controller::Response;

use xi_rpc::Peer;

#[derive(Debug, Default, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub actions: HashMap<String, String>,
}

#[derive(Default)]
pub struct InsertMode {
    actions: Actions,
}

impl InsertMode {
    pub fn handle_keystroke(&self, key: KeyStroke, view_id: &str, core: &dyn Peer) -> Response {
        let action = self.actions.get(key);
        if let Some(action) = action {
            return match action {
                Action::ExitInsertMode => Response::SwitchToNormalMode,
                Action::MoveUp => move_up(view_id, core),
                Action::MoveDown => move_down(view_id, core),
                Action::MoveLeft => move_left(view_id, core),
                Action::MoveRight => move_right(view_id, core),
                Action::PageUp => page_up(view_id, core),
                Action::PageDown => page_down(view_id, core),
                Action::DeleteBackward => delete_backward(view_id, core),
                Action::DeleteForward => delete_forward(view_id, core),
            };
        }

        insert_char(view_id, key, core);
        Response::Continue
    }
}

impl From<&Config> for InsertMode {
    fn from(config: &Config) -> Self {
        Self {
            actions: Actions::from(&config.actions),
        }
    }
}
