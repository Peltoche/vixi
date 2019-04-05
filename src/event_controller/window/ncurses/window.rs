use crate::event_controller::window::{Window, WindowPosition, WindowSize};

use ncurses::*;

pub struct NcursesWindow {
    win: WINDOW,
    size: WindowSize,
    pos: WindowPosition,
}

impl NcursesWindow {
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

impl Window for NcursesWindow {
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

    fn save_cursor_pos(&self) {}

    fn restore_cursor_pos(&self) {}

    fn append_str(&self, s: &str) {
        waddstr(self.win, s);
    }

    fn refresh(&self) {
        if wrefresh(self.win) == ERR {
            error!("failed to refresh screen");
        }
    }
}
