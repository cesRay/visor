#[path = "constants.rs"]
mod constants;
#[cfg(test)]
#[path = "tests.rs"]
mod tests;
#[path = "types.rs"]
pub mod types;

use std::fs;
use std::path::Path;

use crate::navigator::{self, types::Entry};
use constants::{BINARY_SNIFF_BYTES, MAX_PREVIEW_BYTES};
use types::{Preview, PreviewCache};

/// Loads the preview for a file or directory
pub fn load(path: &Path, show_hidden: bool) -> Preview {
    if path.is_dir() {
        return match navigator::read_dir(path, show_hidden) {
            Ok(entries) if entries.is_empty() => Preview::Message("[Empty directory]".to_string()),
            Ok(entries) => Preview::Dir(entries),
            Err(e) => Preview::Message(format!("[{}]", navigator::describe_error(&e))),
        };
    }

    let metadata = match fs::metadata(path) {
        Ok(m) => m,
        Err(_) => return Preview::Message("[Broken symlink]".to_string()),
    };
    if metadata.len() > MAX_PREVIEW_BYTES {
        return Preview::Message("[File too large to preview]".to_string());
    }

    match fs::read(path) {
        Ok(bytes) if bytes.iter().take(BINARY_SNIFF_BYTES).any(|&b| b == 0) => {
            Preview::Message("[Binary file]".to_string())
        }
        Ok(bytes) => Preview::Text(String::from_utf8_lossy(&bytes).into_owned()),
        Err(e) => Preview::Message(format!("[{}]", navigator::describe_error(&e))),
    }
}

/// Returns the cached preview for `entry`, loading it if the selection changed
pub fn cached<'a>(
    cache: &'a mut Option<PreviewCache>,
    entry: &Entry,
    show_hidden: bool,
) -> &'a Preview {
    let stale = cache
        .as_ref()
        .is_none_or(|c| c.path != entry.path || c.show_hidden != show_hidden);
    if stale {
        *cache = Some(PreviewCache {
            path: entry.path.clone(),
            show_hidden,
            preview: load(&entry.path, show_hidden),
        });
    }
    &cache.as_ref().expect("cache was just filled").preview
}
