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
use crate::ui::theme;

pub fn render(frame: &mut Frame, area: Rect, results: &mut Results) {
    let scroll = results.preview_scroll;
    let Some(hit) = results.selected_hit().cloned() else {
        let empty = Line::from(Span::styled("  No file selected", theme::NO_RESULTS));
        frame.render_widget(
            Paragraph::new(empty).block(block(" Preview ".to_string())),
            area,
        );
        return;
    };

    let content = match results.preview() {
        Some(Preview::Text(text)) => preview_lines(text, &hit),
        Some(Preview::Message(text)) => vec![Line::from(Span::styled(
            format!("  {text}"),
            theme::NO_RESULTS,
        ))],
        Some(Preview::Dir(_)) | None => Vec::new(),
    };

    let paragraph = Paragraph::new(content)
        .block(block(format!(" {} ", hit.relative_path)))
        .scroll((scroll, 0));
    frame.render_widget(paragraph, area);
}

fn block(title: String) -> Block<'static> {
    Block::default()
        .borders(Borders::ALL)
        .title(title)
        .title_style(theme::PREVIEW_TITLE)
        .border_style(theme::BORDER)
}

/// Numbered lines with the matched parts highlighted
fn preview_lines(text: &str, hit: &FileMatches) -> Vec<Line<'static>> {
    let match_map: HashMap<u64, &MatchInfo> =
        hit.matches.iter().map(|m| (m.line_number, m)).collect();

    text.lines()
        .enumerate()
        .map(|(idx, line_text)| {
            let line_num = (idx + 1) as u64;
            let mut spans = vec![Span::styled(format!("{line_num:>4} "), theme::LINE_NUMBER)];
            match match_map.get(&line_num) {
                Some(m) => spans.extend(highlighted(line_text, &m.match_ranges, Style::default())),
                None => spans.push(Span::raw(line_text.to_string())),
            }
            Line::from(spans)
        })
        .collect()
}
