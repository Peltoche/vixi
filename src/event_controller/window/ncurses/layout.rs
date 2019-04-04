use std::panic;
use std::process::exit;
use std::sync::{Once, ONCE_INIT};

use super::window::NcursesWindow;
use crate::event_controller::window::{Layout, Window, WindowPosition, WindowSize};

use ncurses::*;

static HANDLER: Once = ONCE_INIT;

pub struct NcursesLayout {
    height: u32,
    width: u32,
}

impl NcursesLayout {
    pub fn new() -> Self {
        /* Setup ncurses. */
        initscr();
        raw();
        keypad(stdscr(), true); // Allow for extended keyboard (like F1).
        noecho();
        start_color();
        set_escdelay(0);
        halfdelay(1);

        install_custom_panic_handler();

        let mut height: i32 = 0;
        let mut width: i32 = 0;
        getmaxyx(stdscr(), &mut height, &mut width);
        if height == ERR || width == ERR {
            error!("failed to retrieve the main screen size");
        }

        Self {
            height: height as u32,
            width: width as u32,
        }
    }
}

impl Layout for NcursesLayout {
    fn create_view_window(&mut self) -> Box<dyn Window> {
        info!("height: {} / {}", self.height, self.width);
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
