use std::ffi::OsString;
use std::fs;

use super::find_in;

#[cfg(unix)]
#[test]
fn finds_executable_in_later_path_entry() {
    use std::os::unix::fs::PermissionsExt;

    let empty = tempfile::tempdir().unwrap();
    let bin_dir = tempfile::tempdir().unwrap();
    let vim = bin_dir.path().join("vim");
    fs::write(&vim, "#!/bin/sh\n").unwrap();
    fs::set_permissions(&vim, fs::Permissions::from_mode(0o755)).unwrap();

    let path_var = std::env::join_paths([empty.path(), bin_dir.path()]).unwrap();
    assert_eq!(find_in("vim", &path_var), Some(vim));
}

#[cfg(unix)]
#[test]
fn ignores_non_executable_files() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("vim"), "").unwrap();
    let path_var = OsString::from(dir.path());
    assert_eq!(find_in("vim", &path_var), None);
}
