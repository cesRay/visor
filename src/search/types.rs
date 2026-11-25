use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Receiver;

/// A single match occurrence within a file
#[derive(Debug, Clone)]
pub struct MatchInfo {
    /// 1-based line number
    pub line_number: u64,
    /// Byte offset ranges within the line where the match occurs
    pub match_ranges: Vec<(usize, usize)>,
}

/// All matches within a single file
#[derive(Debug, Clone)]
pub struct FileMatches {
    /// Absolute path to the file
    pub path: PathBuf,
    /// Path relative to the search base folder (for display)
    pub relative_path: String,
    /// All matches in this file, sorted by line number. Empty for filename search.
    pub matches: Vec<MatchInfo>,
    /// Byte ranges within relative_path that matched a filename search
    pub name_ranges: Vec<(usize, usize)>,
}

/// Options that control search behavior
#[derive(Debug, Clone, Default)]
pub struct SearchOptions {
    /// When true, search is case-sensitive
    pub case_sensitive: bool,
    /// When true, matches must be whole words
    pub exact_match: bool,
    /// When set, only search files with this extension
    pub extension_filter: Option<String>,
}

/// Which search a request runs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchKind {
    /// `:match`: search file contents
    Content,
    /// `:file`: search file names
    Filename,
}

/// Everything needed to run (or re-run) one search
#[derive(Debug, Clone)]
pub struct SearchRequest {
    pub kind: SearchKind,
    pub query: String,
    /// Directory searched recursively
    pub base: PathBuf,
    pub options: SearchOptions,
}

pub type SearchOutcome = Result<Vec<FileMatches>, SearchError>;

/// A search running on a background thread. Dropping it cancels the search.
pub struct SearchHandle {
    pub request: SearchRequest,
    pub receiver: Receiver<SearchOutcome>,
    pub(super) cancel: Arc<AtomicBool>,
}

impl Drop for SearchHandle {
    fn drop(&mut self) {
        self.cancel.store(true, Ordering::Relaxed);
    }
}

/// Errors that a search can return
#[derive(Debug)]
pub enum SearchError {
    Regex(grep_regex::Error),
}

impl std::fmt::Display for SearchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SearchError::Regex(e) => {
                // Regex errors span several lines with a caret diagram; the
                // last line holds the reason, which fits the status bar
                let text = e.to_string();
                let reason = text
                    .lines()
                    .rfind(|l| !l.trim().is_empty())
                    .unwrap_or(&text);
                write!(
                    f,
                    "Invalid pattern: {}",
                    reason.trim().trim_start_matches("error: ")
                )
            }
        }
    }
}

impl std::error::Error for SearchError {}

impl From<grep_regex::Error> for SearchError {
    fn from(e: grep_regex::Error) -> Self {
        SearchError::Regex(e)
    }
}
