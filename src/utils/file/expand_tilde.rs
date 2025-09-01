use std::{
    env,
    path::{Path, PathBuf},
    sync::LazyLock,
};

use directories::BaseDirs;

static HOME: LazyLock<Option<PathBuf>> = LazyLock::new(|| {
    if let Some(base_dirs) = BaseDirs::new() {
        return Some(base_dirs.home_dir().to_path_buf());
    }

    if let Some(home) = env::var_os("HOME").or_else(|| env::var_os("USERPROFILE")) {
        return Some(Path::new(&home).to_path_buf());
    }

    None
});

pub fn expand_tilde(path: &str) -> PathBuf {
    if let Some(stripped) = path.strip_prefix("~/") {
        if let Some(home) = &*HOME {
            return home.join(stripped);
        }
    }

    PathBuf::from(path)
}
