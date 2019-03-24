pub const KEY_F1: KeyStroke = ncurses::KEY_F1;
pub const KEY_UP: KeyStroke = ncurses::KEY_UP;
pub const KEY_DOWN: KeyStroke = ncurses::KEY_DOWN;
pub const KEY_LEFT: KeyStroke = ncurses::KEY_LEFT;
pub const KEY_RIGHT: KeyStroke = ncurses::KEY_RIGHT;
pub const KEY_PPAGE: KeyStroke = ncurses::KEY_PPAGE;
pub const KEY_NPAGE: KeyStroke = ncurses::KEY_NPAGE;

#[derive(Default)]
pub struct Keyboard {}

pub type KeyStroke = i32;

impl Keyboard {
    pub fn get_next_keystroke(&self) -> KeyStroke {
        ncurses::getch()
    }
}
