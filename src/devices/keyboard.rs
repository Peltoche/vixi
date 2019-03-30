use std::char::*;

use ncurses::{getch, WchResult};

const ESC_OR_ALT_KEY: u32 = 27;

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

        let c_u32 = match res.unwrap() {
            WchResult::Char(c) => c,
            WchResult::KeyCode(k) => {
                warn!("unhandled keycode: {}", k);
                '?' as u32
            }
        };

        if c_u32 == ESC_OR_ALT_KEY {
            // Don't wait for another key
            // If it was Alt then curses has already sent the other key
            // otherwise -1 is sent (Escape)
            let next_key = getch();
            if next_key == -1 {
                return Some(KeyStroke::KeyEscape);
            }

            return Some(KeyStroke::Alt(from_u32(next_key as u32).unwrap_or('?')));
        }

        Some(KeyStroke::Char(from_u32(c_u32).unwrap()))
    }
}
