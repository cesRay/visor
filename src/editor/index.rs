#[path = "constants.rs"]
pub mod constants;
#[cfg(test)]
#[path = "tests.rs"]
mod tests;

use std::ffi::OsStr;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;

use constants::EDITOR_BIN;

/// Finds vim on the current PATH
pub fn find() -> Option<PathBuf> {
    find_in(EDITOR_BIN, &std::env::var_os("PATH")?)
}

/// Finds an executable called `bin` in a PATH-style list of directories
pub fn find_in(bin: &str, path_var: &OsStr) -> Option<PathBuf> {
    std::env::split_paths(path_var)
        .map(|dir| dir.join(bin))
        .find(|candidate| is_executable(candidate))
}

/// Opens `path` in vim, at `line` when given. Blocks until vim exits.
/// vim's exit status is ignored; only a failure to start it is an error.
pub fn open(path: &Path, line: Option<u64>) -> io::Result<()> {
    let mut cmd = Command::new(EDITOR_BIN);
    if let Some(line) = line {
        cmd.arg(format!("+{line}"));
    }
    cmd.arg(path).status().map(|_| ())
}

#[cfg(unix)]
fn is_executable(path: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;
    path.metadata()
        .map(|m| m.is_file() && m.permissions().mode() & 0o111 != 0)
        .unwrap_or(false)
}

#[cfg(not(unix))]
fn is_executable(path: &Path) -> bool {
    path.is_file()
}
