use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::Line;

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
