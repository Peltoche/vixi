mod ncurses;

pub use self::ncurses::Ncurses;

use crate::style::Style;

#[derive(Eq, PartialEq, Debug)]
pub enum RedrawBehavior {
    OnlyDirty,
    Everything,
}

#[derive(Debug, Copy, Clone)]
pub struct WindowPosition {
    pub y: u32,
    pub x: u32,
}

#[derive(Debug, Copy, Clone)]
pub struct WindowSize {
    pub height: u32,
    pub width: u32,
}

pub trait Window {
    fn get_size(&self) -> WindowSize;
    fn move_cursor(&self, y: u32, x: u32);
    fn move_cursor_and_clear_line(&self, line: u32);
    fn append_ch(&self, ch: char, style: &Style);
    fn refresh(&self);
}
