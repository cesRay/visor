use std::fs;

use super::App;
use super::types::View;
use crate::input::types::Action;
use crate::results::types::Results;
use crate::search::types::{SearchKind, SearchOptions};

#[test]
fn leaving_results_cancels_a_running_search() {
    let tmp = tempfile::tempdir().unwrap();
    fs::write(tmp.path().join("a.txt"), "todo").unwrap();
    let mut app = App::new(tmp.path().to_path_buf(), SearchOptions::default());
    app.start_search(SearchKind::Content, "todo".into());
    let request = app.search.as_ref().unwrap().request.clone();
    app.results = Some(Results::new(request, Vec::new()));
    app.view = View::Results;

    app.handle(Action::Cancel);
    assert_eq!(app.view, View::Navigator);
    assert!(app.search.is_none());
}
