#[path = "constants.rs"]
mod constants;
#[cfg(test)]
#[path = "tests.rs"]
mod tests;
#[path = "types.rs"]
pub mod types;

use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use constants::SHOW_HIDDEN_DEFAULT;
use types::{Entry, EntryKind, Navigator};

/// Lists `dir`, directories first, each group sorted case-insensitively
pub fn read_dir(dir: &Path, show_hidden: bool) -> io::Result<Vec<Entry>> {
    let mut entries: Vec<Entry> = fs::read_dir(dir)?
        .filter_map(Result::ok)
        .filter_map(|e| {
            let name = e.file_name().to_string_lossy().to_string();
            if !show_hidden && name.starts_with('.') {
                return None;
            }
            let path = e.path();
            let is_symlink = e.file_type().map(|t| t.is_symlink()).unwrap_or(false);
            // fs::metadata follows symlinks, so a link to a directory counts as a directory
            let kind = match fs::metadata(&path) {
                Ok(m) if m.is_dir() => EntryKind::Dir,
                Ok(_) => EntryKind::File,
                Err(_) if is_symlink => EntryKind::BrokenLink,
                Err(_) => EntryKind::File,
            };
            Some(Entry {
                name,
                path,
                kind,
                is_symlink,
            })
        })
        .collect();

    entries.sort_by(|a, b| {
        b.is_dir()
            .cmp(&a.is_dir())
            .then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
    });
    Ok(entries)
}

/// Human-readable reason for an IO error, without the OS error code
pub fn describe_error(err: &io::Error) -> String {
    match err.kind() {
        io::ErrorKind::PermissionDenied => "permission denied".to_string(),
        io::ErrorKind::NotFound => "not found".to_string(),
        _ => err.to_string(),
    }
}

impl Navigator {
    pub fn new(start: PathBuf) -> Self {
        let mut nav = Self {
            cwd: start,
            entries: Vec::new(),
            selected: 0,
            scroll: 0,
            parent_entries: Vec::new(),
            show_hidden: SHOW_HIDDEN_DEFAULT,
            error: None,
            history: HashMap::new(),
        };
        nav.refresh();
        nav
    }

    pub fn selected_entry(&self) -> Option<&Entry> {
        self.entries.get(self.selected)
    }

    /// Re-reads the current and parent directories. If the current directory
    /// no longer exists, moves up to the nearest parent that does.
    pub fn refresh(&mut self) {
        while !self.cwd.is_dir() {
            match self.cwd.parent() {
                Some(p) => self.cwd = p.to_path_buf(),
                None => break,
            }
        }

        let keep = self.selected_entry().map(|e| e.name.clone());
        match read_dir(&self.cwd, self.show_hidden) {
            Ok(entries) => {
                self.entries = entries;
                self.error = None;
            }
            Err(e) => {
                self.entries = Vec::new();
                self.error = Some(describe_error(&e));
            }
        }

        let wanted = keep.or_else(|| self.history.get(&self.cwd).cloned());
        self.selected = wanted
            .and_then(|name| self.entries.iter().position(|e| e.name == name))
            .unwrap_or(0)
            .min(self.entries.len().saturating_sub(1));

        self.parent_entries = self
            .cwd
            .parent()
            .and_then(|p| read_dir(p, self.show_hidden).ok())
            .unwrap_or_default();
    }

    /// Index of `cwd` inside `parent_entries`, for highlighting the left column
    pub fn parent_selected(&self) -> Option<usize> {
        let name = self.cwd.file_name()?.to_string_lossy();
        self.parent_entries.iter().position(|e| e.name == name)
    }

    pub fn move_up(&mut self) {
        self.selected = self.selected.saturating_sub(1);
    }

    pub fn move_down(&mut self) {
        if self.selected + 1 < self.entries.len() {
            self.selected += 1;
        }
    }

    pub fn move_top(&mut self) {
        self.selected = 0;
    }

    pub fn move_bottom(&mut self) {
        self.selected = self.entries.len().saturating_sub(1);
    }

    /// Goes into the selected directory. Returns an error message when the
    /// selection is not a directory or cannot be read.
    pub fn enter(&mut self) -> Result<(), String> {
        let Some(entry) = self.selected_entry().cloned() else {
            return Err("Nothing selected".to_string());
        };
        if !entry.is_dir() {
            return Err(format!("{} is not a directory", entry.name));
        }
        if let Err(e) = fs::read_dir(&entry.path) {
            return Err(format!(
                "Cannot open {}: {}",
                entry.name,
                describe_error(&e)
            ));
        }
        self.remember();
        self.change_dir(entry.path);
        Ok(())
    }

    /// Goes to the parent directory and selects the directory we came from
    pub fn parent(&mut self) {
        let Some(parent) = self.cwd.parent().map(Path::to_path_buf) else {
            return;
        };
        self.remember();
        let came_from = self
            .cwd
            .file_name()
            .map(|n| n.to_string_lossy().to_string());
        if let Some(name) = came_from {
            self.history.insert(parent.clone(), name);
        }
        self.change_dir(parent);
    }

    pub fn toggle_hidden(&mut self) {
        self.show_hidden = !self.show_hidden;
        self.refresh();
    }

    /// Where `q` takes the shell: the selected directory, or the current
    /// directory when a file is selected
    pub fn exit_target(&self) -> PathBuf {
        match self.selected_entry() {
            Some(e) if e.is_dir() => e.path.clone(),
            _ => self.cwd.clone(),
        }
    }

    /// Switches to `dir`. The old entries are cleared first so the selection
    /// comes from history, not from a same-named entry in the old directory.
    fn change_dir(&mut self, dir: PathBuf) {
        self.cwd = dir;
        self.entries.clear();
        self.selected = 0;
        self.scroll = 0;
        self.refresh();
    }

    fn remember(&mut self) {
        if let Some(name) = self.selected_entry().map(|e| e.name.clone()) {
            self.history.insert(self.cwd.clone(), name);
        }
    }
}
