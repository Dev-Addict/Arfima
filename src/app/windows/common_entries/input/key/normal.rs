use std::sync::mpsc::Sender;

use crossterm::event::{KeyCode, KeyEvent};

use crate::app::{AppEvent, InputMode, precommand::Precommand, windows::CommonEntriesWindow};

pub fn handle(
    window: &mut CommonEntriesWindow,
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
            (_, KeyCode::Right | KeyCode::Char('l') | KeyCode::Enter) => {
                if let Some(entity) = window.entries.get(window.selected_index) {
                    let _ = event_tx.send(AppEvent::Open {
                        path: entity.path().to_owned(),
                        new: false,
                    });
                }
            }
            (_, KeyCode::Char('o')) => {
                if let Some(entity) = window.entries.get(window.selected_index) {
                    let _ = event_tx.send(AppEvent::Open {
                        path: entity.path().to_owned(),
                        new: true,
                    });
                }
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
