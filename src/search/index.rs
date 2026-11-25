#[path = "content/index.rs"]
mod content;
#[path = "filename/index.rs"]
mod filename;
#[path = "matcher/index.rs"]
mod matcher;
#[cfg(test)]
#[path = "tests.rs"]
mod tests;
#[path = "types.rs"]
pub mod types;

use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::mpsc;
use std::thread;

use ignore::{WalkBuilder, WalkParallel};

use types::{SearchHandle, SearchKind, SearchOutcome, SearchRequest};

/// Starts `request` on a background thread. Poll the handle's receiver for
/// the outcome; drop the handle to cancel.
pub fn spawn(request: SearchRequest) -> SearchHandle {
    let cancel = Arc::new(AtomicBool::new(false));
    let (tx, receiver) = mpsc::channel();
    let worker_request = request.clone();
    let worker_cancel = Arc::clone(&cancel);

    thread::spawn(move || {
        let _ = tx.send(run(&worker_request, &worker_cancel));
    });

    SearchHandle {
        request,
        receiver,
        cancel,
    }
}

/// Runs `request` on the current thread. Content results are sorted by
/// path; filename results come best match first.
pub fn run(request: &SearchRequest, cancel: &AtomicBool) -> SearchOutcome {
    match request.kind {
        SearchKind::Content => {
            let mut results = content::search(request, cancel)?;
            results.sort_by(|a, b| a.relative_path.cmp(&b.relative_path));
            Ok(results)
        }
        SearchKind::Filename => Ok(filename::search(request, cancel)),
    }
}

/// Parallel walker that skips hidden files and respects .gitignore
fn walker(base: &Path) -> WalkParallel {
    WalkBuilder::new(base)
        .hidden(true)
        .git_ignore(true)
        .threads(
            thread::available_parallelism()
                .map(|n| n.get())
                .unwrap_or(4),
        )
        .build_parallel()
}

fn passes_extension_filter(path: &Path, filter: Option<&str>) -> bool {
    match filter {
        None => true,
        Some(ext) => path.extension().and_then(|e| e.to_str()) == Some(ext),
    }
}

fn relative_path(path: &Path, base: &Path) -> String {
    path.strip_prefix(base)
        .unwrap_or(path)
        .to_string_lossy()
        .to_string()
}
