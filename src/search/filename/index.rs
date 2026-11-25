use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};

use ignore::WalkState;

use super::types::{FileMatches, SearchError, SearchRequest};
use super::{matcher, passes_extension_filter, relative_path, walker};

/// Finds every file under `request.base` whose name matches `request.query`.
/// Only the file name is matched, not the directories above it.
pub fn search(
    request: &SearchRequest,
    cancel: &AtomicBool,
) -> Result<Vec<FileMatches>, SearchError> {
    let matcher = matcher::build(&request.query, &request.options)?;
    let ext_filter = request.options.extension_filter.as_deref();
    let base = request.base.as_path();
    let all_results: Mutex<Vec<FileMatches>> = Mutex::new(Vec::new());

    walker(base).run(|| {
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
            let Some(name) = path.file_name().map(|n| n.to_string_lossy()) else {
                return WalkState::Continue;
            };

            let ranges = matcher::find_ranges(&matcher, &name);
            if !ranges.is_empty() {
                let relative_path = relative_path(path, base);
                // Shift the ranges so they point into relative_path, which ends with the name
                let offset = relative_path.len() - name.len();
                all_results.lock().unwrap().push(FileMatches {
                    path: path.to_path_buf(),
                    name_ranges: ranges
                        .iter()
                        .map(|&(s, e)| (s + offset, e + offset))
                        .collect(),
                    relative_path,
                    matches: Vec::new(),
                });
            }
            WalkState::Continue
        })
    });

    Ok(all_results.into_inner().unwrap())
}
