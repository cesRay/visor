use std::path::PathBuf;

use super::types::Results;
use crate::search::types::{FileMatches, MatchInfo, SearchKind, SearchOptions, SearchRequest};

fn hit(path: &str, lines: &[u64]) -> FileMatches {
    FileMatches {
        path: PathBuf::from(path),
        relative_path: path.trim_start_matches('/').to_string(),
        matches: lines
            .iter()
            .map(|&line_number| MatchInfo {
                line_number,
                match_ranges: Vec::new(),
            })
            .collect(),
        name_ranges: Vec::new(),
    }
}

fn results(hits: Vec<FileMatches>) -> Results {
    let request = SearchRequest {
        kind: SearchKind::Content,
        query: "x".into(),
        base: PathBuf::from("/"),
        options: SearchOptions::default(),
    };
    Results::new(request, hits)
}

#[test]
fn opens_at_first_match_and_scrolls_preview_near_it() {
    let r = results(vec![hit("/p/a.rs", &[20, 30])]);
    assert_eq!(r.open_target(), Some((PathBuf::from("/p/a.rs"), Some(20))));
    assert_eq!(r.preview_scroll, 15);
    assert_eq!(r.total_matches(), 2);
}

#[test]
fn filename_hit_opens_at_top() {
    let r = results(vec![hit("/p/a.rs", &[])]);
    assert_eq!(r.open_target(), Some((PathBuf::from("/p/a.rs"), None)));
    assert_eq!(r.preview_scroll, 0);
}

#[test]
fn exit_target_is_the_files_directory() {
    let r = results(vec![hit("/p/src/a.rs", &[1])]);
    assert_eq!(r.exit_target(), Some(PathBuf::from("/p/src")));
}

#[test]
fn moving_resets_preview_scroll_and_stays_in_bounds() {
    let mut r = results(vec![hit("/a", &[10]), hit("/b", &[40])]);
    r.scroll_preview_down();
    r.move_down();
    assert_eq!(r.selected, 1);
    assert_eq!(r.preview_scroll, 35);
    r.move_down();
    assert_eq!(r.selected, 1);
    r.move_top();
    assert_eq!(r.selected, 0);
}
