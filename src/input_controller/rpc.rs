use crate::devices::keyboard::KeyStroke;
use crate::input_controller::Response;

use xi_rpc::Peer;

pub fn insert_char(view_id: &str, key: KeyStroke, core: &dyn Peer) -> Response {
    let output = match key {
        KeyStroke::Char(c) => c.to_string(),
        _ => String::from("<?>"),
    };

    core.send_rpc_notification(
        "edit",
        &json!({
            "method": "insert",
            "view_id": view_id,
            "params": {
                "chars": output,
            }
        }),
    );
    Response::Continue
}

pub fn exit(_view_id: &str, _core: &dyn Peer) -> Response {
    Response::Stop
}

pub fn move_up(view_id: &str, core: &dyn Peer) -> Response {
    core.send_rpc_notification("edit", &json!({ "method": "move_up", "view_id": view_id}));
    Response::Continue
}

pub fn move_down(view_id: &str, core: &dyn Peer) -> Response {
    core.send_rpc_notification("edit", &json!({ "method": "move_down", "view_id": view_id}));
    Response::Continue
}

pub fn move_left(view_id: &str, core: &dyn Peer) -> Response {
    core.send_rpc_notification("edit", &json!({ "method": "move_left", "view_id": view_id}));
    Response::Continue
}

pub fn move_right(view_id: &str, core: &dyn Peer) -> Response {
    core.send_rpc_notification(
        "edit",
        &json!({ "method": "move_right", "view_id": view_id}),
    );
    Response::Continue
}

pub fn page_up(view_id: &str, core: &dyn Peer) -> Response {
    core.send_rpc_notification(
        "edit",
        &json!({ "method": "scroll_page_up", "view_id": view_id}),
    );
    Response::Continue
}

pub fn page_down(view_id: &str, core: &dyn Peer) -> Response {
    core.send_rpc_notification(
        "edit",
        &json!({ "method": "scroll_page_down", "view_id": view_id}),
    );
    Response::Continue
}
