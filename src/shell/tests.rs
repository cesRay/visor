use std::fs;

use super::types::Shell;
use super::{change_dir, init_script};

#[test]
fn every_init_script_passes_cwd_file_and_cds() {
    for shell in [Shell::Zsh, Shell::Bash, Shell::Fish] {
        let script = init_script(shell);
        assert!(script.contains("command visor --cwd-file"), "{shell:?}");
        assert!(script.contains("builtin cd --"), "{shell:?}");
    }
}

#[test]
fn change_dir_writes_the_path_to_the_cwd_file() {
    let tmp = tempfile::tempdir().unwrap();
    let file = tmp.path().join("cwd");
    change_dir(tmp.path(), Some(&file)).unwrap();
    assert_eq!(
        fs::read_to_string(file).unwrap(),
        tmp.path().to_string_lossy()
    );
}
