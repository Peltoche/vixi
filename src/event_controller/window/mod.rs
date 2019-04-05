pub mod ncurses;
pub mod termion;

pub use self::ncurses::NcursesLayout;
pub use self::termion::TermionLayout;

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
    fn refresh(&self);
    fn append_str(&self, s: &str);
    fn save_cursor_pos(&self);
    fn restore_cursor_pos(&self);
}

pub trait Layout {
    fn create_view_window(&self) -> Box<dyn Window>;
    fn create_new_status_bar_window(&self) -> Box<dyn Window>;
    fn clear(&self);
}
