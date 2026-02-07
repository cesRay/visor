use std::path::Path;
use std::sync::Mutex;

use grep_matcher::Matcher;
use grep_regex::RegexMatcherBuilder;
use grep_searcher::sinks::UTF8;
use grep_searcher::SearcherBuilder;
use ignore::WalkBuilder;

use super::types::{FileMatches, MatchInfo, SearchOptions};

#[derive(Debug)]
pub enum SearchError {
    Regex(grep_regex::Error),
}

impl std::fmt::Display for SearchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SearchError::Regex(e) => write!(f, "Regex error: {e}"),
        }
    }
}

impl std::error::Error for SearchError {}

impl From<grep_regex::Error> for SearchError {
    fn from(e: grep_regex::Error) -> Self {
        SearchError::Regex(e)
    }
}

pub fn search(
    base_path: &Path,
    query: &str,
    options: &SearchOptions,
) -> Result<Vec<FileMatches>, SearchError> {
    let mut builder = RegexMatcherBuilder::new();
    builder.case_insensitive(!options.case_sensitive);

    if options.exact_match {
        builder.word(true);
    }

    let matcher = builder.build(query)?;
    let ext_filter = options.extension_filter.clone();
    let base = base_path.to_path_buf();

    let all_results: Mutex<Vec<FileMatches>> = Mutex::new(Vec::new());

    let walker = WalkBuilder::new(base_path)
        .hidden(true)
        .git_ignore(true)
        .threads(std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4))
        .build_parallel();

    walker.run(|| {
        // Each thread gets its own searcher (they're not Send)
        let mut searcher = SearcherBuilder::new().line_number(true).build();
        let matcher = matcher.clone();
        let ext_filter = ext_filter.clone();
        let base = base.clone();
        let all_results = &all_results;

        Box::new(move |entry| {
            let entry = match entry {
                Ok(e) => e,
                Err(_) => return ignore::WalkState::Continue,
            };

            let path = entry.path();

            if !path.is_file() {
                return ignore::WalkState::Continue;
            }

            // Apply extension filter
            if let Some(ref ext_filter) = ext_filter {
                match path.extension().and_then(|e| e.to_str()) {
                    Some(ext) if ext == ext_filter.as_str() => {}
                    _ => return ignore::WalkState::Continue,
                }
            }

            let mut file_matches: Vec<MatchInfo> = Vec::new();

            let result = searcher.search_path(
                &matcher,
                path,
                UTF8(|line_number, line_content| {
                    let match_ranges = find_match_ranges(&matcher, line_content);
                    file_matches.push(MatchInfo {
                        line_number,
                        line_content: line_content.trim_end_matches('\n').to_string(),
                        match_ranges,
                    });
                    Ok(true)
                }),
            );

            if result.is_err() {
                return ignore::WalkState::Continue;
            }

            if !file_matches.is_empty() {
                let fm = FileMatches {
                    path: path.to_path_buf(),
                    relative_path: path
                        .strip_prefix(&base)
                        .unwrap_or(path)
                        .to_string_lossy()
                        .to_string(),
                    matches: file_matches,
                };
                all_results.lock().unwrap().push(fm);
            }

            ignore::WalkState::Continue
        })
    });

    let mut results = all_results.into_inner().unwrap();
    results.sort_by(|a, b| a.relative_path.cmp(&b.relative_path));
    Ok(results)
}

fn find_match_ranges(matcher: &impl Matcher, line: &str) -> Vec<(usize, usize)> {
    let mut ranges = Vec::new();
    let _ = matcher.find_iter(line.as_bytes(), |m| {
        ranges.push((m.start(), m.end()));
        true
    });
    ranges
}
