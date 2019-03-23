use ncurses::*;
use serde_json::Value;
use xi_rpc::{RemoteError, RpcCall, RpcCtx};

/// The style id is used to override the ncurses default colors and save the
/// style color. If this number is two hight, some color conflicts will appeares.
///
/// Check the `handle_style_change` method documentation for more informations.
const MAX_STYLE_ID: i16 = 50;

#[derive(Default, Clone)]
struct Line {
    raw: String,
    styles: Vec<usize>,
}

#[derive(Default)]
pub struct EventHandler {
    /// An index pointing to the Line rendered at the top of the screen.
    ///
    /// Changing its value make the screen scoll up/down.
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
            "update" => self.handle_content_update(&rpc.params),
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
    /// Handle the "def_style" event.
    ///
    /// This function need to create a new set of background/foreground and save
    /// it with the given id.
    ///
    /// The style saving process is done via the ncurse routines by overriding
    /// the existing colors presets via the `init_color` and `init_pair`
    /// functions. Thoses functionalities are only available for the
    /// terminals with the `truecolor` capability. In order to check if your
    /// terminal can handle it check that the output of `echo $COLORTERM` is
    /// equal to `truecolor` and the output of `echo $TERM` is equal to
    /// `xterm-256color`.
    ///
    /// As the `xterm-256color` feature is set, the terminal preset a set of
    /// 256 colors with the ids from 0 to 255. As the feature `truecolor` is set
    /// we can overrid those colors with some arbitrary RGB color. It will save
    /// the color_pairs (a set of background + foreground color) within the id
    /// range of [0..50], the foreground colors within the range [50..100] and
    /// the background colors whithin the range [100...150]. As each style
    /// correspond to a color pair, which is composed of a background and and
    /// foreground color it can save only 50 differents styles. After this number
    /// the colors_pairs saved will override the foreground colors and the
    /// foreground colors will override the background colors leading to some
    /// randome colors sets.
    fn handle_style_change(&mut self, body: &Value) {
        #[derive(Deserialize, Debug)]
        struct Event {
            id: i16,
            fg_color: u32,
            #[serde(default)]
            bg_color: u32,
        }

        let event: Event = serde_json::from_value(body.clone()).unwrap();

        if event.id > MAX_STYLE_ID {
            error!(
                "the new style id is greater than {}, this will load to some randome colors.",
                MAX_STYLE_ID
            );
        }

        // Name space the foreground and background colors.
        let fg_color_id = 50 + event.id;
        let bg_color_id = 100 + event.id;

        // Override the default colors with the `init_color` method. Once save
        // those colors will be accessible via the ids `fg_color_id` and
        // `bg_color_id`.
        //
        // The `init_color` method take a color range within [0..1000] but the
        // RGBA colors received by the event are within the range [0..256]. A
        // rough conversion is done by multiplying the event values by 4.

        // fg
        let fg_rgba: [u8; 4] = event.fg_color.to_le_bytes();
        let fg_r = i16::from(fg_rgba[0]) * 4;
        let fg_g = i16::from(fg_rgba[1]) * 4;
        let fg_b = i16::from(fg_rgba[2]) * 4;
        init_color(bg_color_id, fg_r, fg_g, fg_b);

        // bg
        let bg_rgba: [u8; 4] = event.bg_color.to_le_bytes();
        let bg_r = i16::from(bg_rgba[0]) * 4;
        let bg_g = i16::from(bg_rgba[1]) * 4;
        let bg_b = i16::from(bg_rgba[2]) * 4;
        init_color(bg_color_id, bg_r, bg_g, bg_b);

        // Save the new pair of background/foreground color with the `init_pair`
        // method. The pair_id must be the same id than the style id in order
        // to avoid translation during the rendering (cf: the
        // `print_stylized_line` method).

        // pair
        init_pair(event.id, fg_color_id, bg_color_id);
    }

    /// Handle the "scroll_to" event.
    ///
    /// It move the cursor into the given position. If the position is not
    /// within the screen, it will scroll all the view content by modifying
    /// the `self.screen_start` value.
    fn handle_cursor_move(&mut self, ctx: &RpcCtx, body: &Value) {
        #[derive(Deserialize, Debug)]
        struct Event {
            view_id: String,
            col: i32,
            line: i32,
        }

        let event: Event = serde_json::from_value(body.clone()).unwrap();

        // TODO: Avoid to check the term size at each event by saving it.
        // This will implicate to have some background process checking the
        // window size changes.
        let size_y = getmaxy(stdscr());
        let mut cursor_y = event.line - self.screen_start;

        let mut scroll: bool = false;
        if cursor_y == size_y {
            // The cursor is bellow the current screen view. Trigger a scroll.
            self.screen_start += 1;
            scroll = true;
            cursor_y -= 1
        } else if cursor_y <= -1 {
            // The cursor is abor the current screen view. Trigger a scroll.
            self.screen_start -= 1;
            scroll = true;
            cursor_y += 1
        }

        // Move the cursor at its new position.
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

    /// Handle the "update" event.
    ///
    /// It create a new buffer, apply all the event directives, swith this
    /// new buffer with the old one than trigger a redraw.
    ///
    /// The event is compose of differents `Operation`. Each one indicate how
    /// to fill the new buffer. The posible operations are:
    /// - "copy" -> Copy a part of the old buffer into the new one.
    /// - "skip" -> Keep a number of line empty.
    /// - "invalidate" -> Mark some lines as not available because the core
    ///     doesn't have given their content yet.
    /// - "ins" -> Insert some new content.
    fn handle_content_update(&mut self, body: &Value) {
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
        struct Event {
            view_id: String,
            update: Update,
        }

        let event: Event = serde_json::from_value(body.clone()).unwrap();
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

    /// Redraw the screen content.
    ///
    /// It take the Line corresponding to `this.buffer[this.screen_start]` and
    /// render it as the top line and fill the screen with all the following
    /// lines.
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

    /// Display the line content with the specified styles.
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
