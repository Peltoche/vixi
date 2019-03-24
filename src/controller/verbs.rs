use crate::controller::Noun;

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
    info!("delete: {:?}", noun);
}
