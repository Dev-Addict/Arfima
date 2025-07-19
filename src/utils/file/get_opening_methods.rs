use std::path::{Path, PathBuf};
use std::process::Command;

#[cfg(target_os = "macos")]
use std::collections::HashSet;
#[cfg(any(target_os = "windows", target_os = "macos"))]
use std::ffi::OsStr;

use super::{FileError, FileResult};

pub fn get_opening_methods(path: &PathBuf) -> FileResult<Vec<String>> {
    let path = Path::new(path);
    if !path.exists() {
        return Err(FileError::FileDoesNotExists);
    }

    #[cfg(target_os = "linux")]
    {
        let mime_type = {
            let output = Command::new("xdg-mime")
                .arg("query")
                .arg("filetype")
                .arg(path)
                .output()?;
            if !output.status.success() {
                return Err(FileError::FailedToGetMimeType);
            }
            String::from_utf8_lossy(&output.stdout).trim().to_string()
        };

        let output = Command::new("gio").arg("mime").arg(&mime_type).output()?;
        if !output.status.success() {
            return Err(FileError::FailedToGetAppsWithMimeType);
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let apps = stdout
            .lines()
            .filter(|line| line.contains(".desktop"))
            .flat_map(|line| line.split_whitespace())
            .filter(|word| word.ends_with(".desktop"))
            .map(|s| s.to_string())
            .collect();

        return Ok(apps);
    }

    #[cfg(target_os = "macos")]
    {
        let output = Command::new("mdls")
            .arg("-name")
            .arg("kMDItemContentType")
            .arg(path)
            .output()?;
        let stdout = String::from_utf8_lossy(&output.stdout);
        let uti = stdout
            .lines()
            .find(|line| line.contains("kMDItemContentType"))
            .and_then(|line| line.split('=').nth(1))
            .map(|s| s.trim().trim_matches('"'))
            .ok_or(FileError::CouldNotDetermineUTI)?;

        let output = Command::new("mdfind")
            .arg(&format!("kMDItemContentTypeTree == '{}'", uti))
            .output()?;

        let files = String::from_utf8_lossy(&output.stdout);
        let apps: HashSet<String> = files
            .lines()
            .filter(|line| line.contains("/Applications/"))
            .map(|line| {
                Path::new(line)
                    .file_name()
                    .unwrap_or_else(|| OsStr::new(""))
                    .to_string_lossy()
                    .to_string()
            })
            .collect();

        return Ok(apps.into_iter().collect());
    }

    #[cfg(target_os = "windows")]
    {
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .ok_or(FileError::NoFileExtension)?;
        let assoc = Command::new("cmd")
            .args(["/C", &format!("assoc .{}", ext)])
            .output()?;
        let assoc_output = String::from_utf8_lossy(&assoc.stdout);
        let filetype = assoc_output
            .split('=')
            .nth(1)
            .ok_or(FileError::NoFileExtension)?
            .trim();

        let ftype = Command::new("cmd")
            .args(["/C", &format!("ftype {}", filetype)])
            .output()?;
        let ftype_output = String::from_utf8_lossy(&ftype.stdout);

        let apps: Vec<String> = ftype_output
            .split('=')
            .nth(1)
            .unwrap_or("")
            .split_whitespace()
            .filter(|s| s.ends_with(".exe"))
            .map(|s| s.trim_matches('"').to_string())
            .collect();

        return Ok(apps);
    }

    #[allow(unreachable_code)]
    Err(FileError::UnsupportedPlatform)
}
