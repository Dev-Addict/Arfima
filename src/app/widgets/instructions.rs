use ratatui::{style::Stylize, text::Line, widgets::Block};

pub fn add_instructions_to_block(block: Block) -> Block {
    block.title_bottom(Line::from(vec![
        " Up ".into(),
        "<K>".blue().bold(),
        " Down ".into(),
        "<J>".blue().bold(),
        " Back ".into(),
        "<H>".blue().bold(),
        " Down ".into(),
        "<J>".blue().bold(),
        " Help ".into(),
        "<Ctrl+H>".blue().bold(),
        " Quit ".into(),
        "<Q> ".blue().bold(),
    ]))
}
