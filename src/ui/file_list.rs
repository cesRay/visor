use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::text::Span;
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};

use super::theme;
use crate::app::App;

pub fn render(frame: &mut Frame, area: Rect, app: &mut App) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Files ")
        .border_style(theme::BORDER_FOCUSED);

    if app.results.is_empty() {
        let msg = Paragraph::new(Span::styled("  No matches found", theme::NO_RESULTS))
            .block(block);
        frame.render_widget(msg, area);
        return;
    }

    let items: Vec<ListItem> = app
        .results
        .iter()
        .map(|fm| {
            let label = format!("{} ({})", fm.relative_path, fm.matches.len());
            ListItem::new(label)
        })
        .collect();

    let list = List::new(items)
        .block(block)
        .highlight_symbol("> ")
        .highlight_style(theme::SELECTED_FILE)
        .style(theme::FILE_LIST_BASE);

    frame.render_stateful_widget(list, area, &mut app.file_list_state);
}
