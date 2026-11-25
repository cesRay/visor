#[path = "constants.rs"]
mod constants;
#[cfg(test)]
#[path = "tests.rs"]
mod tests;

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::Line;

use constants::TAB_WIDTH;

/// Returns a scroll offset that keeps `selected` inside a window of `height` rows
pub fn keep_visible(selected: usize, height: usize, scroll: usize) -> usize {
    if height == 0 || selected < scroll {
        selected
    } else if selected >= scroll + height {
        selected + 1 - height
    } else {
        scroll
    }
}

/// Draws one line per row. Every cell is reset to `base` first, so styles
/// from the previous frame never leak into empty space.
pub fn render(frame: &mut Frame, area: Rect, rows: Vec<Line>, base: Style) {
    let buf = frame.buffer_mut();
    for y in area.top()..area.bottom() {
        for x in area.left()..area.right() {
            let cell = &mut buf[(x, y)];
            cell.reset();
            cell.set_style(base);
        }
    }
    for (i, line) in rows.iter().take(area.height as usize).enumerate() {
        buf.set_line(area.x, area.y + i as u16, line, area.width);
    }
}

/// Replaces tabs with spaces (ratatui draws a tab with zero width) and shifts
/// the byte `ranges` so they still point at the same text
pub fn expand_tabs(text: &str, ranges: &[(usize, usize)]) -> (String, Vec<(usize, usize)>) {
    if !text.contains('\t') {
        return (text.to_string(), ranges.to_vec());
    }
    let extra = TAB_WIDTH - 1;
    let shift = |offset: usize| {
        offset
            + text.as_bytes()[..offset.min(text.len())]
                .iter()
                .filter(|&&b| b == b'\t')
                .count()
                * extra
    };
    let expanded = text.replace('\t', &" ".repeat(TAB_WIDTH));
    let ranges = ranges.iter().map(|&(s, e)| (shift(s), shift(e))).collect();
    (expanded, ranges)
}
