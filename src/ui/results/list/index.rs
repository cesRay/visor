use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders};

use crate::results::types::Results;
use crate::search::types::FileMatches;
use crate::ui::{rows, theme};

pub fn render(frame: &mut Frame, area: Rect, results: &mut Results) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Files ")
        .border_style(theme::BORDER_FOCUSED);
    let inner = block.inner(area);
    frame.render_widget(block, area);

    if results.hits.is_empty() {
        let msg = Line::from(Span::styled("  No matches found", theme::NO_RESULTS));
        rows::render(frame, inner, vec![msg], theme::FILE_LIST_BASE);
        return;
    }

    results.list_scroll =
        rows::keep_visible(results.selected, inner.height as usize, results.list_scroll);
    let lines = results
        .hits
        .iter()
        .enumerate()
        .skip(results.list_scroll)
        .take(inner.height as usize)
        .map(|(i, hit)| hit_line(hit, i == results.selected))
        .collect();
    rows::render(frame, inner, lines, theme::FILE_LIST_BASE);
}

/// `path (count)` for content hits; the path with the matched part
/// highlighted for filename hits
fn hit_line(hit: &FileMatches, selected: bool) -> Line<'static> {
    let (prefix, style) = if selected {
        ("> ", theme::SELECTED_FILE)
    } else {
        ("  ", theme::FILE_LIST_BASE)
    };
    let mut spans = vec![Span::styled(prefix, style)];
    spans.extend(highlighted(&hit.relative_path, &hit.path_ranges, style));
    if !hit.matches.is_empty() {
        spans.push(Span::styled(format!(" ({})", hit.matches.len()), style));
    }
    Line::from(spans)
}

/// Splits `text` into spans, styling the byte `ranges` as matches
pub fn highlighted(text: &str, ranges: &[(usize, usize)], base: Style) -> Vec<Span<'static>> {
    let mut spans = Vec::new();
    let mut last = 0;
    for &(start, end) in ranges {
        let start = start.min(text.len()).max(last);
        let end = end.min(text.len());
        if start >= end || !text.is_char_boundary(start) || !text.is_char_boundary(end) {
            continue;
        }
        if start > last {
            spans.push(Span::styled(text[last..start].to_string(), base));
        }
        spans.push(Span::styled(
            text[start..end].to_string(),
            theme::MATCH_HIGHLIGHT,
        ));
        last = end;
    }
    if last < text.len() {
        spans.push(Span::styled(text[last..].to_string(), base));
    }
    spans
}
