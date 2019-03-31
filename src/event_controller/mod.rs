use crate::devices::terminal::{RGBColor, RedrawBehavior, Terminal};

use serde_json::Value;
use xi_rpc::{RemoteError, RpcCall, RpcCtx};

#[derive(Default, Clone)]
pub struct Buffer {
    pub lines: Vec<Line>,
    pub nb_invalid_lines: usize,
}

impl Buffer {
    pub fn total_len(&self) -> usize {
        self.lines.len() + self.nb_invalid_lines
    }

    pub fn lines_availables_after(&self, start: usize) -> usize {
        self.lines.len() - start
    }
}

#[derive(Default, Clone)]
pub struct Cursor {
    pub y: u32,
    pub x: u32,
}

#[derive(Default, Clone)]
pub struct Line {
    pub raw: String,
    pub styles: Vec<i32>,
    /// The "real" line number.
    ///
    /// A line wrapped in two lines will keep the same `ln` value.
    pub ln: usize,
    /// Indicate if the line needs to be rendered during the next redraw.
    pub is_dirty: bool,
}

pub struct EventController {
    terminal: Terminal,
    /// An index pointing to the Line rendered at the top of the screen.
    ///
    /// Changing its value make the screen scoll up/down.
    screen_start: usize,
    screen_width: usize,

    /// Cursor positions into the editing screen.
    ///
    /// This position take into account the line_section. This means that when
    /// `cursor.x` is equal to 0, its real position is calculated dynamically
    /// base on the size of the line section.
    cursor: Cursor,

    /// Buffer containing all the lines informations.
    buffer: Buffer,
}

impl xi_rpc::Handler for EventController {
    type Notification = RpcCall;
    type Request = RpcCall;

    fn handle_notification(&mut self, ctx: &RpcCtx, rpc: Self::Notification) {
        match rpc.method.as_str() {
            "add_status_item" => self.handle_new_status_item(&rpc.params),
            "available_languages" => debug!("{}", &rpc.method),
            "available_themes" => debug!("{}", &rpc.method),
            "available_plugins" => debug!("{}", &rpc.method),
            "config_changed" => debug!("{}", &rpc.method),
            "def_style" => self.handle_style_change(&rpc.params),
            "language_changed" => debug!("{}", &rpc.method),
            "scroll_to" => self.handle_cursor_move(&ctx, &rpc.params),
            "update" => self.handle_content_update(&ctx, &rpc.params),
            "update_status_item" => self.update_status_item(&rpc.params),
            "plugin_started" => debug!("{}: -> {}", &rpc.method, &rpc.params),
            "theme_changed" => debug!("{}", &rpc.method),
            _ => debug!("unhandled notif {} -> {}", &rpc.method, &rpc.params),
        };

        self.terminal.redraw();
    }

    fn handle_request(&mut self, _ctx: &RpcCtx, rpc: Self::Request) -> Result<Value, RemoteError> {
        info!("[request] {} -> {:#?}", rpc.method, rpc.params);
        Ok(json!({}))
    }
}

impl EventController {
    pub fn new(terminal: Terminal) -> Self {
        Self {
            terminal,
            screen_start: 0,
            screen_width: 0,
            cursor: Cursor::default(),
            buffer: Buffer::default(),
        }
    }

    fn handle_new_status_item(&mut self, body: &Value) {
        info!("new status item: {}", body);
        #[derive(Deserialize, Debug)]
        struct Event {
            source: String,
            key: String,
            value: String,
            alignment: String,
        }

        info!("receive status item: {}", body);
        let event: Event = serde_json::from_value(body.clone()).unwrap();

        if let "change-mode" = event.key.as_str() {
            self.terminal.update_status_bar_mode(&event.value);
        }
    }

    fn update_status_item(&mut self, body: &Value) {
        info!("udpate status item: {}", body);
        #[derive(Deserialize, Debug)]
        struct Event {
            key: String,
            value: String,
        }

        let event: Event = serde_json::from_value(body.clone()).unwrap();

        if let "change-mode" = event.key.as_str() {
            self.terminal.update_status_bar_mode(&event.value);
        }
    }

    /// Handle the "def_style" event.
    ///
    /// This function need to create a new set of background/foreground and save
    /// it with the given id.
    fn handle_style_change(&mut self, body: &Value) {
        info!("style change: {}", body);
        #[derive(Deserialize, Debug)]
        struct Event {
            id: u32,
            #[serde(default)]
            italic: bool,
            fg_color: u32,
            #[serde(default)]
            bg_color: u32,
        }

        let event: Event = serde_json::from_value(body.clone()).unwrap();

        // Override the default colors with the `init_color` method. Once save
        // those colors will be accessible via the ids `fg_style_id` and
        // `bg_style_id`.
        //

        // fg
        let fg_rgba: [u8; 4] = event.fg_color.to_le_bytes();
        let fg_color = RGBColor {
            r: fg_rgba[0],
            g: fg_rgba[1],
            b: fg_rgba[2],
        };

        let bg_rgba: [u8; 4] = event.bg_color.to_le_bytes();
        let bg_color = RGBColor {
            r: bg_rgba[0],
            g: bg_rgba[1],
            b: bg_rgba[2],
        };

        self.terminal
            .save_style_set(event.id, fg_color, bg_color, event.italic);
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
            col: u32,
            line: u32,
        }

        let event: Event = serde_json::from_value(body.clone()).unwrap();

        // TODO: Avoid to check the term size at each event by saving it.
        // This will implicate to have some background process checking the
        // window size changes.
        let (size_y, _) = self.terminal.get_size();
        let mut cursor_y = (event.line as i32) - (self.screen_start as i32);

        let mut scroll: bool = false;
        if cursor_y >= (size_y as i32) {
            // The cursor is bellow the current screen view. Trigger a scroll.
            self.screen_start += (cursor_y as usize) - size_y + 1;
            scroll = true;
            cursor_y -= cursor_y - (size_y as i32) + 1
        } else if cursor_y <= -1 {
            // The cursor is abor the current screen view. Trigger a scroll.
            self.screen_start -= cursor_y.checked_abs().unwrap() as usize;
            scroll = true;
            cursor_y = 0;
        }

        // Move the cursor at its new position.
        self.cursor.x = event.col;
        self.cursor.y = cursor_y as u32;

        if scroll {
            // In case of scroll it need to redraw the screen and after it
            // the cursor is automatically reset at (self.cursor_y/self.cursor_x).
            ctx.get_peer().send_rpc_notification(
                "edit",
                &json!({
                    "method": "scroll",
                    "view_id": event.view_id,
                    "params": [self.screen_start , self.screen_start + (size_y as usize)  + 1] // + 1 bc range not inclusive
                }),
            );

            self.terminal
                .redraw_view(self.screen_start, RedrawBehavior::Everything, &self.buffer);
        } else {
            // No scroll needed so it move the cursor without any redraw.
            self.terminal.move_cursor(&self.cursor);
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
    fn handle_content_update(&mut self, ctx: &RpcCtx, body: &Value) {
        #[derive(Deserialize, Debug)]
        struct Annotation {
            #[serde(rename = "type")]
            kind: String,
            n: usize,
            payloads: Option<()>,
            ranges: Vec<[usize; 4]>,
        }

        #[derive(Deserialize, Debug)]
        struct LineDescription {
            cursor: Option<Vec<i32>>,
            ln: usize,
            styles: Vec<i32>,
            text: String,
        }

        #[derive(Deserialize, Debug)]
        struct Operation {
            #[serde(rename = "op")]
            kind: String,
            n: usize,
            ln: Option<usize>,
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
        let mut new_buffer = Buffer::default();
        let mut old_idx: usize = 0;
        let mut new_idx: usize = 0;

        for operation in event.update.operations {
            match operation.kind.as_str() {
                "copy" => {
                    let mut is_dirty = true;
                    let ln = operation.ln.unwrap();
                    if old_idx == new_idx {
                        is_dirty = false;
                    }

                    for i in 0..operation.n {
                        let old_buffer = &self.buffer.lines[old_idx + i];
                        new_buffer.lines.push(Line {
                            raw: old_buffer.raw.clone(),
                            styles: old_buffer.styles.clone(),
                            ln: ln + i,
                            is_dirty,
                        });
                        new_idx += 1;
                    }

                    old_idx += operation.n;
                }
                "skip" => old_idx += operation.n,
                "invalidate" => new_buffer.nb_invalid_lines += operation.n,
                "ins" => {
                    for line in operation.lines.unwrap() {
                        new_buffer.lines.push(Line {
                            raw: line.text.to_owned(),
                            styles: line.styles,
                            ln: line.ln,
                            is_dirty: true,
                        });
                        new_idx += 1;
                    }
                }
                _ => warn!("unhandled update 2: {:?}", operation),
            }
        }

        let (size_y, size_x) = self.terminal.get_size();
        if size_x != self.screen_width {
            ctx.get_peer().send_rpc_notification(
                "edit",
                &json!({
                    "method": "resize",
                    "view_id": event.view_id,
                    "params": {
                        "width": size_x  ,
                        "height": size_y,
                    }
                }),
            );
        }

        self.buffer = new_buffer;
        self.terminal
            .redraw_view(self.screen_start, RedrawBehavior::OnlyDirty, &self.buffer);
        self.terminal.move_cursor(&self.cursor);
    }
}
