mod rpc;

use super::KeyStroke;
use crate::core::ClientToClientWriter;

use xi_rpc::Peer;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Response {
    Continue,
    Stop,
    SwitchToInsertMode,
    SwitchToNormalMode,
    SwitchToVisualMode,
    SwitchToActionMode,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Command {
    WriteToFile,
    Quite,

    SwitchToInsertMode,
    SwitchToVisualMode,
    SwitchToActionMode,
    SwitchToNormalMode,

    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    PageUp,
    PageDown,

    MoveUpAndSelect,
    MoveDownAndSelect,
    MoveLeftAndSelect,
    MoveRightAndSelect,

    YankSelection,
    DeleteSelection,
    DeleteSelectionAndPaste,

    Paste,

    InsertLineBelow,
    InsertLineAbove,

    DeleteBackward,
    DeleteForward,

    // Custom for the insert mode. Not configurable
    InsertKeyStroke(KeyStroke),
}

impl Command {
    pub fn execute(
        &self,
        view_id: &str,
        core: &dyn Peer,
        front_event_writer: &mut ClientToClientWriter,
    ) -> Response {
        match *self {
            Command::WriteToFile => rpc::write_to_file(view_id, front_event_writer),
            Command::Quite => rpc::quite(view_id, core),

            Command::SwitchToInsertMode => Response::SwitchToInsertMode,
            Command::SwitchToVisualMode => Response::SwitchToVisualMode,
            Command::SwitchToActionMode => Response::SwitchToActionMode,
            Command::SwitchToNormalMode => Response::SwitchToNormalMode,

            Command::MoveUp => rpc::move_up(view_id, core),
            Command::MoveDown => rpc::move_down(view_id, core),
            Command::MoveLeft => rpc::move_left(view_id, core),
            Command::MoveRight => rpc::move_right(view_id, core),
            Command::PageUp => rpc::page_up(view_id, core),
            Command::PageDown => rpc::page_down(view_id, core),

            Command::MoveUpAndSelect => rpc::move_up_and_select(view_id, core),
            Command::MoveDownAndSelect => rpc::move_down_and_select(view_id, core),
            Command::MoveLeftAndSelect => rpc::move_left_and_select(view_id, core),
            Command::MoveRightAndSelect => rpc::move_right_and_select(view_id, core),

            Command::YankSelection => rpc::yank_selection(view_id, core),
            Command::DeleteSelection => rpc::cute_selection(view_id, core),
            Command::DeleteSelectionAndPaste => rpc::cute_selection_and_paste(view_id, core),

            Command::Paste => rpc::paste(view_id, core),

            Command::InsertKeyStroke(k) => rpc::insert_keystroke(view_id, k, core),
            Command::InsertLineBelow => rpc::insert_line_below(view_id, core),
            Command::InsertLineAbove => rpc::insert_line_above(view_id, core),

            Command::DeleteBackward => rpc::delete_backward(view_id, core),
            Command::DeleteForward => rpc::delete_forward(view_id, core),
        }
    }

    pub fn from_description(desc: &str) -> Option<Command> {
        match desc {
            ":write_to_file" => Some(Command::WriteToFile),
            ":quit" => Some(Command::Quite),

            ":switch_to_insert_mode" => Some(Command::SwitchToInsertMode),
            ":switch_to_visual_mode" => Some(Command::SwitchToVisualMode),
            ":switch_to_action_mode" => Some(Command::SwitchToActionMode),
            ":switch_to_normal_mode" => Some(Command::SwitchToNormalMode),

            ":move_up" => Some(Command::MoveUp),
            ":move_down" => Some(Command::MoveDown),
            ":move_left" => Some(Command::MoveLeft),
            ":move_right" => Some(Command::MoveRight),
            ":page_up" => Some(Command::PageUp),
            ":page_down" => Some(Command::PageDown),

            ":move_up_and_select" => Some(Command::MoveUpAndSelect),
            ":move_down_and_select" => Some(Command::MoveDownAndSelect),
            ":move_left_and_select" => Some(Command::MoveLeftAndSelect),
            ":move_right_and_select" => Some(Command::MoveRightAndSelect),

            ":yank_selection" => Some(Command::YankSelection),
            ":delete_selection" => Some(Command::DeleteSelection),
            ":delete_selection_and_past" => Some(Command::DeleteSelectionAndPaste),

            ":paste" => Some(Command::Paste),

            ":insert_line_below" => Some(Command::InsertLineBelow),
            ":insert_line_above" => Some(Command::InsertLineAbove),

            ":delete_backward" => Some(Command::DeleteBackward),
            ":delete_forward" => Some(Command::DeleteForward),

            _ => None,
        }
    }
}
