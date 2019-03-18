use ncurses::*;

pub struct Terminal {}

impl Terminal {
    pub fn new() -> Self {
        /* Setup ncurses. */
        initscr();
        raw();

        /* Allow for extended keyboard (like F1). */
        keypad(stdscr(), true);
        noecho();

        Self {}
    }

    pub fn clean(&self) {
        endwin();
    }
}
