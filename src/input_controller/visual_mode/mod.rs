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
pub struct VisualMode {
    actions: Actions,
}

impl VisualMode {
    pub fn handle_keystroke(&self, key: KeyStroke, view_id: &str, core: &dyn Peer) -> Response {
        let action = self.actions.get(key);
        if let Some(action) = action {
            return match action {
                Action::ExitSelectionMode => exit_selection_mode(view_id, core),
                Action::MoveUp => move_up_and_select(view_id, core),
                Action::MoveDown => move_down_and_select(view_id, core),
                Action::MoveLeft => move_left_and_select(view_id, core),
                Action::MoveRight => move_right_and_select(view_id, core),
                Action::YankSelection => copy_selection(view_id, core),
                Action::DeleteSelection => cute_selection(view_id, core),
                Action::DeleteSelectionAndPaste => delete_selection_and_paste(view_id, core),
                // The current implementation fail if the buffer is not
                // already available
                //
                //Action::PageUp => page_up_and_select(view_id, core),
                //Action::PageDown => page_down_and_select(view_id, core),
            };
        }

        Response::Continue
    }
}

impl From<&Config> for VisualMode {
    fn from(config: &Config) -> Self {
        Self {
            actions: Actions::from(&config.actions),
        }
    }
}
