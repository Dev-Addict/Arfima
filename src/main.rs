#![allow(dead_code)]

mod app;
mod config;
mod data;
mod directory_entry;
mod types;
mod utils;

use std::{env, path::PathBuf};

use directories::ProjectDirs;

use crate::{
    app::App,
    config::Config,
    data::{CONFIG_FILE_NAME, package},
};

// TODO: Switch to crossbeam for channels

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

    let config_file = if let Some(proj_dirs) = ProjectDirs::from(
        package::get().tld(),
        package::get().domain(),
        package::get().application(),
    ) {
        proj_dirs.config_dir().to_owned().join(CONFIG_FILE_NAME)
    } else {
        eprintln!("Failed to get project directories");
        std::process::exit(1);
    };

    color_eyre::install()?;

    let config = Config::try_from(config_file).unwrap_or_else(|_e| {
        // TODO: Add logging for the error
        Config::default()
    });

    let terminal = ratatui::init();
    let (app, tx) = App::new(directory.to_string_lossy().as_ref(), config)?;
    let result = app.run(terminal, &tx);

    ratatui::restore();

    Ok(result?)
}
