use xi_rpc::Peer;

#[derive(Debug, Copy, Clone)]
pub enum Action {
    Exit,
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    PageUp,
    PageDown,
}

pub enum Response {
    Continue,
    Stop,
}

pub fn run(action: &Action, view_id: &str, core: &dyn Peer) -> Response {
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
