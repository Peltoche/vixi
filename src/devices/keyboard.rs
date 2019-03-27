use self::keys::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct KeyStroke(i32);

impl KeyStroke {
    pub fn from_description(description: &str) -> Option<Self> {
        if description.len() == 1 {
            return Some(KeyStroke(description.chars().nth(0).unwrap() as i32));
        }

        match description {
            "f1" => Some(KEY_F1),
            "key_up" => Some(KEY_UP),
            "key_down" => Some(KEY_DOWN),
            "key_left" => Some(KEY_LEFT),
            "key_right" => Some(KEY_RIGHT),
            "page_up" => Some(KEY_PPAGE),
            "page_down" => Some(KEY_NPAGE),
            _ => None,
        }
    }

    pub fn from_char(c: char) -> Option<Self> {
        // TODO: The input should be validated. There is a lot of possible
        // char not available to a keyboard.
        return Some(KeyStroke(c as i32));
    }
}

#[derive(Default)]
pub struct Keyboard {}

impl Keyboard {
    pub fn get_next_keystroke(&self) -> KeyStroke {
        KeyStroke(ncurses::getch())
    }
}

pub mod keys {
    use super::KeyStroke;

    pub const KEY_F1: KeyStroke = KeyStroke(ncurses::KEY_F1);
    pub const KEY_UP: KeyStroke = KeyStroke(ncurses::KEY_UP);
    pub const KEY_DOWN: KeyStroke = KeyStroke(ncurses::KEY_DOWN);
    pub const KEY_LEFT: KeyStroke = KeyStroke(ncurses::KEY_LEFT);
    pub const KEY_RIGHT: KeyStroke = KeyStroke(ncurses::KEY_RIGHT);
    pub const KEY_PPAGE: KeyStroke = KeyStroke(ncurses::KEY_PPAGE);
    pub const KEY_NPAGE: KeyStroke = KeyStroke(ncurses::KEY_NPAGE);
}
