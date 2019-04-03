use std::panic;
use std::process::exit;
use std::sync::{Once, ONCE_INIT};

use ncurses::*;

//const SPACES_IN_LINE_SECTION: u32 = 2;

static HANDLER: Once = ONCE_INIT;

#[derive(Clone, Debug, Default)]
pub struct Terminal {
    size_line_section: u32,
}

impl Terminal {
    pub fn new() -> Self {
        let locale_conf = LcCategory::all;
        setlocale(locale_conf, "en_US.UTF-8");

        /* Setup ncurses. */
        initscr();
        raw();
        keypad(stdscr(), true); // Allow for extended keyboard (like F1).
        noecho();
        start_color();
        //nodelay(stdscr(), true);
        set_escdelay(0);
        halfdelay(1);

        install_custom_panic_handler();

        let terminal = Self::default();

        terminal
    }

    //pub fn update_status_bar_mode(&mut self, mode: &str) {
    //let size_y = getmaxy(stdscr());

    //// Remove 1 for the command line.
    //mv(size_y - 1, 0);

    //addstr(&mode.to_uppercase());
    //}
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
