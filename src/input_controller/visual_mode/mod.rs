mod actions;

use self::actions::{Action, Actions};
use crate::devices::keyboard::KeyStroke;
use crate::input_controller::rpc::*;
use crate::input_controller::Response;

use xi_rpc::Peer;

#[derive(Default)]
#[allow(dead_code)]
pub struct Config {
    actions: actions::Config,
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
                Action::SwitchToNormalMode => Response::SwitchToNormalMode,
                Action::MoveUp => move_up_and_select(view_id, core),
                Action::MoveDown => move_down_and_select(view_id, core),
                Action::MoveLeft => move_left_and_select(view_id, core),
                Action::MoveRight => move_right_and_select(view_id, core),
                Action::Yank => copy_selection(view_id, core),
                Action::Paste => paste(view_id, core),
                // The current Core implementation doesn't fail of the buffer is not
                // already available
                //
                //Action::PageUp => page_up_and_select(view_id, core),
                //Action::PageDown => page_down_and_select(view_id, core),
            };
        }

        Response::Continue
    }
}
