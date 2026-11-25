#[path = "constants.rs"]
mod constants;
#[cfg(test)]
#[path = "tests.rs"]
mod tests;
#[path = "types.rs"]
pub mod types;

use std::fs;
use std::io;
use std::path::Path;

use constants::{FISH_INIT, POSIX_INIT, SETUP_HINT};
use types::Shell;

/// The wrapper function printed by `visor init <shell>`
pub fn init_script(shell: Shell) -> &'static str {
    match shell {
        Shell::Zsh | Shell::Bash => POSIX_INIT,
        Shell::Fish => FISH_INIT,
    }
}

/// Hands `dir` to the shell wrapper. Without a wrapper (`cwd_file` is None)
/// the shell cannot follow, so print the directory and how to set it up.
pub fn change_dir(dir: &Path, cwd_file: Option<&Path>) -> io::Result<()> {
    match cwd_file {
        Some(file) => fs::write(file, dir.to_string_lossy().as_bytes()),
        None => {
            eprintln!("{SETUP_HINT}");
            println!("{}", dir.display());
            Ok(())
        }
    }
}
