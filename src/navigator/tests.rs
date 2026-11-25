use std::fs;

use tempfile::TempDir;

use super::read_dir;
use super::types::{EntryKind, Navigator};

/// Builds: alpha/ beta/ .hidden/ b.txt A.txt alpha/inner.txt
fn tree() -> TempDir {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    fs::create_dir(root.join("beta")).unwrap();
    fs::create_dir(root.join("alpha")).unwrap();
    fs::create_dir(root.join(".hidden")).unwrap();
    fs::write(root.join("b.txt"), "b").unwrap();
    fs::write(root.join("A.txt"), "a").unwrap();
    fs::write(root.join("alpha/inner.txt"), "inner").unwrap();
    tmp
}

fn names(nav: &Navigator) -> Vec<&str> {
    nav.entries.iter().map(|e| e.name.as_str()).collect()
}

#[test]
fn lists_directories_first_then_files_case_insensitive() {
    let tmp = tree();
    let entries = read_dir(tmp.path(), false).unwrap();
    let names: Vec<_> = entries.iter().map(|e| e.name.as_str()).collect();
    assert_eq!(names, ["alpha", "beta", "A.txt", "b.txt"]);
    assert_eq!(entries[0].kind, EntryKind::Dir);
    assert_eq!(entries[2].kind, EntryKind::File);
}

#[test]
fn toggle_hidden_shows_dot_entries() {
    let tmp = tree();
    let mut nav = Navigator::new(tmp.path().to_path_buf());
    assert!(!names(&nav).contains(&".hidden"));
    nav.toggle_hidden();
    assert!(names(&nav).contains(&".hidden"));
}

#[test]
fn enter_and_parent_restore_selection() {
    let tmp = tree();
    let mut nav = Navigator::new(tmp.path().to_path_buf());
    assert_eq!(nav.selected_entry().unwrap().name, "alpha");

    nav.enter().unwrap();
    assert_eq!(nav.cwd, tmp.path().join("alpha"));
    assert_eq!(names(&nav), ["inner.txt"]);
    assert_eq!(nav.parent_selected(), Some(0));

    nav.parent();
    assert_eq!(nav.cwd, tmp.path());
    assert_eq!(nav.selected_entry().unwrap().name, "alpha");
}

#[test]
fn enter_on_file_is_an_error() {
    let tmp = tree();
    let mut nav = Navigator::new(tmp.path().to_path_buf());
    nav.move_bottom();
    assert!(nav.enter().is_err());
    assert_eq!(nav.cwd, tmp.path());
}

#[test]
fn movement_stays_in_bounds() {
    let tmp = tree();
    let mut nav = Navigator::new(tmp.path().to_path_buf());
    nav.move_up();
    assert_eq!(nav.selected, 0);
    nav.move_bottom();
    nav.move_down();
    assert_eq!(nav.selected, 3);
    nav.move_top();
    assert_eq!(nav.selected, 0);
}

#[test]
fn exit_target_is_selected_dir_or_cwd_for_files() {
    let tmp = tree();
    let mut nav = Navigator::new(tmp.path().to_path_buf());
    assert_eq!(nav.exit_target(), tmp.path().join("alpha"));
    nav.move_bottom();
    assert_eq!(nav.exit_target(), tmp.path());
}

#[test]
fn refresh_moves_up_when_cwd_is_deleted() {
    let tmp = tree();
    let mut nav = Navigator::new(tmp.path().to_path_buf());
    nav.enter().unwrap();
    fs::remove_dir_all(tmp.path().join("alpha")).unwrap();
    nav.refresh();
    assert_eq!(nav.cwd, tmp.path());
}

#[cfg(unix)]
#[test]
fn symlinks_to_dirs_are_dirs_and_broken_links_are_marked() {
    let tmp = tree();
    let root = tmp.path();
    std::os::unix::fs::symlink(root.join("alpha"), root.join("link_dir")).unwrap();
    std::os::unix::fs::symlink(root.join("missing"), root.join("link_broken")).unwrap();
    let entries = read_dir(root, false).unwrap();
    let find = |n: &str| entries.iter().find(|e| e.name == n).unwrap();
    assert_eq!(find("link_dir").kind, EntryKind::Dir);
    assert!(find("link_dir").is_symlink);
    assert_eq!(find("link_broken").kind, EntryKind::BrokenLink);
}
