mod actions;
mod nouns;
mod verbs;

use std::collections::HashMap;

use self::actions::{Action, Actions};
use self::nouns::Nouns;
use self::verbs::Verbs;
use crate::devices::keyboard::KeyStroke;
use crate::input_controller::rpc::*;
use crate::input_controller::Response;

use xi_rpc::Peer;

#[derive(Default, Debug, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub actions: HashMap<String, String>,
    #[serde(default)]
    pub verbs: HashMap<String, String>,
    #[serde(default)]
    pub nouns: HashMap<String, String>,
}

#[derive(Default)]
pub struct NormalMode {
    #[allow(dead_code)]
    verbs: Verbs,
    #[allow(dead_code)]
    nouns: Nouns,
    actions: Actions,
}

impl From<&Config> for NormalMode {
    fn from(config_map: &Config) -> Self {
        Self {
            actions: Actions::from(&config_map.actions),
            nouns: Nouns::from(&config_map.nouns),
            verbs: Verbs::from(&config_map.verbs),
        }
    }
}

impl NormalMode {
    pub fn handle_keystroke(&self, key: KeyStroke, view_id: &str, core: &dyn Peer) -> Response {
        let action = self.actions.get(key);
        if let Some(action) = action {
            return match action {
                Action::SwitchToInsertMode => Response::SwitchToInsertMode,
                Action::SwitchToVisualMode => Response::SwitchToVisualMode,
                Action::Exit => exit(view_id, core),
                Action::MoveUp => move_up(view_id, core),
                Action::MoveDown => move_down(view_id, core),
                Action::MoveLeft => move_left(view_id, core),
                Action::MoveRight => move_right(view_id, core),
                Action::PageUp => page_up(view_id, core),
                Action::PageDown => page_down(view_id, core),
                Action::Paste => paste(view_id, core),
                Action::InsertLineBellow => insert_line_bellow(view_id, core),
                Action::InsertLineAbove => insert_line_above(view_id, core),
            };
        }

        Response::Continue
    }
}

fn insert_line_bellow(view_id: &str, core: &dyn Peer) -> Response {
    move_down(view_id, core);
    insert_newline(view_id, core);
    move_up(view_id, core);

    Response::SwitchToInsertMode
}
fn insert_line_above(view_id: &str, core: &dyn Peer) -> Response {
    insert_newline(view_id, core);
    move_up(view_id, core);

    Response::SwitchToInsertMode
}
