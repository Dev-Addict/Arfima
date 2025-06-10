use ratatui::{style::Stylize, widgets::Block};

use crate::app::Error;

pub fn add_error_to_block<'a>(block: Block<'a>, error: &'a Error) -> Block<'a> {
    block.title_bottom(format!(" {} ", error).red())
}
