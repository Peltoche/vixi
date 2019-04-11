use super::window::Window;

pub struct StatusBar {
    window: Box<dyn Window>,
}

impl StatusBar {
    pub fn new(window: Box<dyn Window>) -> Self {
        Self { window }
    }

    pub fn update_mode(&mut self, mode: &str) {
        self.window.save_cursor_pos();
        self.window.move_cursor_and_clear_line(0);

        self.window.append_str(&mode);
        self.window.restore_cursor_pos();
        self.window.refresh();
    }
}
