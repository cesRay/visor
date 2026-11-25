use std::path::PathBuf;

use crate::preview::types::Preview;
use crate::search::types::{FileMatches, SearchRequest};

/// State of the results view shown after `:match` or `:file`
pub struct Results {
    /// The search that produced these hits, kept so `c` and `e` can re-run it
    pub request: SearchRequest,
    pub hits: Vec<FileMatches>,
    pub selected: usize,
    /// First visible row of the results list
    pub list_scroll: usize,
    pub preview_scroll: u16,
    pub(super) preview: Option<(PathBuf, Preview)>,
}
