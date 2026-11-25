use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};

use ignore::WalkState;
use nucleo_matcher::pattern::{AtomKind, CaseMatching, Normalization, Pattern};
use nucleo_matcher::{Config, Matcher, Utf32Str};

use super::types::{FileMatches, SearchOptions, SearchRequest};
use super::{passes_extension_filter, relative_path, walker};

/// Finds files under `request.base` whose relative path fuzzy-matches
/// `request.query`, like VS Code's Cmd+P. Words separated by spaces must all
/// match. Results come best match first.
pub fn search(request: &SearchRequest, cancel: &AtomicBool) -> Vec<FileMatches> {
    // A pattern with no words matches everything, so a blank query finds nothing
    if request.query.trim().is_empty() {
        return Vec::new();
    }
    let pattern = pattern(&request.query, &request.options);
    let exact = request.options.exact_match;
    let ext_filter = request.options.extension_filter.as_deref();
    let base = request.base.as_path();
    let scored: Mutex<Vec<(u32, FileMatches)>> = Mutex::new(Vec::new());

    walker(base).run(|| {
        // Each thread gets its own matcher; it holds scratch memory
        let mut matcher = Matcher::new(Config::DEFAULT.match_paths());
        let mut buf = Vec::new();
        let mut indices = Vec::new();
        let pattern = &pattern;
        let scored = &scored;

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

            let relative_path = relative_path(path, base);
            indices.clear();
            let haystack = haystack(&relative_path, &mut buf, exact);
            if let Some(score) = pattern.indices(haystack, &mut matcher, &mut indices) {
                let path_ranges = char_indices_to_ranges(&relative_path, &mut indices);
                scored.lock().unwrap().push((
                    score,
                    FileMatches {
                        path: path.to_path_buf(),
                        relative_path,
                        matches: Vec::new(),
                        path_ranges,
                    },
                ));
            }
            WalkState::Continue
        })
    });

    let mut scored = scored.into_inner().unwrap();
    // Best score first; on a tie, the shorter path, then A to Z
    scored.sort_by(|(sa, a), (sb, b)| {
        sb.cmp(sa)
            .then(a.relative_path.len().cmp(&b.relative_path.len()))
            .then_with(|| a.relative_path.cmp(&b.relative_path))
    });
    scored.into_iter().map(|(_, hit)| hit).collect()
}

/// `c` makes the match case-sensitive; `e` turns fuzzy matching into
/// contiguous (substring) matching
fn pattern(query: &str, options: &SearchOptions) -> Pattern {
    let case = if options.case_sensitive {
        CaseMatching::Respect
    } else {
        CaseMatching::Ignore
    };
    let kind = if options.exact_match {
        AtomKind::Substring
    } else {
        AtomKind::Fuzzy
    };
    Pattern::new(query, case, Normalization::Smart, kind)
}

/// Same as `Utf32Str::new` (one u32 per char, since the unicode-segmentation
/// feature is off), plus a workaround: nucleo-matcher 0.3.1's substring
/// search skips the last start position in non-ASCII text, so `:file txt`
/// in exact mode would miss `é.txt`. A trailing NUL, which no query
/// contains, makes that position reachable again.
fn haystack<'a>(text: &'a str, buf: &'a mut Vec<char>, exact: bool) -> Utf32Str<'a> {
    if text.is_ascii() {
        return Utf32Str::Ascii(text.as_bytes());
    }
    buf.clear();
    buf.extend(text.chars());
    if exact {
        buf.push('\0');
    }
    Utf32Str::Unicode(buf)
}

/// Turns the matcher's character positions into merged byte ranges of `text`
fn char_indices_to_ranges(text: &str, indices: &mut [u32]) -> Vec<(usize, usize)> {
    indices.sort_unstable();
    let mut wanted = indices.iter().map(|&i| i as usize).peekable();
    let mut ranges: Vec<(usize, usize)> = Vec::new();

    for (char_index, (byte, ch)) in text.char_indices().enumerate() {
        while wanted.next_if(|&w| w < char_index).is_some() {}
        if wanted.next_if_eq(&char_index).is_none() {
            continue;
        }
        let end = byte + ch.len_utf8();
        match ranges.last_mut() {
            Some(last) if last.1 == byte => last.1 = end,
            _ => ranges.push((byte, end)),
        }
    }
    ranges
}
