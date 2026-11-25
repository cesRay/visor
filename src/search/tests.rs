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
fn file_search_matches_relative_paths_like_cmd_p() {
    let tmp = tree();
    fs::create_dir_all(tmp.path().join("components/ui/Button")).unwrap();
    fs::write(tmp.path().join("components/ui/Button/index.tsx"), "").unwrap();
    fs::write(tmp.path().join("components/ui/index.tsx"), "").unwrap();

    let found = paths(
        SearchKind::Filename,
        "Button/index.tsx",
        &tmp,
        SearchOptions::default(),
    );
    assert_eq!(
        found.first().map(String::as_str),
        Some("components/ui/Button/index.tsx")
    );

    // Fuzzy: the letters only have to appear in order
    let found = paths(
        SearchKind::Filename,
        "btnidx",
        &tmp,
        SearchOptions::default(),
    );
    assert_eq!(found, ["components/ui/Button/index.tsx"]);

    // Directory names count too
    let found = paths(
        SearchKind::Filename,
        "src main",
        &tmp,
        SearchOptions::default(),
    );
    assert_eq!(found, ["src/main.rs"]);
}

#[test]
fn file_search_ranks_best_match_first_and_highlights_it() {
    let tmp = tree();
    let req = request(SearchKind::Filename, "app", &tmp, SearchOptions::default());
    let results = run(&req, &AtomicBool::new(false)).unwrap();
    let found: Vec<_> = results.iter().map(|f| f.relative_path.as_str()).collect();
    assert_eq!(found, ["app.rs", "src/app_state.rs"]);

    let nested = &results[1];
    let highlighted: String = nested
        .path_ranges
        .iter()
        .map(|&(s, e)| &nested.relative_path[s..e])
        .collect();
    assert_eq!(highlighted, "app");
}

#[test]
fn file_search_exact_needs_contiguous_text_and_case_is_respected() {
    let tmp = tree();
    let exact = SearchOptions {
        exact_match: true,
        ..Default::default()
    };
    assert!(
        paths(SearchKind::Filename, "apst", &tmp, SearchOptions::default())
            .contains(&"src/app_state.rs".to_string())
    );
    assert!(paths(SearchKind::Filename, "apst", &tmp, exact).is_empty());

    let case = SearchOptions {
        case_sensitive: true,
        ..Default::default()
    };
    assert!(paths(SearchKind::Filename, "APP", &tmp, case).is_empty());
    assert!(!paths(SearchKind::Filename, "APP", &tmp, SearchOptions::default()).is_empty());
}

#[test]
fn file_search_highlights_non_ascii_paths_on_char_boundaries() {
    let tmp = tree();
    fs::write(tmp.path().join("café_menü.txt"), "").unwrap();
    let req = request(SearchKind::Filename, "menü", &tmp, SearchOptions::default());
    let results = run(&req, &AtomicBool::new(false)).unwrap();
    let hit = &results[0];
    let highlighted: String = hit
        .path_ranges
        .iter()
        .map(|&(s, e)| &hit.relative_path[s..e])
        .collect();
    assert_eq!(highlighted, "menü");
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

#[test]
fn invalid_utf8_line_keeps_the_files_other_matches() {
    let tmp = tree();
    fs::write(
        tmp.path().join("latin1.txt"),
        b"todo one\ncaf\xe9 todo two\n",
    )
    .unwrap();
    let req = request(SearchKind::Content, "todo", &tmp, SearchOptions::default());
    let results = run(&req, &AtomicBool::new(false)).unwrap();
    let latin1 = results
        .iter()
        .find(|f| f.relative_path == "latin1.txt")
        .unwrap();
    assert_eq!(latin1.matches.len(), 2);
}

#[test]
fn file_search_exact_finds_matches_at_the_end_of_non_ascii_paths() {
    let tmp = tree();
    fs::write(tmp.path().join("é.txt"), "").unwrap();
    let exact = SearchOptions {
        exact_match: true,
        ..Default::default()
    };
    assert_eq!(
        paths(SearchKind::Filename, "é.txt", &tmp, exact.clone()),
        ["é.txt"]
    );
    assert!(paths(SearchKind::Filename, "txt", &tmp, exact).contains(&"é.txt".to_string()));
}

#[test]
fn blank_file_query_finds_nothing() {
    let tmp = tree();
    assert!(paths(SearchKind::Filename, "   ", &tmp, SearchOptions::default()).is_empty());
}
