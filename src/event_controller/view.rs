use crate::event_controller::Operation;
use crate::window::{StyleID, Window};

use xi_rpc::RpcCtx;

type ViewID = String;

#[derive(Debug, Clone)]
pub struct Cursor {
    pub y: u32,
    pub x: u32,
}

#[derive(Debug, Default, Clone)]
pub struct Buffer {
    pub lines: Vec<Line>,
    pub nb_invalid_lines: usize,
}

#[derive(Debug, Default, Clone)]
pub struct Line {
    pub raw: String,
    pub styles: Vec<StyleID>,
    /// The "real" line number.
    ///
    /// A line wrapped in two lines will keep the same `ln` value.
    pub ln: usize,
    /// Indicate if the line needs to be rendered during the next redraw.
    pub is_dirty: bool,
}

impl Buffer {
    pub fn total_len(&self) -> usize {
        self.lines.len() + self.nb_invalid_lines
    }

    pub fn lines_availables_after(&self, start: usize) -> usize {
        self.lines.len() - start
    }
}

pub struct View {
    id: ViewID,
    cursor: Cursor,
    buffer: Buffer,
    window: Box<dyn Window>,
    /// An index pointing to the Line rendered at the top of the screen.
    ///
    /// Changing its value make the screen scoll up/down.
    screen_start: u32,
}

impl View {
    pub fn new(view_id: &str, window: Box<dyn Window>) -> Self {
        View {
            window,
            id: view_id.to_owned(),
            cursor: Cursor { y: 0, x: 0 },
            buffer: Buffer::default(),
            screen_start: 0,
        }
    }

    pub fn move_cursor(&mut self, ctx: &RpcCtx, line: u32, col: u32) {
        let window_size = self.window.get_size();
        let mut cursor_y = (line as i32) - (self.screen_start as i32);

        let mut scroll: bool = false;
        if cursor_y >= (window_size.height as i32) {
            // The cursor is bellow the current screen view. Trigger a scroll.
            self.screen_start += (cursor_y as u32) - window_size.height + 1;
            scroll = true;
            cursor_y -= cursor_y - (window_size.height as i32) + 1
        } else if cursor_y <= -1 {
            // The cursor is abor the current screen view. Trigger a scroll.
            self.screen_start -= cursor_y.checked_abs().unwrap() as u32;
            scroll = true;
            cursor_y = 0;
        }

        // Move the cursor at its new position.
        self.cursor.x = col;
        self.cursor.y = cursor_y as u32;

        if scroll {
            self.scroll_to(
                ctx,
                self.screen_start,
                self.screen_start + window_size.height,
            );
        } else {
            // No scroll needed so it move the cursor without any redraw.
            self.window.move_cursor(self.cursor.y, self.cursor.x);
        }
    }

    pub fn scroll_to(&mut self, ctx: &RpcCtx, start: u32, end: u32) {
        ctx.get_peer().send_rpc_notification(
            "edit",
            &json!({
                "method": "scroll",
                "view_id": self.id,
                "params": [start , start + end  + 1], // + 1 bc range not inclusive
            }),
        );

        //self.terminal
        //.redraw_view(self.screen_start, RedrawBehavior::Everything, &self.buffer);
    }

    pub fn update_buffer(&mut self, operations: Vec<Operation>) {
        let mut new_buffer = Buffer::default();
        let mut old_idx: usize = 0;
        let mut new_idx: usize = 0;

        for operation in operations {
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
                _ => warn!("unhandled update: {:?}", operation),
            }
        }

        self.buffer = new_buffer;
    }
}
