#![allow(dead_code)]

mod app;
mod data;
mod directory_entry;
mod types;
mod utils;

use std::{env, path::PathBuf};

use crate::app::App;

fn main() -> color_eyre::Result<()> {
    let directory = match env::args().nth(1) {
        Some(p) => {
            let path = PathBuf::from(p).canonicalize().unwrap_or_else(|_| {
                eprintln!("Invalid path provided.");
                std::process::exit(1);
            });

            if path.is_dir() {
                path
            } else {
                eprintln!("The path provided needs to be a directory");
                std::process::exit(1);
            }
        }
        None => env::current_dir().expect("Failed to get current directory."),
    };

    color_eyre::install()?;

    let terminal = ratatui::init();
    let (app, tx) = App::new(directory.to_string_lossy().as_ref())?;
    let result = app.run(terminal, &tx);

    ratatui::restore();

    Ok(result?)
}
