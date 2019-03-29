use self::keys::*;
use ncurses::WchResult;
use std::char::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct KeyStroke(pub char);

impl KeyStroke {
    pub fn from_description(description: &str) -> Option<Self> {
        if description.len() == 1 {
            return Some(KeyStroke(description.chars().nth(0).unwrap()));
        }

        match description {
            "f1" => Some(KeyStroke(from_u32(KEY_F1).unwrap())),
            "key_up" => Some(KeyStroke(from_u32(KEY_UP).unwrap())),
            "key_down" => Some(KeyStroke(from_u32(KEY_DOWN).unwrap())),
            "key_left" => Some(KeyStroke(from_u32(KEY_LEFT).unwrap())),
            "key_right" => Some(KeyStroke(from_u32(KEY_RIGHT).unwrap())),
            "page_up" => Some(KeyStroke(from_u32(KEY_PPAGE).unwrap())),
            "page_down" => Some(KeyStroke(from_u32(KEY_NPAGE).unwrap())),
            _ => None,
        }
    }
}

#[derive(Default)]
pub struct Keyboard {}

impl Keyboard {
    pub fn get_next_keystroke(&self) -> KeyStroke {
        let res = ncurses::get_wch();
        if res.is_none() {
            return KeyStroke('?');
        }

        match res.unwrap() {
            WchResult::Char(c) => KeyStroke(from_u32(c).unwrap()),
            WchResult::KeyCode(k) => KeyStroke('?'),
        }
    }
}

pub mod keys {
    pub const KEY_F1: u32 = ncurses::KEY_F1 as u32;
    pub const KEY_UP: u32 = ncurses::KEY_UP as u32;
    pub const KEY_DOWN: u32 = ncurses::KEY_DOWN as u32;
    pub const KEY_LEFT: u32 = ncurses::KEY_LEFT as u32;
    pub const KEY_RIGHT: u32 = ncurses::KEY_RIGHT as u32;
    pub const KEY_PPAGE: u32 = ncurses::KEY_PPAGE as u32;
    pub const KEY_NPAGE: u32 = ncurses::KEY_NPAGE as u32;
}
