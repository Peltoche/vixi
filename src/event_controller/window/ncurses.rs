use super::{Window, WindowPosition, WindowSize};
use crate::event_controller::style::Style;

use ncurses::*;

pub struct Ncurses {
    win: WINDOW,
    size: WindowSize,
    pos: WindowPosition,
}

impl Ncurses {
    /// Create a new window at the given position with the given size.
    pub fn new(pos: WindowPosition, size: WindowSize) -> Self {
        let win = newwin(
            size.height as i32,
            size.width as i32,
            pos.y as i32,
            pos.x as i32,
        );
        if wrefresh(win) == ERR {
            error!("failed to refresh the window during initialization");
        }

        Self { win, pos, size }
    }
}

impl Window for Ncurses {
    fn get_size(&self) -> WindowSize {
        self.size
    }

    fn move_cursor(&self, y: u32, x: u32) {
        if wmove(self.win, y as i32, x as i32) == ERR {
            error!("failed to move the cursor");
        }
    }

    fn move_cursor_and_clear_line(&self, line: u32) {
        if wmove(self.win, line as i32, 0) == ERR {
            error!("failed to move the cursor for clearing the line")
        }

        if wclrtoeol(self.win) == ERR {
            error!("failed to clear the line");
        }
    }

    fn append_ch(&self, ch: char, style: &Style) {
        let attrs = attrs_from_style(style);

        waddch(self.win, ch as chtype | attrs);
    }

    fn refresh(&self) {
        if wrefresh(self.win) == ERR {
            error!("failed to refresh screen");
        }
    }
}

fn attrs_from_style(style: &Style) -> attr_t {
    let mut attrs = COLOR_PAIR(style.style_id);
    attrs = attrs | if style.italic { A_ITALIC() } else { A_NORMAL() };

    attrs
}
