use std::io::Write;

use ncurses::*;
use serde_json::value::Value;
use xi_rpc::RpcCtx;

#[derive(Default)]
pub struct EventHandler {
    buffer: Vec<String>,
}

impl<W: Write> xi_rpc::Handler<W> for EventHandler {
    fn handle_notification(&mut self, _ctx: RpcCtx<W>, method: &str, params: &Value) {
        match method {
            "available_languages" => debug!("{}", method),
            "available_themes" => debug!("{}", method),
            "available_plugins" => debug!("{}", method),
            "config_changed" => debug!("{}", method),
            "scroll_to" => self.handle_cursor_move(params),
            "language_changed" => debug!("{}", method),
            "update" => self.handle_update(params),
            _ => debug!("unhandled notif {} -> {:#?}", method, params),
        };

        refresh();
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

impl EventHandler {
    fn handle_cursor_move(&mut self, body: &Value) {
        #[derive(Deserialize, Debug)]
        struct ScrollInfo {
            col: i32,
            line: i32,
        }

        let event: ScrollInfo = serde_json::from_value(body.clone()).unwrap();
        mv(event.line, event.col);
    }

    fn handle_update(&mut self, body: &Value) {
        #[derive(Deserialize, Debug)]
        struct Annotation {
            #[serde(rename = "type")]
            annotation_type: String,
            n: usize,
            payloads: Option<()>,
            ranges: Vec<Vec<i32>>,
        }

        #[derive(Deserialize, Debug)]
        struct Line {
            cursor: Option<Vec<i32>>,
            ln: i32,
            styles: Vec<String>,
            text: String,
        }

        #[derive(Deserialize, Debug)]
        struct Operation {
            #[serde(rename = "op")]
            kind: String,
            n: usize,
            lines: Option<Vec<Line>>,
        }

        #[derive(Deserialize, Debug)]
        struct Update {
            annotations: Vec<Annotation>,
            #[serde(rename = "ops")]
            operations: Vec<Operation>,
        }

        #[derive(Deserialize, Debug)]
        struct UpdateEvent {
            view_id: String,
            update: Update,
        }

        let event: UpdateEvent = serde_json::from_value(body.clone()).unwrap();

        let mut new_buffer: Vec<String> = Vec::new();
        let mut old_ix: usize = 0;

        for operation in event.update.operations {
            match operation.kind.as_str() {
                "copy" => {
                    for i in 0..operation.n {
                        new_buffer.push(self.buffer[old_ix + i].to_owned());
                    }

                    old_ix += operation.n;
                }
                "skip" => old_ix += operation.n,
                "invalidate" => {
                    //for _ in 0..operation.n {
                    //let line = String::from("????INVALID LINE???????").to_owned();
                    //new_buffer.push(line);
                    //}
                }
                "ins" => {
                    for line in operation.lines.unwrap() {
                        //let tmp = line.text.clone::<'a>();
                        let tmp = line.text.to_owned();

                        //let boxed_line: Box<&'a str> = Box::new(tmp);
                        //let line: &'a &str = &boxed_line;
                        //new_buffer.push(tmp);
                        new_buffer.push(tmp);
                    }
                }
                _ => warn!("unhandled update 2: {:?}", operation),
            }
        }

        self.switch_buffers(new_buffer);
    }

    fn switch_buffers(&mut self, new_buffer: Vec<String>) {
        clear();
        for line in new_buffer.iter() {
            addstr(line.as_str());
        }

        self.buffer = new_buffer;
    }
}
