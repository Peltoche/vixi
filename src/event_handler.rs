use ncurses::*;
use serde_json::Value;
use xi_rpc::{RemoteError, RpcCall, RpcCtx};

#[derive(Default)]
pub struct EventHandler {
    screen_start: i32,
    cursor_x: i32,
    cursor_y: i32,
    buffer: Vec<String>,
}

impl xi_rpc::Handler for EventHandler {
    type Notification = RpcCall;
    type Request = RpcCall;

    fn handle_notification(&mut self, ctx: &RpcCtx, rpc: Self::Notification) {
        match rpc.method.as_str() {
            "available_languages" => debug!("{}", &rpc.method),
            "available_themes" => debug!("{}", &rpc.method),
            "available_plugins" => debug!("{} -> {}", &rpc.method, &rpc.params),
            "config_changed" => debug!("{}", &rpc.method),
            "scroll_to" => self.handle_cursor_move(&ctx, &rpc.params),
            "language_changed" => debug!("{}: -> {}", &rpc.method, &rpc.params),
            "def_style" => self.handle_style_change(&rpc.params),
            "update" => self.handle_update(&rpc.params),
            _ => debug!("unhandled notif {} -> {}", &rpc.method, &rpc.params),
        };

        refresh();
    }

    fn handle_request(&mut self, _ctx: &RpcCtx, rpc: Self::Request) -> Result<Value, RemoteError> {
        info!("[request] {} -> {:#?}", rpc.method, rpc.params);
        Ok(json!({}))
    }
}

impl EventHandler {
    fn handle_style_change(&mut self, body: &Value) {
        #[derive(Deserialize, Debug)]
        struct StyleInfo {
            id: String,
            fg_color: i32,
        }

        let event: StyleInfo = serde_json::from_value(body.clone()).unwrap();

        info!("foobar");
        info!(
            "color: {:?} - {:?}",
            event.fg_color.to_le_bytes(),
            event.fg_color.to_ne_bytes()
        );

        //Colour::RGB()
    }

    fn handle_cursor_move(&mut self, ctx: &RpcCtx, body: &Value) {
        #[derive(Deserialize, Debug)]
        struct ScrollInfo {
            view_id: String,
            col: i32,
            line: i32,
        }

        let event: ScrollInfo = serde_json::from_value(body.clone()).unwrap();

        let size_y = getmaxy(stdscr());
        let mut cursor_y = event.line - self.screen_start;

        let mut scroll: bool = false;
        if cursor_y == size_y {
            self.screen_start += 1;
            scroll = true;
            cursor_y -= 1
        } else if cursor_y <= -1 {
            self.screen_start -= 1;
            scroll = true;
            cursor_y += 1
        }

        self.cursor_x = event.col as i32;
        self.cursor_y = cursor_y;

        if scroll {
            // In case of scroll it need to redraw the screen and after it
            // the cursor is automatically reset at (self.cursor_y/self.cursor_x).
            ctx.get_peer().send_rpc_notification(
                "edit",
                &json!({
                    "method": "scroll",
                    "view_id": event.view_id,
                    "params": [self.screen_start , self.screen_start + size_y]
                }),
            );

            self.redraw_view();
        } else {
            // No scroll needed so it move the cursor without any redraw.
            mv(self.cursor_y, self.cursor_x);
        }
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
                    for _ in 0..operation.n {
                        let line = String::from("????INVALID LINE???????\n").to_owned();
                        new_buffer.push(line);
                    }
                }
                "ins" => {
                    for line in operation.lines.unwrap() {
                        let tmp = line.text.to_owned();
                        new_buffer.push(tmp);
                    }
                }
                _ => warn!("unhandled update 2: {:?}", operation),
            }
        }

        self.buffer = new_buffer;
        self.redraw_view();
    }

    fn redraw_view(&mut self) {
        clear();

        let size_y = getmaxy(stdscr()) - 1;
        let buffer_size = self.buffer.len() as i32;

        let nb_lines_to_draw = if size_y > buffer_size - self.screen_start {
            buffer_size - self.screen_start
        } else {
            self.screen_start + size_y
        };

        let mut output = String::new();
        self.buffer
            .iter()
            .skip(self.screen_start as usize)
            .take(nb_lines_to_draw as usize)
            .for_each(|x| output.push_str(&x));

        addstr(output.as_str());
        mv(self.cursor_y, self.cursor_x);
    }
}
