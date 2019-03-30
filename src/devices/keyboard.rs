use std::char::*;

use ncurses::*;

const ESC_OR_ALT_KEY: i32 = 27;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
//pub struct KeyStroke(pub char);
pub enum KeyStroke {
    Char(char),
    KeyF(u32),
    Alt(char),
    KeyUp,
    KeyDown,
    KeyLeft,
    KeyRight,
    KeyPreviousPage,
    KeyNextPage,
    KeyEscape,
    KeyBackSpace,
    KeyDeleteChar,
}

impl KeyStroke {
    pub fn from_description(description: &str) -> Option<Self> {
        if description.len() == 1 {
            return Some(KeyStroke::Char(description.chars().nth(0).unwrap()));
        }

        match description {
            "f1" => Some(KeyStroke::KeyF(1)),
            "key_up" => Some(KeyStroke::KeyUp),
            "key_down" => Some(KeyStroke::KeyDown),
            "key_left" => Some(KeyStroke::KeyLeft),
            "key_right" => Some(KeyStroke::KeyRight),
            "page_up" => Some(KeyStroke::KeyPreviousPage),
            "page_down" => Some(KeyStroke::KeyNextPage),
            "backspace" => Some(KeyStroke::KeyBackSpace),
            _ => None,
        }
    }
}

#[derive(Default)]
pub struct Keyboard {}

impl Keyboard {
    pub fn get_next_keystroke(&self) -> Option<KeyStroke> {
        let res = ncurses::get_wch();
        if res.is_none() {
            return None;
        }

        match res.unwrap() {
            WchResult::KeyCode(k) => match k as i32 {
                330 => Some(KeyStroke::KeyDeleteChar),
                _ => {
                    warn!("unhandled keycode: {}", k);
                    Some(KeyStroke::Char('?'))
                }
            },

            WchResult::Char(c) => {
                match c as i32 {
                    KEY_BACKSPACE | 127 => Some(KeyStroke::KeyBackSpace),
                    KEY_UP => Some(KeyStroke::KeyUp),
                    KEY_DOWN => Some(KeyStroke::KeyDown),
                    KEY_LEFT => Some(KeyStroke::KeyLeft),
                    KEY_RIGHT => Some(KeyStroke::KeyRight),
                    KEY_NPAGE => Some(KeyStroke::KeyNextPage),
                    KEY_PPAGE => Some(KeyStroke::KeyPreviousPage),
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
