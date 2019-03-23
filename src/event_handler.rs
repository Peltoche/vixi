use ncurses::*;
use serde_json::Value;
use xi_rpc::{RemoteError, RpcCall, RpcCtx};

#[derive(Default, Clone)]
struct Line {
    raw: String,
    styles: Vec<usize>,
}

#[derive(Default)]
pub struct EventHandler {
    screen_start: i32,
    cursor_x: i32,
    cursor_y: i32,
    buffer: Vec<Line>,
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
            id: i16,
            fg_color: u32,
            #[serde(default)]
            bg_color: u32,
        }

        let event: StyleInfo = serde_json::from_value(body.clone()).unwrap();

        let fg_color_id = 200 + event.id;
        let bg_color_id = 100 + event.id;

        //
        // fg
        //
        let fg_rgba: [u8; 4] = event.fg_color.to_le_bytes();
        let fg_r = i16::from(fg_rgba[0]) * 4;
        let fg_g = i16::from(fg_rgba[1]) * 4;
        let fg_b = i16::from(fg_rgba[2]) * 4;
        init_color(bg_color_id, fg_r, fg_g, fg_b);

        //
        // bg
        //
        let bg_rgba: [u8; 4] = event.bg_color.to_le_bytes();
        let bg_r = i16::from(bg_rgba[0]) * 4;
        let bg_g = i16::from(bg_rgba[1]) * 4;
        let bg_b = i16::from(bg_rgba[2]) * 4;
        init_color(bg_color_id, bg_r, bg_g, bg_b);

        //
        // pair
        //
        let pair_id = event.id;

        init_pair(pair_id, fg_color_id, bg_color_id);

        bkgd(' ' as chtype | COLOR_PAIR(pair_id) as chtype);
        let mut w: i32 = 0;
        let mut h: i32 = 0;
        getmaxyx(stdscr(), &mut h, &mut w);
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
        struct LineDescription {
            cursor: Option<Vec<i32>>,
            ln: i32,
            styles: Vec<usize>,
            text: String,
        }

        #[derive(Deserialize, Debug)]
        struct Operation {
            #[serde(rename = "op")]
            kind: String,
            n: usize,
            lines: Option<Vec<LineDescription>>,
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
        let mut new_buffer = Vec::new();
        let mut old_ix: usize = 0;

        for operation in event.update.operations {
            match operation.kind.as_str() {
                "copy" => {
                    for i in 0..operation.n {
                        new_buffer.push(self.buffer[old_ix + i].clone());
                    }

                    old_ix += operation.n;
                }
                "skip" => old_ix += operation.n,
                "invalidate" => {
                    for _ in 0..operation.n {
                        new_buffer.push(Line {
                            raw: String::from("????INVALID LINE???????\n").to_owned(),
                            styles: Vec::new(),
                        });
                    }
                }
                "ins" => {
                    for line in operation.lines.unwrap() {
                        new_buffer.push(Line {
                            raw: line.text.to_owned(),
                            styles: line.styles,
                        });
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

        let size_y = getmaxy(stdscr());
        let buffer_size = self.buffer.len() as i32;

        let nb_lines_to_draw = if size_y > buffer_size - self.screen_start {
            buffer_size - self.screen_start
        } else {
            size_y
        };

        self.buffer
            .iter()
            .skip(self.screen_start as usize)
            .take(nb_lines_to_draw as usize)
            .for_each(|line| self.print_stylized_line(&line));

        mv(self.cursor_y, self.cursor_x);
    }

    fn print_stylized_line(&self, line: &Line) {
        let mut idx: usize = 0;
        let mut memory: Option<(usize, usize, usize)> = None;

        let line_len = line.raw.len();
        let mut style_iter = line.styles.iter();
        loop {
            let (style_start, style_length, style_id) = if let Some((start, id, length)) = memory {
                memory = None;
                (idx + start, id, length)
            } else if let Some(style_start) = style_iter.next() {
                (
                    idx + *style_start,
                    *style_iter.next().unwrap(),
                    *style_iter.next().unwrap(),
                )
            } else {
                (usize::max_value(), usize::max_value(), usize::max_value())
            };

            if style_start == usize::max_value() {
                break;
            }

            if style_start == idx {
                let attr = COLOR_PAIR(style_id as i16);
                attron(attr);
                addstr(unsafe {
                    line.raw
                        .get_unchecked(style_start..(style_start + style_length))
                });
                attroff(attr);

                idx += style_length;
            } else {
                addstr(unsafe { line.raw.get_unchecked(idx..style_start) });

                memory = Some((style_start, style_id, style_length));
                idx = style_start;
            }
        }

        addstr(unsafe { line.raw.get_unchecked(idx..line_len) });
    }
}
