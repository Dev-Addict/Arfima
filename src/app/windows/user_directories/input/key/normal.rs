use std::sync::mpsc::Sender;

use crossterm::event::{KeyCode, KeyEvent};

use crate::app::{AppEvent, InputMode, precommand::Precommand, windows::UserDirectoriesWindow};

pub fn handle(
    window: &mut UserDirectoriesWindow,
    input_mode: &InputMode,
    key: &KeyEvent,
    event_tx: &Sender<AppEvent>,
) -> bool {
    if let InputMode::Normal { precommand } = input_mode {
        match (key.modifiers, key.code) {
            (_, KeyCode::Down | KeyCode::Char('j')) => {
                let mut count = 1;
                if let Some(Precommand::Repeat(repeat)) = precommand {
                    count = *repeat;
                }

                let _ = event_tx.send(AppEvent::UpdatePrecommand(None));

                window.selected_index = window
                    .selected_index
                    .saturating_add(count)
                    .min(window.entries.len().saturating_sub(1));
            }
            (_, KeyCode::Up | KeyCode::Char('k')) => {
                let mut count = 1;
                if let Some(Precommand::Repeat(repeat)) = precommand {
                    count = *repeat;
                }

                let _ = event_tx.send(AppEvent::UpdatePrecommand(None));

                window.selected_index = window.selected_index.saturating_sub(count);
            }
            (_, KeyCode::Left | KeyCode::Char('h') | KeyCode::Backspace | KeyCode::Char('o')) => {
                // TODO: Open in a new window or existing window the current directory or if it is
                // a file open the file
                //
                // let mut count = 0;
                // if let Some(Precommand::Repeat(repeat)) = precommand {
                //     if *repeat == 0 {
                //         return true;
                //     }
                //
                //     count = repeat.saturating_sub(1);
                // }
                //
                // let mut target_directory: &Path;
                //
                // if let Some(parent) = Path::new(&window.directory).parent() {
                //     target_directory = parent;
                // } else {
                //     return true;
                // }
                //
                // while let Some(parent) = Path::new(target_directory).parent() {
                //     if count == 0 {
                //         break;
                //     }
                //
                //     target_directory = parent;
                //     count = count.saturating_sub(1);
                // }
                //
                // if let Err(e) = window.set_directory(target_directory.to_string_lossy().to_string())
                // {
                //     let _ = event_tx.send(AppEvent::SetError(Some(e)));
                // }
            }
            (_, KeyCode::Home | KeyCode::Char('g')) => {
                if let Some(Precommand::Repeat(repeat)) = precommand {
                    window.selected_index = (*repeat - 1).min(window.entries.len() - 1).max(1);

                    let _ = event_tx.send(AppEvent::UpdatePrecommand(None));
                } else {
                    window.selected_index = 0;
                }
            }
            (_, KeyCode::End | KeyCode::Char('G')) => {
                window.selected_index = window.entries.len().saturating_sub(1)
            }
            _ => {
                return false;
            }
        }

        return true;
    }

    false
}
