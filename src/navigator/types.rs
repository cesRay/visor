use std::collections::HashMap;
use std::path::PathBuf;

/// What a directory entry points at
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntryKind {
    Dir,
    File,
    /// A symlink whose target does not exist
    BrokenLink,
}

/// One row in a directory listing
#[derive(Debug, Clone)]
pub struct Entry {
    pub name: String,
    pub path: PathBuf,
    pub kind: EntryKind,
    pub is_symlink: bool,
}

impl Entry {
    pub fn is_dir(&self) -> bool {
        self.kind == EntryKind::Dir
    }
}

/// State of the ranger-style navigator
pub struct Navigator {
    /// Directory shown in the middle column
    pub cwd: PathBuf,
    pub entries: Vec<Entry>,
    pub selected: usize,
    /// First visible row of the middle column
    pub scroll: usize,
    /// Listing of the parent directory, shown in the left column
    pub parent_entries: Vec<Entry>,
    pub show_hidden: bool,
    /// Set when `cwd` could not be read
    pub error: Option<String>,
    /// Last selected entry name per directory, restored when coming back
    pub(super) history: HashMap<PathBuf, String>,
}
