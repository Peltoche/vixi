use crate::devices::keyboard::KeyStroke;
use crate::input_controller::{Response, PAST_BUFFER};

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

pub fn exit_selection_mode(view_id: &str, core: &dyn Peer) -> Response {
    core.send_rpc_notification(
        "edit",
        &json!({ "method": "collapse_selections", "view_id": view_id}),
    );

    Response::SwitchToNormalMode
}

pub fn delete_backward(view_id: &str, core: &dyn Peer) -> Response {
    core.send_rpc_notification(
        "edit",
        &json!({ "method": "delete_backward", "view_id": view_id}),
    );
    Response::Continue
}

pub fn delete_forward(view_id: &str, core: &dyn Peer) -> Response {
    core.send_rpc_notification(
        "edit",
        &json!({ "method": "delete_forward", "view_id": view_id}),
    );
    Response::Continue
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

pub fn move_up_and_select(view_id: &str, core: &dyn Peer) -> Response {
    core.send_rpc_notification(
        "edit",
        &json!({ "method": "move_up_and_modify_selection", "view_id": view_id}),
    );
    Response::Continue
}

pub fn move_down_and_select(view_id: &str, core: &dyn Peer) -> Response {
    core.send_rpc_notification(
        "edit",
        &json!({ "method": "move_down_and_modify_selection", "view_id": view_id}),
    );
    Response::Continue
}

pub fn move_left_and_select(view_id: &str, core: &dyn Peer) -> Response {
    core.send_rpc_notification(
        "edit",
        &json!({ "method": "move_left_and_modify_selection", "view_id": view_id}),
    );
    Response::Continue
}

pub fn move_right_and_select(view_id: &str, core: &dyn Peer) -> Response {
    core.send_rpc_notification(
        "edit",
        &json!({ "method": "move_right_and_modify_selection", "view_id": view_id}),
    );
    Response::Continue
}

pub fn insert_newline(view_id: &str, core: &dyn Peer) -> Response {
    core.send_rpc_notification(
        "edit",
        &json!({ "method": "insert_newline", "view_id": view_id}),
    );
    Response::Continue
}

pub fn copy_selection(view_id: &str, core: &dyn Peer) -> Response {
    let res = core.send_rpc_request("edit", &json!({ "method": "copy", "view_id": view_id}));
    if let Ok(paste_buffer) = res {
        let mut buffer = PAST_BUFFER.lock().unwrap();
        *buffer = Some(String::from(paste_buffer.as_str().unwrap()));
    } else {
        error!("failed to copy selection: {:?}", res.unwrap_err());
    }

    // Remove the selection
    core.send_rpc_notification(
        "edit",
        &json!({ "method": "collapse_selections", "view_id": view_id}),
    );

    Response::SwitchToNormalMode
}

pub fn cute_selection(view_id: &str, core: &dyn Peer) -> Response {
    let cut_res = core.send_rpc_request("edit", &json!({ "method": "cut", "view_id": view_id}));
    if cut_res.is_err() {
        error!("failed to cut the selection: {:?}", cut_res);
    }

    let mut buffer = PAST_BUFFER.lock().unwrap();
    *buffer = Some(String::from(cut_res.unwrap().as_str().unwrap()));

    // Remove the selection
    core.send_rpc_notification(
        "edit",
        &json!({ "method": "collapse_selections", "view_id": view_id}),
    );

    Response::SwitchToNormalMode
}

pub fn delete_selection_and_paste(view_id: &str, core: &dyn Peer) -> Response {
    let cut_res = core.send_rpc_request("edit", &json!({ "method": "cut", "view_id": view_id}));
    if cut_res.is_err() {
        error!("failed to cut the selection: {:?}", cut_res);
    }

    paste(view_id, core);

    let mut buffer = PAST_BUFFER.lock().unwrap();
    *buffer = Some(String::from(cut_res.unwrap().as_str().unwrap()));

    // Remove the selection
    core.send_rpc_notification(
        "edit",
        &json!({ "method": "collapse_selections", "view_id": view_id}),
    );

    Response::SwitchToNormalMode
}

pub fn paste(view_id: &str, core: &dyn Peer) -> Response {
    let buffer = PAST_BUFFER.lock().unwrap();
    if let Some(ref s) = *buffer {
        core.send_rpc_notification(
            "edit",
            &json!({
                "method": "paste",
                "view_id": view_id,
                "params": {
                    "chars": s,
                }
            }),
        );
    }
    Response::Continue
}
