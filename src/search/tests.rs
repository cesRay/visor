use std::fs;
use std::sync::atomic::AtomicBool;

use tempfile::TempDir;

use super::run;
use super::types::{SearchKind, SearchOptions, SearchRequest};

/// Builds: app.rs, src/app_state.rs, src/main.rs, notes.md
fn tree() -> TempDir {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    fs::create_dir(root.join("src")).unwrap();
    fs::write(root.join("app.rs"), "fn App() {}\n// todo later\n").unwrap();
    fs::write(root.join("src/app_state.rs"), "struct State;\n").unwrap();
    fs::write(root.join("src/main.rs"), "fn main() { TODO }\n").unwrap();
    fs::write(root.join("notes.md"), "TODO: write docs\n").unwrap();
    tmp
}

fn request(kind: SearchKind, query: &str, base: &TempDir, options: SearchOptions) -> SearchRequest {
    SearchRequest {
        kind,
        query: query.to_string(),
        base: base.path().to_path_buf(),
        options,
    }
}

fn paths(kind: SearchKind, query: &str, tmp: &TempDir, options: SearchOptions) -> Vec<String> {
    run(&request(kind, query, tmp, options), &AtomicBool::new(false))
        .unwrap()
        .into_iter()
        .map(|f| f.relative_path)
        .collect()
}

#[test]
fn content_search_is_case_insensitive_by_default() {
    let tmp = tree();
    let found = paths(SearchKind::Content, "todo", &tmp, SearchOptions::default());
    assert_eq!(found, ["app.rs", "notes.md", "src/main.rs"]);
}

#[test]
fn content_search_respects_case_and_extension() {
    let tmp = tree();
    let options = SearchOptions {
        case_sensitive: true,
        extension_filter: Some("rs".into()),
        ..Default::default()
    };
    assert_eq!(
        paths(SearchKind::Content, "TODO", &tmp, options),
        ["src/main.rs"]
    );
}

#[test]
fn content_matches_carry_line_numbers_and_ranges() {
    let tmp = tree();
    let req = request(SearchKind::Content, "todo", &tmp, SearchOptions::default());
    let results = run(&req, &AtomicBool::new(false)).unwrap();
    let app = &results[0];
    assert_eq!(app.matches[0].line_number, 2);
    assert_eq!(app.matches[0].match_ranges, [(3, 7)]);
}

#[test]
fn filename_search_matches_name_only_and_offsets_ranges() {
    let tmp = tree();
    // "src" is in a directory name, not a file name, so nothing matches
    assert!(paths(SearchKind::Filename, "src", &tmp, SearchOptions::default()).is_empty());

    let req = request(SearchKind::Filename, "app", &tmp, SearchOptions::default());
    let results = run(&req, &AtomicBool::new(false)).unwrap();
    let found: Vec<_> = results.iter().map(|f| f.relative_path.as_str()).collect();
    assert_eq!(found, ["app.rs", "src/app_state.rs"]);
    let nested = &results[1];
    let (s, e) = nested.name_ranges[0];
    assert_eq!(&nested.relative_path[s..e], "app");
}

#[test]
fn filename_exact_match_needs_whole_word() {
    let tmp = tree();
    let options = SearchOptions {
        exact_match: true,
        ..Default::default()
    };
    // "app_state" is one word, so only app.rs matches
    assert_eq!(
        paths(SearchKind::Filename, "app", &tmp, options),
        ["app.rs"]
    );
}

#[test]
fn invalid_pattern_is_an_error() {
    let tmp = tree();
    let req = request(SearchKind::Content, "(", &tmp, SearchOptions::default());
    let err = run(&req, &AtomicBool::new(false)).unwrap_err();
    assert_eq!(err.to_string(), "Invalid pattern: unclosed group");
}

#[test]
fn cancelled_search_returns_nothing() {
    let tmp = tree();
    let req = request(SearchKind::Content, "todo", &tmp, SearchOptions::default());
    assert!(run(&req, &AtomicBool::new(true)).unwrap().is_empty());
}
