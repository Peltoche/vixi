mod termion;

pub use self::termion::TermionKeyboard;

pub trait Keyboard {
    fn get_next_keystroke(&mut self) -> Option<KeyStroke>;
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum KeyStroke {
    Char(char),
    KeyF(u8),
    Alt(char),
    KeyUp,
    KeyDown,
    KeyLeft,
    KeyRight,
    KeyPreviousPage,
    KeyNextPage,
    KeyEscape,
    KeyBackSpace,
    KeyDelete,
    KeySpace,
}

impl KeyStroke {
    pub fn from_description(description: &str) -> Option<Self> {
        if description.len() == 1 {
            return Some(KeyStroke::Char(description.chars().nth(0).unwrap()));
        }

        match description {
            "<f1>" => Some(KeyStroke::KeyF(1)),
            "<key_up>" => Some(KeyStroke::KeyUp),
            "<key_down>" => Some(KeyStroke::KeyDown),
            "<key_left>" => Some(KeyStroke::KeyLeft),
            "<key_right>" => Some(KeyStroke::KeyRight),
            "<page_up>" => Some(KeyStroke::KeyPreviousPage),
            "<page_down>" => Some(KeyStroke::KeyNextPage),
            "<backspace>" => Some(KeyStroke::KeyBackSpace),
            "<del>" => Some(KeyStroke::KeyDelete),
            "<space>" => Some(KeyStroke::KeySpace),
            "<esc>" => Some(KeyStroke::KeyEscape),
            _ => None,
        }
    }
}
