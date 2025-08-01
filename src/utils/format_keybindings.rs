use crossterm::event::{KeyCode, KeyModifiers};

use crate::app::Precommand;

pub fn format_keybindings(
    bindings: &[(KeyModifiers, KeyCode)],
    precommand: Option<&Precommand>,
) -> String {
    bindings
        .iter()
        .map(|(modifiers, code)| {
            let mut parts: Vec<String> = Vec::new();

            if modifiers.contains(KeyModifiers::CONTROL) {
                parts.push("Ctrl".into());
            }
            if modifiers.contains(KeyModifiers::ALT) {
                parts.push("Alt".into());
            }
            if modifiers.contains(KeyModifiers::SHIFT) {
                parts.push("Shift".into());
            }

            parts.push(match code {
                KeyCode::Backspace => "Backspace".into(),
                KeyCode::Enter => "Enter".into(),
                KeyCode::Left => "←".into(),
                KeyCode::Right => "→".into(),
                KeyCode::Up => "↑".into(),
                KeyCode::Down => "↓".into(),
                KeyCode::Home => "Home".into(),
                KeyCode::End => "End".into(),
                KeyCode::PageUp => "PageUp".into(),
                KeyCode::PageDown => "PageDown".into(),
                KeyCode::Tab => "Tab".into(),
                KeyCode::BackTab => "BackTab".into(),
                KeyCode::Delete => "Del".into(),
                KeyCode::Insert => "Ins".into(),
                KeyCode::F(n) => format!("F{n}"),
                KeyCode::Char(c) => c.to_string(),
                KeyCode::Null => "Null".into(),
                KeyCode::Esc => "Esc".into(),
                _ => format!("{code:?}"),
            });

            if let Some(precommand) = precommand {
                match precommand {
                    Precommand::Repeat(_) => {
                        return format!("{{n}}{}", parts.join("+"));
                    }
                    Precommand::RepeatWindow(_) => {
                        return format!("{{n}}Cnrtl+w {}", parts.join("+"));
                    }
                    Precommand::Leader => return format!("<leader> {}", parts.join("+")),
                }
            }

            parts.join("+")
        })
        .collect::<Vec<_>>()
        .join(", ")
}
