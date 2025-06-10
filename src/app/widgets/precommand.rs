use ratatui::widgets::Block;

use crate::app::precommand::Precommand;

pub fn add_precommand_to_block<'a>(block: Block<'a>, precommand: &'a Precommand) -> Block<'a> {
    block.title_bottom(match precommand {
        Precommand::Leader => " <leader> ".to_string(),
        Precommand::Repeat(repeat) => format!(" {} ", repeat),
    })
}
