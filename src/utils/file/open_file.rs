use std::{
    path::Path,
    process::{Command, Stdio},
};

use crate::utils::file::FileError;

use super::FileResult;

pub fn open_file(path: &Path) -> FileResult<()> {
    #[cfg(target_os = "macos")]
    let mut cmd = Command::new("open");

    #[cfg(target_os = "linux")]
    let mut cmd = Command::new("xdg-open");

    #[cfg(target_os = "windows")]
    let mut cmd = Command::new("cmd");

    #[cfg(target_os = "windows")]
    {
        cmd.args(["/C", "start", "", path]);
    }

    #[cfg(not(target_os = "windows"))]
    {
        cmd.arg(path);
    }

    cmd.stdout(Stdio::null())
        .stderr(Stdio::null())
        .stdin(Stdio::null())
        .spawn()?;

    Ok(())
}

pub fn open_file_with_app(app: &str, file_path: &str) -> FileResult<()> {
    let path = Path::new(file_path);
    if !path.exists() {
        return Err(FileError::FileDoesNotExists);
    }

    #[cfg(target_os = "linux")]
    {
        use std::fs;

        let desktop_file = if app.ends_with(".desktop") {
            app.to_string()
        } else {
            format!("{app}.desktop")
        };

        let home = std::env::var("HOME")?;

        let contents = fs::read_to_string(format!("/usr/share/applications/{desktop_file}"))
            .or_else(|_| {
                fs::read_to_string(format!("{home}/.local/share/applications/{desktop_file}",))
            })?;

        let exec_line = contents
            .lines()
            .find(|line| line.trim_start().starts_with("Exec="))
            .ok_or(FileError::NoExecLine)?;

        let exec_cmd = exec_line
            .trim_start_matches("Exec=")
            .split_whitespace()
            .filter(|part| !part.starts_with('%'))
            .collect::<Vec<_>>();

        if exec_cmd.is_empty() {
            return Err(FileError::NoExecutableFound);
        }

        Command::new(exec_cmd[0])
            .args(&exec_cmd[1..])
            .arg(file_path)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .stdin(Stdio::null())
            .spawn()?;
    }

    #[cfg(target_os = "macos")]
    {
        let status = Command::new("open")
            .arg("-a")
            .arg(app)
            .arg(file_path)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .stdin(Stdio::null())
            .spawn()?;

        if !status.success() {
            return Err(FileError::FailedToOpenFile);
        }
    }

    #[cfg(target_os = "windows")]
    {
        Command::new(app)
            .arg(file_path)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .stdin(Stdio::null())
            .spawn()?;
    }

    Ok(())
}
