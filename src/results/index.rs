#[path = "constants.rs"]
mod constants;
#[cfg(test)]
#[path = "tests.rs"]
mod tests;
#[path = "types.rs"]
pub mod types;

use std::path::PathBuf;

use crate::preview::{self, types::Preview};
use crate::search::types::{FileMatches, SearchRequest};
use constants::{PREVIEW_CONTEXT_LINES, PREVIEW_PAGE_LINES};
use types::Results;

impl Results {
    pub fn new(request: SearchRequest, hits: Vec<FileMatches>) -> Self {
        let mut results = Self {
            request,
            hits,
            selected: 0,
            list_scroll: 0,
            preview_scroll: 0,
            preview: None,
        };
        results.reset_preview_scroll();
        results
    }

    pub fn selected_hit(&self) -> Option<&FileMatches> {
        self.hits.get(self.selected)
    }

    pub fn total_matches(&self) -> usize {
        self.hits.iter().map(|h| h.matches.len()).sum()
    }

    pub fn move_up(&mut self) {
        self.select(self.selected.saturating_sub(1));
    }

    pub fn move_down(&mut self) {
        if self.selected + 1 < self.hits.len() {
            self.select(self.selected + 1);
        }
    }

    pub fn move_top(&mut self) {
        self.select(0);
    }

    pub fn move_bottom(&mut self) {
        self.select(self.hits.len().saturating_sub(1));
    }

    pub fn scroll_preview_up(&mut self) {
        self.preview_scroll = self.preview_scroll.saturating_sub(PREVIEW_PAGE_LINES);
    }

    pub fn scroll_preview_down(&mut self) {
        self.preview_scroll = self.preview_scroll.saturating_add(PREVIEW_PAGE_LINES);
    }

    /// File and line vim should open: the first match for `:match`, the top for `:file`
    pub fn open_target(&self) -> Option<(PathBuf, Option<u64>)> {
        let hit = self.selected_hit()?;
        Some((hit.path.clone(), hit.matches.first().map(|m| m.line_number)))
    }

    /// Where `q` takes the shell: the directory that holds the selected file
    pub fn exit_target(&self) -> Option<PathBuf> {
        self.selected_hit()?.path.parent().map(PathBuf::from)
    }

    /// Loads the preview of the selected file, once per selection
    pub fn load_preview(&mut self) {
        let Some(path) = self.selected_hit().map(|h| h.path.clone()) else {
            return;
        };
        if self.preview.as_ref().is_none_or(|(p, _)| *p != path) {
            let loaded = preview::load(&path, false);
            self.preview = Some((path, loaded));
        }
    }

    /// The preview loaded by `load_preview`, if it belongs to the selection
    pub fn preview(&self) -> Option<&Preview> {
        let hit = self.selected_hit()?;
        self.preview
            .as_ref()
            .filter(|(p, _)| *p == hit.path)
            .map(|(_, p)| p)
    }

    /// Forgets the loaded preview so it is read again, e.g. after editing in vim
    pub fn reload_preview(&mut self) {
        self.preview = None;
    }

    fn select(&mut self, index: usize) {
        self.selected = index;
        self.reset_preview_scroll();
    }

    fn reset_preview_scroll(&mut self) {
        self.preview_scroll = self
            .selected_hit()
            .and_then(|h| h.matches.first())
            .map(|m| {
                m.line_number
                    .saturating_sub(PREVIEW_CONTEXT_LINES)
                    .min(u16::MAX as u64) as u16
            })
            .unwrap_or(0);
    }
}
