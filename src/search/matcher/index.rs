use grep_matcher::Matcher;
use grep_regex::{RegexMatcher, RegexMatcherBuilder};

use super::types::{SearchError, SearchOptions};

/// Builds the regex matcher shared by content and filename search
pub fn build(query: &str, options: &SearchOptions) -> Result<RegexMatcher, SearchError> {
    let mut builder = RegexMatcherBuilder::new();
    builder.case_insensitive(!options.case_sensitive);

    if options.exact_match {
        builder.word(true);
    }

    Ok(builder.build(query)?)
}

/// Byte ranges of every match of `matcher` within `text`
pub fn find_ranges(matcher: &impl Matcher, text: &str) -> Vec<(usize, usize)> {
    let mut ranges = Vec::new();
    let _ = matcher.find_iter(text.as_bytes(), |m| {
        ranges.push((m.start(), m.end()));
        true
    });
    ranges
}
