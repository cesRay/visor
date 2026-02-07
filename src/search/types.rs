use std::path::PathBuf;

/// A single match occurrence within a file
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct MatchInfo {
    /// 1-based line number
    pub line_number: u64,
    /// The full text of the line (trailing newline stripped)
    pub line_content: String,
    /// Byte offset ranges within line_content where the match occurs
    pub match_ranges: Vec<(usize, usize)>,
}

/// All matches within a single file
#[derive(Debug, Clone)]
pub struct FileMatches {
    /// Absolute path to the file
    pub path: PathBuf,
    /// Path relative to the search base folder (for display)
    pub relative_path: String,
    /// All matches in this file, sorted by line number
    pub matches: Vec<MatchInfo>,
}

/// Options that control search behavior
#[derive(Debug, Clone)]
pub struct SearchOptions {
    /// When true, search is case-sensitive
    pub case_sensitive: bool,
    /// When true, matches must be whole words
    pub exact_match: bool,
    /// When set, only search files with this extension
    pub extension_filter: Option<String>,
}

impl Default for SearchOptions {
    fn default() -> Self {
        Self {
            case_sensitive: false,
            exact_match: false,
            extension_filter: None,
        }
    }
}
