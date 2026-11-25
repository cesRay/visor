use std::fs;

use ratatui::Terminal;
use ratatui::backend::TestBackend;

use super::draw;
use crate::app::App;
use crate::app::types::View;
use crate::results::types::Results;
use crate::search::types::{FileMatches, SearchKind, SearchOptions, SearchRequest};

fn screen(app: &mut App, width: u16) -> String {
    let mut terminal = Terminal::new(TestBackend::new(width, 12)).unwrap();
    terminal.draw(|f| draw(f, app)).unwrap();
    let buffer = terminal.backend().buffer();
    buffer.content.iter().map(|c| c.symbol()).collect()
}

fn project() -> tempfile::TempDir {
    let tmp = tempfile::tempdir().unwrap();
    fs::create_dir_all(tmp.path().join("proj/src")).unwrap();
    fs::write(tmp.path().join("proj/readme.md"), "hello visor\n").unwrap();
    tmp
}

#[test]
fn navigator_shows_three_columns_and_hints() {
    let tmp = project();
    let mut app = App::new(tmp.path().join("proj"), SearchOptions::default());
    let text = screen(&mut app, 100);
    assert!(text.contains("> src/"), "selected directory");
    assert!(text.contains("readme.md"));
    assert!(text.contains(" proj "), "current column title");
    assert!(text.contains("[Enter]"), "key hints");
}

#[test]
fn narrow_navigator_hides_parent_column() {
    let tmp = project();
    let mut app = App::new(tmp.path().join("proj"), SearchOptions::default());
    let parent_name = tmp
        .path()
        .file_name()
        .unwrap()
        .to_string_lossy()
        .to_string();
    assert!(screen(&mut app, 100).contains(&parent_name));
    assert!(!screen(&mut app, 50).contains(&format!(" {parent_name} ")));
}

#[test]
fn command_line_is_drawn_in_footer() {
    let tmp = project();
    let mut app = App::new(tmp.path().join("proj"), SearchOptions::default());
    app.command_line = Some("match hello".into());
    assert!(screen(&mut app, 100).contains(":match hello█"));
}

#[test]
fn results_view_shows_query_and_hits() {
    let tmp = project();
    let mut app = App::new(tmp.path().join("proj"), SearchOptions::default());
    let path = tmp.path().join("proj/readme.md");
    let request = SearchRequest {
        kind: SearchKind::Filename,
        query: "read".into(),
        base: tmp.path().join("proj"),
        options: SearchOptions::default(),
    };
    let hit = FileMatches {
        path,
        relative_path: "readme.md".into(),
        matches: Vec::new(),
        path_ranges: vec![(0, 4)],
    };
    app.results = Some(Results::new(request, vec![hit]));
    app.view = View::Results;
    let text = screen(&mut app, 100);
    assert!(text.contains(":file \"read\""));
    assert!(text.contains("1 files"));
    assert!(text.contains("> readme.md"));
    assert!(text.contains("hello visor"), "preview");
}
