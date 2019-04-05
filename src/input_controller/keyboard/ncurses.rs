use std::char::*;

use super::{KeyStroke, Keyboard};

use ncurses::*;

const ESC_OR_ALT_KEY: i32 = 27;

#[derive(Default)]
pub struct NcursesKeyboard {}

impl Keyboard for NcursesKeyboard {
    fn get_next_keystroke(&mut self) -> Option<KeyStroke> {
        let res = ncurses::get_wch();
        if res.is_none() {
            return None;
        }

        match res.unwrap() {
            WchResult::KeyCode(k) => match k as i32 {
                KEY_DOWN => Some(KeyStroke::KeyDown),
                KEY_UP => Some(KeyStroke::KeyUp),
                KEY_LEFT => Some(KeyStroke::KeyLeft),
                KEY_RIGHT => Some(KeyStroke::KeyRight),
                KEY_NPAGE => Some(KeyStroke::KeyNextPage),
                KEY_PPAGE => Some(KeyStroke::KeyPreviousPage),
                KEY_DC => Some(KeyStroke::KeyDeleteChar),
                _ => {
                    warn!("unhandled keycode: {}", k);
                    Some(KeyStroke::Char('?'))
                }
            },

            WchResult::Char(c) => {
                match c as i32 {
                    KEY_BACKSPACE | 127 => Some(KeyStroke::KeyBackSpace),
                    ESC_OR_ALT_KEY => {
                        // Don't wait for another key
                        // If it was Alt then curses has already sent the other key
                        // otherwise -1 is sent (Escape)
                        let next_key = getch();
                        if next_key == -1 {
                            return Some(KeyStroke::KeyEscape);
                        }

                        Some(KeyStroke::Alt(from_u32(next_key as u32).unwrap_or('?')))
                    }
                    _ => Some(KeyStroke::Char(from_u32(c as u32).unwrap())),
                }
            }
        }
    }
}
