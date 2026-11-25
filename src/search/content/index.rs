use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};

use grep_searcher::sinks::Lossy;
use grep_searcher::{BinaryDetection, SearcherBuilder};
use ignore::WalkState;

use super::types::{FileMatches, MatchInfo, SearchError, SearchRequest};
use super::{matcher, passes_extension_filter, relative_path, walker};

/// Finds every line under `request.base` that matches `request.query`
pub fn search(
    request: &SearchRequest,
    cancel: &AtomicBool,
) -> Result<Vec<FileMatches>, SearchError> {
    let matcher = matcher::build(&request.query, &request.options)?;
    let ext_filter = request.options.extension_filter.as_deref();
    let base = request.base.as_path();
    let all_results: Mutex<Vec<FileMatches>> = Mutex::new(Vec::new());

    walker(base).run(|| {
        // Each thread gets its own searcher (they're not Send)
        let mut searcher = SearcherBuilder::new()
            .line_number(true)
            .binary_detection(BinaryDetection::quit(b'\x00'))
            .build();
        let matcher = matcher.clone();
        let all_results = &all_results;

        Box::new(move |entry| {
            if cancel.load(Ordering::Relaxed) {
                return WalkState::Quit;
            }
            let Ok(entry) = entry else {
                return WalkState::Continue;
            };
            let path = entry.path();
            if !path.is_file() || !passes_extension_filter(path, ext_filter) {
                return WalkState::Continue;
            }

            let mut file_matches: Vec<MatchInfo> = Vec::new();
            let result = searcher.search_path(
                &matcher,
                path,
                // Lossy: a line with invalid UTF-8 must not drop the file's other matches
                Lossy(|line_number, line_content| {
                    file_matches.push(MatchInfo {
                        line_number,
                        match_ranges: matcher::find_ranges(&matcher, line_content),
                    });
                    Ok(true)
                }),
            );

            if result.is_ok() && !file_matches.is_empty() {
                all_results.lock().unwrap().push(FileMatches {
                    path: path.to_path_buf(),
                    relative_path: relative_path(path, base),
                    matches: file_matches,
                    name_ranges: Vec::new(),
                });
            }
            WalkState::Continue
        })
    });

    Ok(all_results.into_inner().unwrap())
}
