use std::cell::RefCell;
use std::io::Write;
use std::rc::Rc;

use crate::event_controller::window::{Window, WindowPosition, WindowSize};

use termion::{clear, cursor};

pub struct TermionWindow {
    writer: Rc<RefCell<Box<dyn Write>>>,
    size: WindowSize,
    pos: WindowPosition,
}

impl TermionWindow {
    /// Create a new window at the given position with the given size.
    pub fn new(writer: Rc<RefCell<Box<dyn Write>>>, pos: WindowPosition, size: WindowSize) -> Self {
        Self { writer, pos, size }
    }
}

impl Window for TermionWindow {
    fn get_size(&self) -> WindowSize {
        self.size
    }

    fn move_cursor(&self, y: u32, x: u32) {
        // The Goto function is (1, 1)-based
        write!(
            self.writer.borrow_mut(),
            "{}",
            cursor::Goto((self.pos.x + x) as u16 + 1, (self.pos.y + y + 1) as u16),
        )
        .unwrap();
    }

    fn save_cursor_pos(&self) {
        write!(self.writer.borrow_mut(), "{}", cursor::Save).unwrap();
    }

    fn restore_cursor_pos(&self) {
        write!(self.writer.borrow_mut(), "{}", cursor::Restore).unwrap();
    }

    fn move_cursor_and_clear_line(&self, line: u32) {
        // The Goto function is (1, 1)-based
        write!(
            self.writer.borrow_mut(),
            "{}{}",
            cursor::Goto(1, (self.pos.y + line + 1) as u16),
            clear::CurrentLine
        )
        .unwrap();
    }

    fn append_str(&self, s: &str) {
        write!(self.writer.borrow_mut(), "{}", s).unwrap();
    }

    fn refresh(&self) {
        self.writer.borrow_mut().flush().unwrap();
    }
}
