use std::panic;
use std::process::exit;
use std::sync::{Once, ONCE_INIT};

use ncurses::*;

static HANDLER: Once = ONCE_INIT;

pub struct Terminal {}

impl Terminal {
    pub fn new() -> Self {
        /* Setup ncurses. */
        initscr();
        raw();
        keypad(stdscr(), true); // Allow for extended keyboard (like F1).
        noecho();
        start_color();

        install_custom_panic_handler();

        Self {}
    }

    pub fn move_cursor(&self, y: usize, x: usize) {
        mv(y as i32, x as i32);
    }
}

fn install_custom_panic_handler() {
    HANDLER.call_once(|| {
        let default_handler = panic::take_hook();
        panic::set_hook(Box::new(move |info| {
            // Clean the terminal.
            endwin();

            // Run the default panic handler.
            default_handler(info);

            // Exit with the status '1'.
            exit(1);
        }));
    })
}
