use ratatui::{Frame, widgets::Block};

use super::{
    App, InputMode,
    widgets::{
        add_error_to_block, add_instructions_to_block, add_title_to_block, draw_entries_table,
        modals::{show_help_modal, show_input_modal, show_yes_no_modal},
    },
};

pub fn render_ui(app: &mut App, frame: &mut Frame) {
    let mut block = Block::bordered();
    block = add_title_to_block(app, block);

    if let Some(e) = &app.error {
        block = add_error_to_block(block, e);
    } else {
        block = add_instructions_to_block(block);
    }

    draw_entries_table(frame, app, block);

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
