use std::fs;

use super::load;
use super::types::Preview;

#[test]
fn text_file_is_text() {
    let tmp = tempfile::tempdir().unwrap();
    let file = tmp.path().join("a.txt");
    fs::write(&file, "hello").unwrap();
    assert!(matches!(load(&file, false), Preview::Text(t) if t == "hello"));
}

#[test]
fn nul_byte_marks_binary() {
    let tmp = tempfile::tempdir().unwrap();
    let file = tmp.path().join("a.bin");
    fs::write(&file, [b'a', 0, b'b']).unwrap();
    assert!(matches!(load(&file, false), Preview::Message(m) if m == "[Binary file]"));
}

#[test]
fn directory_lists_children_or_says_empty() {
    let tmp = tempfile::tempdir().unwrap();
    assert!(matches!(load(tmp.path(), false), Preview::Message(m) if m == "[Empty directory]"));
    fs::write(tmp.path().join("x"), "").unwrap();
    assert!(matches!(load(tmp.path(), false), Preview::Dir(e) if e.len() == 1));
}
