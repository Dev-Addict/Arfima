pub mod file;
mod format_keybindings;
mod get_icon_and_color;
pub mod process_command;
pub mod str;

pub use format_keybindings::format_keybindings;
pub use get_icon_and_color::get_icon_and_color;
pub use process_command::parse_command;
