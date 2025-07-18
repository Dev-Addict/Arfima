#[allow(clippy::module_inception)]
mod app;
mod app_event;
mod error;
mod input;
mod input_mode;
mod precommand;
mod result;
mod ui;
pub mod widgets;
mod window;
mod windows;

pub use app::App;
pub use app_event::AppEvent;
pub use error::Error;
pub use input_mode::InputMode;
pub use precommand::Precommand;
pub use result::Result;
