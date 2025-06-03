use std::{
    fs, io,
    path::{Path, PathBuf},
    time::SystemTime,
};

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    style::Stylize,
    text::Line,
    widgets::{Block, Paragraph},
};

pub enum DirectoryEntryType {
    File {
        extension: Option<String>,
        size: u64,
    },
    Directory,
    Other,
}

pub struct DirectoryEntry {
    name: String,
    path: PathBuf,
    modified: Option<SystemTime>,
    entry_type: DirectoryEntryType,
}

#[derive(Debug, Default)]
pub struct App {
    running: bool,
    directory: String,
}

impl App {
    pub fn new(directory: String) -> Self {
        Self {
            running: false,
            directory,
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|frame| {
                let _ = self.render(frame);
            })?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame) -> io::Result<()> {
        let title = Line::from("Arfima").bold().blue().centered();

        let instructions = Line::from(vec![
            " Up ".into(),
            "<K>".blue().bold(),
            " Down ".into(),
            "<J>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);

        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.left_aligned());

        let path = Path::new(&self.directory);

        if !path.is_dir() {
            panic!("The directory path provided is not a directory");
        }

        let mut entries: Vec<DirectoryEntry> = Vec::new();

        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            let metadata = entry.metadata()?;

            let file_name = entry.file_name().to_string_lossy().to_string();

            if metadata.is_file() {
                let extension = path
                    .extension()
                    .map(|ext| ext.to_string_lossy().to_string());

                entries.push(DirectoryEntry {
                    name: file_name,
                    path,
                    modified: metadata.modified().ok(),
                    entry_type: DirectoryEntryType::File {
                        size: metadata.len(),
                        extension,
                    },
                });
            } else if metadata.is_dir() {
                entries.push(DirectoryEntry {
                    name: file_name,
                    path,
                    modified: metadata.modified().ok(),
                    entry_type: DirectoryEntryType::Directory,
                })
            } else {
                entries.push(DirectoryEntry {
                    name: file_name,
                    path,
                    modified: metadata.modified().ok(),
                    entry_type: DirectoryEntryType::Other,
                })
            }
        }

        frame.render_widget(
            Paragraph::new(
                entries
                    .iter()
                    .map(|entry| entry.name.to_string())
                    .collect::<Vec<_>>()
                    .join("\n"),
            )
            .block(block)
            .centered(),
            frame.area(),
        );

        Ok(())
    }

    fn handle_crossterm_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            _ => {}
        }
        Ok(())
    }

    fn on_key_event(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            _ => {}
        }
    }

    fn quit(&mut self) {
        self.running = false;
    }
}
