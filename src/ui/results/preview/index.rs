use std::collections::HashMap;

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};

use super::list::highlighted;
use crate::preview::types::Preview;
use crate::results::types::Results;
use crate::search::types::{FileMatches, MatchInfo};
use crate::ui::{rows, theme};

pub fn render(frame: &mut Frame, area: Rect, results: &mut Results) {
    results.load_preview();
    let Some(hit) = results.selected_hit() else {
        let empty = Line::from(Span::styled("  No file selected", theme::NO_RESULTS));
        frame.render_widget(
            Paragraph::new(empty).block(block(" Preview ".to_string())),
            area,
        );
        return;
    };

    let block = block(format!(" {} ", hit.relative_path));
    let height = block.inner(area).height as usize;
    let content = match results.preview() {
        Some(Preview::Text(text)) => {
            preview_lines(text, hit, results.preview_scroll as usize, height)
        }
        Some(Preview::Message(text)) => vec![Line::from(Span::styled(
            format!("  {text}"),
            theme::NO_RESULTS,
        ))],
        Some(Preview::Dir(_)) | None => Vec::new(),
    };

    frame.render_widget(Paragraph::new(content).block(block), area);
}

fn block(title: String) -> Block<'static> {
    Block::default()
        .borders(Borders::ALL)
        .title(title)
        .title_style(theme::PREVIEW_TITLE)
        .border_style(theme::BORDER)
}

/// The `height` numbered lines starting at `scroll`, matches highlighted.
/// Only the visible window is built, since this runs on every frame.
fn preview_lines(
    text: &str,
    hit: &FileMatches,
    scroll: usize,
    height: usize,
) -> Vec<Line<'static>> {
    let match_map: HashMap<u64, &MatchInfo> =
        hit.matches.iter().map(|m| (m.line_number, m)).collect();

    text.lines()
        .enumerate()
        .skip(scroll)
        .take(height)
        .map(|(idx, line_text)| {
            let line_num = (idx + 1) as u64;
            let ranges = match_map
                .get(&line_num)
                .map_or(&[][..], |m| &m.match_ranges[..]);
            let (line_text, ranges) = rows::expand_tabs(line_text, ranges);
            let mut spans = vec![Span::styled(format!("{line_num:>4} "), theme::LINE_NUMBER)];
            spans.extend(highlighted(&line_text, &ranges, Style::default()));
            Line::from(spans)
        })
        .collect()
}
