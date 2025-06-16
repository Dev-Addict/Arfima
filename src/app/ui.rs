use ratatui::Frame;

use super::{
    App, InputMode,
    widgets::modals::{show_help_modal, show_input_modal, show_yes_no_modal},
};

pub fn render_ui(app: &mut App, frame: &mut Frame) {
    match &app.input_mode {
        InputMode::Adding { state } => {
            show_input_modal("Add directory/file", frame, state);
        }
        InputMode::Renaming { state, .. } => {
            show_input_modal("Rename directory/file", frame, state);
        }
        InputMode::Removing { .. } => {
            show_yes_no_modal(
                "Are you sure you want to delete directory/file?",
                frame,
                app.removing_selected,
            );
        }
        InputMode::Help { selected_index } => {
            show_help_modal(frame, *selected_index);
        }
        _ => {}
    }
}
