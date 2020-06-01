use std::collections::HashMap;

use super::actions::Action;
use super::keyboard::KeyStroke;
use super::Mode;

#[derive(Debug)]
pub struct ModeActions(HashMap<KeyStroke, Action>);

impl ModeActions {
    pub fn setup(mode: Mode, config_map: &HashMap<String, String>) -> Self {
        let mut actions = match mode {
            Mode::Normal => defaults::DEFAULT_NORMAL_MODE_ACTIONS.clone(),
            Mode::Insert => defaults::DEFAULT_INSERT_MODE_ACTIONS.clone(),
            Mode::Visual => defaults::DEFAULT_VISUAL_MODE_ACTIONS.clone(),
            Mode::Action => defaults::DEFAULT_ACTION_MODE_ACTIONS.clone(),
        };

        for (action_desc, key_desc) in config_map.iter() {
            let key = KeyStroke::from_description(&key_desc);
            if key.is_none() {
                continue;
            }

            let action = Action::from_description(action_desc);
            if action.is_none() {
                continue;
            }

            actions.insert(key.unwrap(), action.unwrap());
        }

        Self(actions)
    }

    pub fn get_action_from_keystroke(&self, keystroke: KeyStroke) -> Option<Action> {
        self.0.get(&keystroke).cloned()
    }
}

pub mod defaults {
    use maplit::hashmap;
    use std::collections::HashMap;

    use super::super::actions::Action;
    use super::super::keyboard::KeyStroke;

    lazy_static! {
        pub static ref DEFAULT_NORMAL_MODE_ACTIONS: HashMap<KeyStroke, Action> = hashmap! {
            KeyStroke::KeyUp => Action::MoveUp,
            KeyStroke::KeyDown => Action::MoveDown,
            KeyStroke::KeyLeft => Action::MoveLeft,
            KeyStroke::KeyRight => Action::MoveRight,
            KeyStroke::Char('k') => Action::MoveUp,
            KeyStroke::Char('j') => Action::MoveDown,
            KeyStroke::Char('h') => Action::MoveLeft,
            KeyStroke::Char('l') => Action::MoveRight,
            KeyStroke::Char('p') => Action::Paste,
            KeyStroke::Char('q') => Action::Quite,
            KeyStroke::Char('i') => Action::SwitchToInsertMode,
            KeyStroke::Char('v') => Action::SwitchToVisualMode,
            KeyStroke::KeySpace => Action::SwitchToActionMode,
            KeyStroke::Char('o') => Action::InsertLineBelow,
            KeyStroke::Char('O') => Action::InsertLineAbove,
            KeyStroke::Char('w') => Action::MoveWordRight,
            KeyStroke::Char('W') => Action::MoveWordLeft,
            KeyStroke::Char('x') => Action::DeleteForward,
            KeyStroke::Char('X') => Action::DeleteBackward,
            KeyStroke::Char('>') => Action::Indent,
            KeyStroke::Char('<') => Action::Outdent,
        };
        pub static ref DEFAULT_ACTION_MODE_ACTIONS: HashMap<KeyStroke, Action> = hashmap! {
            KeyStroke::KeyEscape => Action::SwitchToNormalMode,
            KeyStroke::Char('q') => Action::Quite,
            KeyStroke::Char('w') => Action::WriteToFile,
        };
        pub static ref DEFAULT_INSERT_MODE_ACTIONS: HashMap<KeyStroke, Action> = hashmap! {
            KeyStroke::KeyEscape => Action::SwitchToNormalMode,
            KeyStroke::KeyBackSpace => Action::DeleteBackward,
            KeyStroke::KeyDelete => Action::DeleteForward,
            KeyStroke::KeyUp => Action::MoveUp,
            KeyStroke::KeyDown => Action::MoveDown,
            KeyStroke::KeyLeft => Action::MoveLeft,
            KeyStroke::KeyRight => Action::MoveRight,
            KeyStroke::KeyPreviousPage => Action::PageUp,
            KeyStroke::KeyNextPage => Action::PageDown,
        };
        pub static ref DEFAULT_VISUAL_MODE_ACTIONS: HashMap<KeyStroke, Action> = hashmap! {
            KeyStroke::KeyEscape => Action::SwitchToNormalMode,
            KeyStroke::Char('q') => Action::SwitchToNormalMode,
            KeyStroke::Char('y') => Action::YankSelection,
            KeyStroke::Char('d') => Action::DeleteSelection,
            KeyStroke::Char('p') => Action::DeleteSelectionAndPaste,
            KeyStroke::KeyUp => Action::MoveUpAndSelect,
            KeyStroke::KeyDown => Action::MoveDownAndSelect,
            KeyStroke::KeyLeft => Action::MoveLeftAndSelect,
            KeyStroke::KeyRight => Action::MoveRightAndSelect,
            KeyStroke::Char('k') => Action::MoveUpAndSelect,
            KeyStroke::Char('j') => Action::MoveDownAndSelect,
            KeyStroke::Char('h') => Action::MoveLeftAndSelect,
            KeyStroke::Char('l') => Action::MoveRightAndSelect,
            KeyStroke::Char('w') => Action::MoveWordRightAndSelect,
            KeyStroke::Char('W') => Action::MoveWordLeftAndSelect,
        };
    }
}
