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
pub enum Action {
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

    MoveWordRight,
    MoveWordLeft,

    PageUp,
    PageDown,

    MoveUpAndSelect,
    MoveDownAndSelect,
    MoveLeftAndSelect,
    MoveRightAndSelect,
    MoveWordRightAndSelect,
    MoveWordLeftAndSelect,

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

impl Action {
    pub fn execute(
        self,
        view_id: &str,
        core: &dyn Peer,
        front_event_writer: &mut ClientToClientWriter,
    ) -> Response {
        match self {
            Action::WriteToFile => rpc::write_to_file(view_id, front_event_writer),
            Action::Quite => rpc::quite(view_id, core),

            Action::SwitchToInsertMode => Response::SwitchToInsertMode,
            Action::SwitchToVisualMode => Response::SwitchToVisualMode,
            Action::SwitchToActionMode => Response::SwitchToActionMode,
            Action::SwitchToNormalMode => Response::SwitchToNormalMode,

            Action::MoveUp => rpc::move_up(view_id, core),
            Action::MoveDown => rpc::move_down(view_id, core),
            Action::MoveLeft => rpc::move_left(view_id, core),
            Action::MoveRight => rpc::move_right(view_id, core),

            Action::MoveWordRight => rpc::move_word_right(view_id, core),
            Action::MoveWordLeft => rpc::move_word_left(view_id, core),

            Action::PageUp => rpc::page_up(view_id, core),
            Action::PageDown => rpc::page_down(view_id, core),

            Action::MoveUpAndSelect => rpc::move_up_and_select(view_id, core),
            Action::MoveDownAndSelect => rpc::move_down_and_select(view_id, core),
            Action::MoveLeftAndSelect => rpc::move_left_and_select(view_id, core),
            Action::MoveRightAndSelect => rpc::move_right_and_select(view_id, core),
            Action::MoveWordRightAndSelect => rpc::move_word_right_and_select(view_id, core),
            Action::MoveWordLeftAndSelect => rpc::move_word_left_and_select(view_id, core),

            Action::YankSelection => rpc::yank_selection(view_id, core),
            Action::DeleteSelection => rpc::cute_selection(view_id, core),
            Action::DeleteSelectionAndPaste => rpc::cute_selection_and_paste(view_id, core),

            Action::Paste => rpc::paste(view_id, core),

            Action::InsertKeyStroke(k) => rpc::insert_keystroke(view_id, k, core),
            Action::InsertLineBelow => rpc::insert_line_below(view_id, core),
            Action::InsertLineAbove => rpc::insert_line_above(view_id, core),

            Action::DeleteBackward => rpc::delete_backward(view_id, core),
            Action::DeleteForward => rpc::delete_forward(view_id, core),
        }
    }

    pub fn from_description(desc: &str) -> Option<Action> {
        match desc {
            "write_to_file" => Some(Action::WriteToFile),
            "quit" => Some(Action::Quite),

            "switch_to_insert_mode" => Some(Action::SwitchToInsertMode),
            "switch_to_visual_mode" => Some(Action::SwitchToVisualMode),
            "switch_to_action_mode" => Some(Action::SwitchToActionMode),
            "switch_to_normal_mode" => Some(Action::SwitchToNormalMode),

            "move_up" => Some(Action::MoveUp),
            "move_down" => Some(Action::MoveDown),
            "move_left" => Some(Action::MoveLeft),
            "move_right" => Some(Action::MoveRight),
            "page_up" => Some(Action::PageUp),
            "page_down" => Some(Action::PageDown),

            "move_up_and_select" => Some(Action::MoveUpAndSelect),
            "move_down_and_select" => Some(Action::MoveDownAndSelect),
            "move_left_and_select" => Some(Action::MoveLeftAndSelect),
            "move_right_and_select" => Some(Action::MoveRightAndSelect),

            "yank_selection" => Some(Action::YankSelection),
            "delete_selection" => Some(Action::DeleteSelection),
            "delete_selection_and_past" => Some(Action::DeleteSelectionAndPaste),

            "paste" => Some(Action::Paste),

            "insert_line_below" => Some(Action::InsertLineBelow),
            "insert_line_above" => Some(Action::InsertLineAbove),

            "delete_backward" => Some(Action::DeleteBackward),
            "delete_forward" => Some(Action::DeleteForward),

            _ => None,
        }
    }
}
