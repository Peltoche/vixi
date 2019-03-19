use std::io::prelude::*;

use serde_json::value::Value;
use xi_rpc::RpcCtx;

pub struct Handler {}

impl Handler {
    pub fn new() -> Self {
        Self {}
    }
}

impl<W: Write> xi_rpc::Handler<W> for Handler {
    fn handle_notification(&mut self, _ctx: RpcCtx<W>, method: &str, params: &Value) {
        match method {
            "available_languages" => debug!("{}", method),
            "available_themes" => debug!("{}", method),
            _ => debug!("unhandled notif {} -> {:#?}", method, params),
        };
    }

    fn handle_request(
        &mut self,
        _ctx: RpcCtx<W>,
        method: &str,
        params: &Value,
    ) -> Result<Value, Value> {
        debug!("[request] {} -> {:#?}", method, params);
        Ok(json!({}))
    }
}
