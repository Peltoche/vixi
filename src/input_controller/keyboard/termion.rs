use std::io::{stdin, Bytes, Read};
use std::thread;
use std::time::Duration;

use super::{KeyStroke, Keyboard};

use termion::event::Key;
use termion::input::TermRead;

#[derive(Default)]
pub struct TermionKeyboard {}

impl Keyboard for TermionKeyboard {
    fn get_next_keystroke(&mut self) -> Option<KeyStroke> {
        let res = stdin().keys().next();
        if res.is_none() {
            return None;
        }

        match res.unwrap().unwrap() {
            Key::Backspace => Some(KeyStroke::KeyBackSpace),
            Key::Left => Some(KeyStroke::KeyLeft),
            Key::Right => Some(KeyStroke::KeyRight),
            Key::Up => Some(KeyStroke::KeyUp),
            Key::Down => Some(KeyStroke::KeyDown),
            Key::Home => None,
            Key::End => None,
            Key::PageUp => Some(KeyStroke::KeyNextPage),
            Key::PageDown => Some(KeyStroke::KeyPreviousPage),
            Key::Delete => Some(KeyStroke::KeyDeleteChar),
            Key::Insert => None,
            Key::F(n) => Some(KeyStroke::KeyF(n)),
            Key::Alt(c) => Some(KeyStroke::Alt(c)),
            Key::Ctrl(_) => None,
            Key::Null => None,
            Key::Esc => Some(KeyStroke::KeyEscape),
            Key::Char(c) => Some(KeyStroke::Char(c)),
            _ => None,
        }
    }
}
