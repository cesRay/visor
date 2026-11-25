use std::path::PathBuf;

use crate::navigator::types::Entry;

/// What the preview column shows for a path
#[derive(Debug, Clone)]
pub enum Preview {
    Text(String),
    Dir(Vec<Entry>),
    /// A short notice such as "[Binary file]"
    Message(String),
}

/// The last loaded preview, so it is not re-read on every frame
pub struct PreviewCache {
    pub path: PathBuf,
    pub show_hidden: bool,
    pub preview: Preview,
}
