mod centered_rect;
mod entries_table;
mod help_table;
mod instructions;
pub mod modals;
mod title;
pub mod types;

pub use entries_table::draw_entries_table;
pub use help_table::get_help_table;
pub use instructions::add_instructions_to_block;
pub use title::add_title_to_block;
