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
    use std::collections::HashMap;

    use super::super::actions::Action;
    use super::super::keyboard::KeyStroke;

    lazy_static! {
        pub static ref DEFAULT_NORMAL_MODE_ACTIONS: HashMap<KeyStroke, Action> = {
            let mut actions = HashMap::with_capacity(21);

            // The classic arrow keys.
            actions.insert(KeyStroke::KeyUp, Action::MoveUp);
            actions.insert(KeyStroke::KeyDown, Action::MoveDown);
            actions.insert(KeyStroke::KeyLeft, Action::MoveLeft);
            actions.insert(KeyStroke::KeyRight, Action::MoveRight);
            actions.insert(KeyStroke::KeyHome, Action::MoveToLeftEndOfLine);
            actions.insert(KeyStroke::KeyEnd, Action::MoveToRightEndOfLine);

            // The "vim like" keys.
            actions.insert(KeyStroke::Char('k'), Action::MoveUp);
            actions.insert(KeyStroke::Char('j'), Action::MoveDown);
            actions.insert(KeyStroke::Char('h'), Action::MoveLeft);
            actions.insert(KeyStroke::Char('l'), Action::MoveRight);

            actions.insert(KeyStroke::Char('p'), Action::Paste);
            actions.insert(KeyStroke::Char('q'), Action::Quit);
            actions.insert(KeyStroke::Char('i'), Action::SwitchToInsertMode);
            actions.insert(KeyStroke::Char('v'), Action::SwitchToVisualMode);
            actions.insert(KeyStroke::KeySpace, Action::SwitchToActionMode);

            actions.insert(KeyStroke::Char('o'), Action::InsertLineBelow);
            actions.insert(KeyStroke::Char('O'), Action::InsertLineAbove);

            actions.insert(KeyStroke::Char('w'), Action::MoveWordRight);
            actions.insert(KeyStroke::Char('W'), Action::MoveWordLeft);

            actions.insert(KeyStroke::Char('x'), Action::DeleteForward);
            actions.insert(KeyStroke::Char('X'), Action::DeleteBackward);

            actions
        };

        pub static ref DEFAULT_ACTION_MODE_ACTIONS: HashMap<KeyStroke, Action> = {
            let mut actions = HashMap::with_capacity(3);

            actions.insert(KeyStroke::KeyEscape, Action::SwitchToNormalMode);
            actions.insert(KeyStroke::Char('q'), Action::Quit);
            actions.insert(KeyStroke::Char('w'), Action::WriteToFile);

            actions
        };

        pub static ref DEFAULT_INSERT_MODE_ACTIONS: HashMap<KeyStroke, Action> = {
            let mut actions = HashMap::with_capacity(12);

            actions.insert(KeyStroke::KeyEscape, Action::SwitchToNormalMode);
            actions.insert(KeyStroke::KeyBackSpace, Action::DeleteBackward);
            actions.insert(KeyStroke::KeyDelete, Action::DeleteForward);

            // The classic arrow keys.
            actions.insert(KeyStroke::KeyUp, Action::MoveUp);
            actions.insert(KeyStroke::KeyDown, Action::MoveDown);
            actions.insert(KeyStroke::KeyLeft, Action::MoveLeft);
            actions.insert(KeyStroke::KeyRight, Action::MoveRight);

            actions.insert(KeyStroke::KeyPreviousPage, Action::PageUp);
            actions.insert(KeyStroke::KeyNextPage, Action::PageDown);
            actions.insert(KeyStroke::KeyHome, Action::MoveToLeftEndOfLine);
            actions.insert(KeyStroke::KeyEnd, Action::MoveToRightEndOfLine);

            actions
        };

        pub static ref DEFAULT_VISUAL_MODE_ACTIONS: HashMap<KeyStroke, Action> = {
            let mut actions = HashMap::with_capacity(1);

            actions.insert(KeyStroke::KeyEscape, Action::SwitchToNormalMode);
            actions.insert(KeyStroke::Char('y'), Action::YankSelection);
            actions.insert(KeyStroke::Char('d'), Action::DeleteSelection);
            actions.insert(KeyStroke::Char('p'), Action::DeleteSelectionAndPaste);

            // The classic arrow keys.
            actions.insert(KeyStroke::KeyUp, Action::MoveUpAndSelect);
            actions.insert(KeyStroke::KeyDown, Action::MoveDownAndSelect);
            actions.insert(KeyStroke::KeyLeft, Action::MoveLeftAndSelect);
            actions.insert(KeyStroke::KeyRight, Action::MoveRightAndSelect);

            actions.insert(KeyStroke::KeyPreviousPage, Action::PageUp);
            actions.insert(KeyStroke::KeyNextPage, Action::PageDown);
            actions.insert(KeyStroke::KeyHome, Action::MoveToLeftEndOfLineAndSelect);
            actions.insert(KeyStroke::KeyEnd, Action::MoveToRightEndOfLineAndSelect);

            // The "vim like" keys.
            actions.insert(KeyStroke::Char('k'), Action::MoveUpAndSelect);
            actions.insert(KeyStroke::Char('j'), Action::MoveDownAndSelect);
            actions.insert(KeyStroke::Char('h'), Action::MoveLeftAndSelect);
            actions.insert(KeyStroke::Char('l'), Action::MoveRightAndSelect);

            actions.insert(KeyStroke::Char('w'), Action::MoveWordRightAndSelect);
            actions.insert(KeyStroke::Char('W'), Action::MoveWordLeftAndSelect);
            actions.insert(KeyStroke::Char('x'), Action::DeleteSelection);
            actions.insert(KeyStroke::Char('X'), Action::DeleteSelection);

            actions
        };
    }
}
