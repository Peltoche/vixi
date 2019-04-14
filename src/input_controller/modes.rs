use std::collections::HashMap;

use super::commands::Command;
use super::keyboard::KeyStroke;
use super::Mode;

#[derive(Debug, Default)]
pub struct ModeCommands(HashMap<KeyStroke, Command>);

impl ModeCommands {
    pub fn setup(mode: Mode, config_map: &HashMap<KeyStroke, Command>) -> Self {
        let mut commands = match mode {
            Mode::Normal => defaults::DEFAULT_NORMAL_MODE_COMMANDS.clone(),
            Mode::Insert => defaults::DEFAULT_INSERT_MODE_COMMANDS.clone(),
            Mode::Visual => defaults::DEFAULT_VISUAL_MODE_COMMANDS.clone(),
            Mode::Action => defaults::DEFAULT_ACTION_MODE_COMMANDS.clone(),
        };

        for (key, command) in config_map.iter() {
            commands.insert(*key, *command);
        }

        Self(commands)
    }

    pub fn get_action_from_keystroke(&self, keystroke: &KeyStroke) -> Option<Command> {
        self.0.get(keystroke).map(|x| *x)
    }
}

pub mod defaults {
    use std::collections::HashMap;

    use super::super::commands::Command;
    use super::super::keyboard::KeyStroke;

    lazy_static! {
        pub static ref DEFAULT_NORMAL_MODE_COMMANDS: HashMap<KeyStroke, Command> = {
            let mut commands = HashMap::with_capacity(12);

            // The classic arrow keys.
            commands.insert(KeyStroke::KeyUp, Command::MoveUpAndSelect);
            commands.insert(KeyStroke::KeyDown, Command::MoveDownAndSelect);
            commands.insert(KeyStroke::KeyLeft, Command::MoveLeftAndSelect);
            commands.insert(KeyStroke::KeyRight, Command::MoveRightAndSelect);

            // The "vim like" keys.
            commands.insert(KeyStroke::Char('k'), Command::MoveUpAndSelect);
            commands.insert(KeyStroke::Char('j'), Command::MoveDownAndSelect);
            commands.insert(KeyStroke::Char('h'), Command::MoveLeftAndSelect);
            commands.insert(KeyStroke::Char('l'), Command::MoveRightAndSelect);

            commands.insert(KeyStroke::Char('i'), Command::SwitchToInsertMode);
            commands.insert(KeyStroke::Char('v'), Command::SwitchToVisualMode);

            commands.insert(KeyStroke::Char('o'), Command::InsertLineBelow);
            commands.insert(KeyStroke::Char('O'), Command::InsertLineAbove);

            commands
        };

        pub static ref DEFAULT_ACTION_MODE_COMMANDS: HashMap<KeyStroke, Command> = {
            let mut commands = HashMap::with_capacity(3);

            commands.insert(KeyStroke::KeyEscape, Command::SwitchToNormalMode);
            commands.insert(KeyStroke::Char('q'), Command::Quite);
            commands.insert(KeyStroke::Char('w'), Command::WriteToFile);

            commands
    };

        pub static ref DEFAULT_INSERT_MODE_COMMANDS: HashMap<KeyStroke, Command> = {
            let mut commands = HashMap::with_capacity(12);

            commands.insert(KeyStroke::KeyEscape, Command::SwitchToNormalMode);
            commands.insert(KeyStroke::KeyBackSpace, Command::DeleteBackward);
            commands.insert(KeyStroke::KeyDelete, Command::DeleteForward);

            // The classic arrow keys.
            commands.insert(KeyStroke::KeyUp, Command::MoveUp);
            commands.insert(KeyStroke::KeyDown, Command::MoveDown);
            commands.insert(KeyStroke::KeyLeft, Command::MoveLeft);
            commands.insert(KeyStroke::KeyRight, Command::MoveRight);
            commands.insert(KeyStroke::KeyPreviousPage, Command::PageUp);
            commands.insert(KeyStroke::KeyNextPage, Command::PageDown);

            commands
    };

        pub static ref DEFAULT_VISUAL_MODE_COMMANDS: HashMap<KeyStroke, Command> = {
            let mut commands = HashMap::with_capacity(1);

            commands.insert(KeyStroke::KeyEscape, Command::SwitchToNormalMode);
            commands.insert(KeyStroke::Char('y'), Command::YankSelection);
            commands.insert(KeyStroke::Char('d'), Command::DeleteSelection);
            commands.insert(KeyStroke::Char('p'), Command::DeleteSelectionAndPaste);

            // The classic arrow keys.
            commands.insert(KeyStroke::KeyUp, Command::MoveUpAndSelect);
            commands.insert(KeyStroke::KeyDown, Command::MoveDownAndSelect);
            commands.insert(KeyStroke::KeyLeft, Command::MoveLeftAndSelect);
            commands.insert(KeyStroke::KeyRight, Command::MoveRightAndSelect);

            // The "vim like" keys.
            commands.insert(KeyStroke::Char('k'), Command::MoveUpAndSelect);
            commands.insert(KeyStroke::Char('j'), Command::MoveDownAndSelect);
            commands.insert(KeyStroke::Char('h'), Command::MoveLeftAndSelect);
            commands.insert(KeyStroke::Char('l'), Command::MoveRightAndSelect);

            commands
    };
    }
}
