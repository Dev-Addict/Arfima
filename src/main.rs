#![allow(dead_code)]

mod app;
mod command;
mod config;
mod data;
mod directory_entry;
mod types;
mod utils;

use std::{env, path::PathBuf};

use directories::ProjectDirs;
use log::{error, info};
use stderrlog::LogLevelNum;

use crate::{
    app::App,
    config::Config,
    data::{CONFIG_FILE_NAME, package},
};

fn main() -> color_eyre::Result<()> {
    stderrlog::new()
        .module(module_path!())
        .quiet(false)
        .verbosity(LogLevelNum::Info)
        .init()
        .unwrap();

    let directory = match env::args().nth(1) {
        Some(p) => {
            let path = PathBuf::from(p).canonicalize().unwrap_or_else(|_| {
                error!("Invalid path provided.");
                std::process::exit(1);
            });

            if path.is_dir() {
                path
            } else {
                error!("The path provided needs to be a directory");
                std::process::exit(1);
            }
        }
        None => env::current_dir().unwrap_or_else(|_| {
            error!("Failed to get current directory.");
            std::process::exit(1);
        }),
    };

    let config_file = match ProjectDirs::from(
        package::get().tld(),
        package::get().domain(),
        package::get().application(),
    ) {
        Some(proj_dirs) => proj_dirs.config_dir().to_owned().join(CONFIG_FILE_NAME),
        None => {
            error!(
                "Failed to get project directories. Please report this issue in GitHub: https://github.com/Dev-Addict/Arfima"
            );
            std::process::exit(1);
        }
    };

    color_eyre::install()?;

    let config = Config::try_from(config_file.clone()).unwrap_or_else(|e| {
        error!(
            "Failed to get config from file({}): {e}",
            config_file.to_string_lossy()
        );
        info!("Using the default config.");
        Config::default()
    });

    let terminal = ratatui::init();
    let (app, tx) = App::new(directory.to_string_lossy().as_ref(), config)?;
    let result = app.run(terminal, &tx);

    ratatui::restore();

    Ok(result?)
}
