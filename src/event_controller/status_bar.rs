use std::cell::RefCell;
use std::rc::Rc;

use super::style::Styles;
use super::window::Window;

pub struct StatusBar {
    window: Box<dyn Window>,
    styles: Rc<RefCell<Box<dyn Styles>>>,
}

impl StatusBar {
    pub fn new(window: Box<dyn Window>, styles: Rc<RefCell<Box<dyn Styles>>>) -> Self {
        Self { window, styles }
    }

    pub fn update_mode(&mut self, mode: &str) {
        self.window.save_cursor_pos();
        self.window.move_cursor_and_clear_line(0);
        self.styles.borrow().set_default();
        self.window.append_str(mode);
        self.window.restore_cursor_pos();
        self.window.refresh();
    }
}
