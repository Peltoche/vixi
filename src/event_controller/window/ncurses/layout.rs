use super::window::NcursesWindow;
use crate::event_controller::window::{Layout, Window, WindowPosition, WindowSize};

use ncurses::*;

pub struct NcursesLayout {
    height: u32,
    width: u32,
}

impl NcursesLayout {
    pub fn new() -> Self {
        let mut height: i32 = 0;
        let mut width: i32 = 0;
        getmaxyx(ncurses::stdscr(), &mut height, &mut width);

        Self {
            height: height as u32,
            width: width as u32,
        }
    }
}

impl Layout for NcursesLayout {
    fn create_view_window(&mut self) -> Box<dyn Window> {
        let window = NcursesWindow::new(
            WindowPosition { y: 0, x: 0 },
            WindowSize {
                height: self.height,
                width: self.width,
            },
        );

        Box::new(window)
    }
}
