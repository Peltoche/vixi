use crate::input_controller::Noun;

use xi_rpc::Peer;

#[derive(Debug)]
pub enum Verb {
    Delete,
}

pub fn run(verb: &Verb, noun: &Noun, view_id: &str, core: &dyn Peer) {
    match verb {
        Verb::Delete => delete(noun, view_id, core),
    }
}

fn delete(noun: &Noun, view_id: &str, core: &dyn Peer) {
    match noun {
        Noun::Line => {} //_ => {
                         //warn!("delete doesn't handle the {:?}", noun);
                         //return;
                         //}
    }

    info!("delete line");
    core.send_rpc_notification(
        "edit",
        &json!({ "method": "move_right_and_modify_selection", "view_id": view_id}),
    );
}
