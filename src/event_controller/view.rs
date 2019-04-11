use std::cell::RefCell;
use std::rc::Rc;

use super::style::{Styles, LINE_SECTION_STYLE_ID, STYLE_LEN};
use super::window::Window;
use super::Operation;

use xi_rpc::RpcCtx;

pub type ViewID = String;

const SPACES_IN_LINE_SECTION: usize = 2;

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
    /// The "real" line number.
    ///
    /// A line wrapped in two lines will keep the same `ln` value.
    pub ln: Option<usize>,
    /// Indicate if the line needs to be rendered during the next redraw.
    pub is_dirty: bool,
}

#[derive(Eq, PartialEq, Debug)]
pub enum RedrawBehavior {
    OnlyDirty,
    Everything,
}

impl Buffer {
    pub fn total_len(&self) -> usize {
        self.lines.len() + self.nb_invalid_lines
    }

    pub fn lines_availables_after(&self, start: u32) -> u32 {
        (self.lines.len() as u32) - start
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
    styles: Rc<RefCell<Box<dyn Styles>>>,
    width_line_section: u32,
}

impl View {
    pub fn new(
        ctx: &RpcCtx,
        view_id: &str,
        window: Box<dyn Window>,
        styles: Rc<RefCell<Box<dyn Styles>>>,
    ) -> Self {
        let window_size = window.get_size();

        let view = View {
            window,
            styles,
            id: view_id.to_string(),
            cursor: Cursor { y: 0, x: 0 },
            buffer: Buffer::default(),
            screen_start: 0,
            width_line_section: 0,
        };

        ctx.get_peer().send_rpc_notification(
            "edit",
            &json!({
                "method": "resize",
                "view_id": view_id,
                "params": {
                    "width": window_size.width,
                    "height": window_size.height,
                }
            }),
        );

        ctx.get_peer().send_rpc_notification(
            "edit",
            &json!({
            "method": "scroll",
            "view_id": view_id,
            "params": [0 ,window_size.height + 1] // + 1 bc range not inclusive
            }),
        );

        view
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
        self.cursor.x = col + self.width_line_section;
        self.cursor.y = cursor_y as u32;

        if scroll {
            // The scroll require a full redraw
            self.redraw_view(RedrawBehavior::Everything);
        } else {
            // No scroll needed so it move the cursor without any redraw.
            self.window.move_cursor(self.cursor.y, self.cursor.x);
            self.window.refresh();
        }
    }

    pub fn update_buffer(&mut self, operations: Vec<Operation>) {
        let mut new_buffer = Buffer::default();
        let mut old_idx: usize = 0;
        let mut new_idx: usize = 0;

        let styles = self.styles.borrow();
        for operation in operations {
            match operation.kind.as_str() {
                "copy" => {
                    let is_dirty = old_idx != new_idx;

                    for i in 0..operation.n {
                        let old_buffer = &self.buffer.lines[old_idx + i];
                        new_buffer.lines.push(Line {
                            raw: old_buffer.raw.clone(),
                            ln: operation.ln.map(|ln| ln + i),
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
                            raw: styles.apply_to(line.styles, &line.text),
                            ln: line.ln,
                            is_dirty: true,
                        });
                        new_idx += 1;
                    }
                }
                _ => warn!("unhandled update: {:?}", operation),
            }
        }

        self.width_line_section =
            ((new_buffer.total_len().to_string().len()) + SPACES_IN_LINE_SECTION) as u32;

        self.buffer = new_buffer;
        self.redraw_view(RedrawBehavior::OnlyDirty);
    }

    pub fn redraw_view(&self, redraw_behavior: RedrawBehavior) {
        let window_size = self.window.get_size();
        let styles_registry = self.styles.borrow();

        let buffer_len =
            if self.buffer.lines_availables_after(self.screen_start) < window_size.height {
                // The number of lines inside the buffer is less than the available lines on the screen so
                // it print all the remaining of the buffer.
                self.buffer.lines_availables_after(self.screen_start)
            } else {
                // The number of lines inside the buffer is greater than the available lines on the screen so
                // it print only what the screen is able to show.
                window_size.height
            };

        let buffer_iter = self
            .buffer
            .lines
            .iter()
            .skip(self.screen_start as usize)
            .take(buffer_len as usize);

        for (screen_line, line) in buffer_iter.enumerate() {
            if redraw_behavior == RedrawBehavior::Everything || line.is_dirty {
                self.window.move_cursor_and_clear_line(screen_line as u32);

                // Print the line number.
                let ln = match line.ln {
                    Some(ln) => ln.to_string(),
                    None => String::from(""),
                };

                let line_size = (self.width_line_section as usize) - SPACES_IN_LINE_SECTION;
                let mut line_section = String::with_capacity(line_size + STYLE_LEN);
                styles_registry.append_with_style(
                    &format!(" {:width$} ", ln, width = line_size),
                    LINE_SECTION_STYLE_ID,
                    &mut line_section,
                );

                self.window.append_str(&line_section);
                self.window.append_str(&line.raw);
            }
        }

        self.window.move_cursor(self.cursor.y, self.cursor.x);
        self.window.refresh();
    }
}
