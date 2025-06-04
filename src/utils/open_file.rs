use std::{
    io,
    path::Path,
    process::{Command, ExitStatus},
};

pub fn open_file(path: &Path) -> io::Result<ExitStatus> {
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

    cmd.status()
}
