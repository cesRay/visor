use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use super::theme;
use crate::app::App;

pub fn render_header(frame: &mut Frame, area: Rect, app: &App) {
    let total_matches: usize = app.results.iter().map(|f| f.matches.len()).sum();

    let spans = vec![
        Span::styled("  lens: ", theme::STATUS_TEXT),
        Span::styled(format!("\"{}\"", app.query), theme::HEADER_QUERY),
        Span::styled("  |  ", theme::STATUS_TEXT),
        Span::styled(
            format!("{total_matches} matches in {} files", app.results.len()),
            theme::MATCH_COUNT,
        ),
        Span::styled("  |  ", theme::STATUS_TEXT),
        Span::styled("case:", theme::STATUS_TEXT),
        if app.search_options.case_sensitive {
            Span::styled("ON", theme::TOGGLE_ON)
        } else {
            Span::styled("OFF", theme::TOGGLE_OFF)
        },
        Span::styled("  exact:", theme::STATUS_TEXT),
        if app.search_options.exact_match {
            Span::styled("ON", theme::TOGGLE_ON)
        } else {
            Span::styled("OFF", theme::TOGGLE_OFF)
        },
    ];

    let header = Paragraph::new(Line::from(spans));
    frame.render_widget(header, area);
}

pub fn render_footer(frame: &mut Frame, area: Rect) {
    let spans = vec![
        Span::styled("  [c]", theme::KEY_LABEL),
        Span::styled("ase  ", theme::STATUS_TEXT),
        Span::styled("[e]", theme::KEY_LABEL),
        Span::styled("xact  ", theme::STATUS_TEXT),
        Span::styled("[q]", theme::KEY_LABEL),
        Span::styled("uit  ", theme::STATUS_TEXT),
        Span::styled("[j/k]", theme::KEY_LABEL),
        Span::styled("navigate  ", theme::STATUS_TEXT),
        Span::styled("[Enter]", theme::KEY_LABEL),
        Span::styled("open  ", theme::STATUS_TEXT),
        Span::styled("[PgUp/PgDn]", theme::KEY_LABEL),
        Span::styled("scroll", theme::STATUS_TEXT),
    ];

    let footer = Paragraph::new(Line::from(spans));
    frame.render_widget(footer, area);
}
