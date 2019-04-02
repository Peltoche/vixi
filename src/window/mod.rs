mod ncurses;

pub use self::ncurses::Ncurses;

pub type StyleID = i16;

/// The style id 0 is reserved for the selection style id.
///
/// This id is different than the pair id.
const SELECTION_CORE_STYLE_ID: StyleID = 0;

#[derive(Debug, Clone, Copy)]
pub struct Style {
    pub style_id: StyleID,
    pub italic: bool,
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
