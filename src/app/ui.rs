use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
};

use crate::app::widgets::render_command;

use super::{
    App, InputMode,
    widgets::{
        modals::{show_help_modal, show_input_modal, show_opening_modal, show_yes_no_modal},
        render_error, render_instructions, render_precommand,
    },
};

pub fn render_ui(app: &mut App, frame: &mut Frame) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Fill(1), Constraint::Length(1)])
        .split(frame.area());

    app.window.render(app, frame, layout[0], true);

    if let InputMode::Normal {
        precommand: Some(precommand),
    } = &app.input_mode
    {
        render_precommand(frame, layout[1], precommand);
    } else if let InputMode::Commanding { state, .. } = &app.input_mode {
        render_command(frame, layout[1], state);
    } else if let Some(e) = &app.error {
        render_error(frame, layout[1], e);
    } else {
        render_instructions(frame, layout[1]);
        frame.render_widget(format!("{}", app.config.history().size()), layout[1]);
    }

    match &app.input_mode {
        InputMode::Adding { state } => {
            show_input_modal("Add directory/file", frame, state);
        }
        InputMode::Renaming { state, .. } => {
            show_input_modal("Rename directory/file", frame, state);
        }
        InputMode::Removing {
            removing_selected, ..
        } => {
            show_yes_no_modal(
                "Are you sure you want to delete directory/file?",
                frame,
                *removing_selected,
            );
        }
        InputMode::Opening {
            apps,
            path,
            selected_index,
        } => {
            show_opening_modal(path, apps, frame, *selected_index);
        }
        InputMode::Help { selected_index } => {
            show_help_modal(frame, *selected_index);
        }
        _ => {}
    }
}
