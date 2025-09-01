use std::{
    env,
    path::{Path, PathBuf},
};

use directories::BaseDirs;

pub fn expand_tilde(path: &str) -> PathBuf {
    if let Some(stripped) = path.strip_prefix("~/") {
        if let Some(base_dirs) = BaseDirs::new() {
            return base_dirs.home_dir().join(stripped);
        }

        if let Some(home) = env::var_os("HOME").or_else(|| env::var_os("USERPROFILE")) {
            return Path::new(&home).join(stripped);
        }
    }

    PathBuf::from(path)
}
