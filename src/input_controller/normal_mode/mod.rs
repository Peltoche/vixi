mod nouns;
mod verbs;

use self::nouns::Nouns;
use self::verbs::Verbs;
use crate::devices::keyboard::KeyStroke;
use crate::input_controller::actions::Action;

use failure::Error;
use xi_rpc::Peer;

#[derive(Default)]
pub struct Config {
    verbs: verbs::Config,
    nouns: nouns::Config,
}

pub enum Response {
    Continue,
    Stop,
}

#[derive(Default)]
pub struct NormalMode {
    verbs: Verbs,
    nouns: Nouns,
}

impl NormalMode {
    pub fn from_config(config_map: &Config) -> Result<Self, Error> {
        Ok(NormalMode {
            verbs: Verbs::from_config(&config_map.verbs)?,
            nouns: Nouns::from_config(&config_map.nouns)?,
        })
    }

    pub fn handle_keystroke(&self, key: KeyStroke, view_id: &str, core: &dyn Peer) -> Response {
        Response::Continue
    }

    pub fn handle_action(&self, action: Action, view_id: &str, core: &dyn Peer) -> Response {
        match action {
            Action::Exit => exit(view_id, core),
            Action::MoveUp => move_up(view_id, core),
            Action::MoveDown => move_down(view_id, core),
            Action::MoveLeft => move_left(view_id, core),
            Action::MoveRight => move_right(view_id, core),
            Action::PageUp => page_up(view_id, core),
            Action::PageDown => page_down(view_id, core),
        }
    }
}

//fn delete(noun: &Noun, view_id: &str, core: &dyn Peer) {
//match noun {
//Noun::Line => {} //_ => {
////warn!("delete doesn't handle the {:?}", noun);
////return;
////}
//}

//info!("delete line");
//core.send_rpc_notification(
//"edit",
//&json!({ "method": "move_right_and_modify_selection", "view_id": view_id}),
//);
//}
fn exit(_view_id: &str, _core: &dyn Peer) -> Response {
    Response::Stop
}

fn move_up(view_id: &str, core: &dyn Peer) -> Response {
    core.send_rpc_notification("edit", &json!({ "method": "move_up", "view_id": view_id}));
    Response::Continue
}

fn move_down(view_id: &str, core: &dyn Peer) -> Response {
    core.send_rpc_notification("edit", &json!({ "method": "move_down", "view_id": view_id}));
    Response::Continue
}

fn move_left(view_id: &str, core: &dyn Peer) -> Response {
    core.send_rpc_notification("edit", &json!({ "method": "move_left", "view_id": view_id}));
    Response::Continue
}

fn move_right(view_id: &str, core: &dyn Peer) -> Response {
    core.send_rpc_notification(
        "edit",
        &json!({ "method": "move_right", "view_id": view_id}),
    );
    Response::Continue
}

fn page_up(view_id: &str, core: &dyn Peer) -> Response {
    core.send_rpc_notification(
        "edit",
        &json!({ "method": "scroll_page_up", "view_id": view_id}),
    );
    Response::Continue
}

fn page_down(view_id: &str, core: &dyn Peer) -> Response {
    core.send_rpc_notification(
        "edit",
        &json!({ "method": "scroll_page_down", "view_id": view_id}),
    );
    Response::Continue
}
