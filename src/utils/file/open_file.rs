use std::{
    io,
    path::Path,
    process::{Command, Stdio},
};

pub fn open_file(path: &Path) -> io::Result<()> {
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
