use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};

use super::theme;
use crate::app::App;

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let selected = app.selected_file();

    let (title, content) = match selected {
        Some(fm) => {
            let title = format!(" {} ", fm.relative_path);
            let file_content = app
                .preview_cache
                .as_ref()
                .map(|(_, c)| c.as_str())
                .unwrap_or("");

            let lines = build_preview_lines(file_content, fm);
            (title, lines)
        }
        None => {
            let lines = vec![Line::from(Span::styled(
                "  No file selected",
                theme::NO_RESULTS,
            ))];
            (" Preview ".to_string(), lines)
        }
    };

    let paragraph = Paragraph::new(content)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title)
                .title_style(theme::PREVIEW_TITLE)
                .border_style(theme::BORDER),
        )
        .scroll((app.preview_scroll, 0));

    frame.render_widget(paragraph, area);
}

fn build_preview_lines<'a>(
    file_content: &'a str,
    fm: &crate::search::types::FileMatches,
) -> Vec<Line<'a>> {
    use std::collections::HashMap;

    let match_map: HashMap<u64, &crate::search::types::MatchInfo> =
        fm.matches.iter().map(|m| (m.line_number, m)).collect();

    file_content
        .lines()
        .enumerate()
        .map(|(idx, line_text)| {
            let line_num = (idx + 1) as u64;
            let gutter = Span::styled(format!("{line_num:>4} "), theme::LINE_NUMBER);

            if let Some(match_info) = match_map.get(&line_num) {
                let mut spans = vec![gutter];
                let mut last_end = 0;

                for &(start, end) in &match_info.match_ranges {
                    let start = start.min(line_text.len());
                    let end = end.min(line_text.len());

                    if start > last_end {
                        spans.push(Span::raw(&line_text[last_end..start]));
                    }
                    spans.push(Span::styled(
                        &line_text[start..end],
                        theme::MATCH_HIGHLIGHT,
                    ));
                    last_end = end;
                }

                if last_end < line_text.len() {
                    spans.push(Span::raw(&line_text[last_end..]));
                }

                Line::from(spans)
            } else {
                Line::from(vec![gutter, Span::raw(line_text)])
            }
        })
        .collect()
}
