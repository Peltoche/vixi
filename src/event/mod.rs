pub mod keyboard;
pub mod server;

use ncurses::*;

// Create the loop handling the keyboard events.
pub fn start_keyboard_event_loop() {
    let _keyboard_event_handler = self::keyboard::Handler::new();
    loop {
        let ch = getch();

        if ch == KEY_F1 {
            break;
        }
    }
}
