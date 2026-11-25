#[path = "constants.rs"]
mod constants;

use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders};

use crate::app::App;
use crate::navigator::types::{Entry, EntryKind};
use crate::preview::{self, types::Preview};
use crate::ui::{rows, theme};
use constants::NARROW_WIDTH;

/// Three ranger-style columns: parent | current directory | preview
pub fn render(frame: &mut Frame, area: Rect, app: &mut App) {
    let (parent_area, current_area, preview_area) = if area.width < NARROW_WIDTH {
        let [current, preview] =
            Layout::horizontal([Constraint::Percentage(45), Constraint::Percentage(55)])
                .areas(area);
        (None, current, preview)
    } else {
        let [parent, current, preview] = Layout::horizontal([
            Constraint::Percentage(20),
            Constraint::Percentage(30),
            Constraint::Percentage(50),
        ])
        .areas(area);
        (Some(parent), current, preview)
    };

    if let Some(parent_area) = parent_area {
        render_parent(frame, parent_area, app);
    }
    render_current(frame, current_area, app);
    render_preview(frame, preview_area, app);
}

fn render_parent(frame: &mut Frame, area: Rect, app: &App) {
    let nav = &app.navigator;
    let title = nav
        .cwd
        .parent()
        .and_then(|p| p.file_name())
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "/".to_string());
    let inner = column(frame, area, &title, theme::BORDER);

    // The parent column has no stored scroll; keep its selection centred
    let selected = nav.parent_selected();
    let scroll = selected
        .map(|s| s.saturating_sub(inner.height as usize / 2))
        .unwrap_or(0);
    let lines = entry_lines(&nav.parent_entries, selected, scroll, inner.height);
    rows::render(frame, inner, lines, theme::FILE_LIST_BASE);
}

fn render_current(frame: &mut Frame, area: Rect, app: &mut App) {
    let nav = &mut app.navigator;
    let title = nav
        .cwd
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "/".to_string());
    let inner = column(frame, area, &title, theme::BORDER_FOCUSED);

    if let Some(error) = &nav.error {
        rows::render(
            frame,
            inner,
            vec![notice(&format!("[{error}]"))],
            theme::FILE_LIST_BASE,
        );
        return;
    }
    if nav.entries.is_empty() {
        rows::render(
            frame,
            inner,
            vec![notice("[Empty directory]")],
            theme::FILE_LIST_BASE,
        );
        return;
    }

    nav.scroll = rows::keep_visible(nav.selected, inner.height as usize, nav.scroll);
    let lines = entry_lines(&nav.entries, Some(nav.selected), nav.scroll, inner.height);
    rows::render(frame, inner, lines, theme::FILE_LIST_BASE);
}

fn render_preview(frame: &mut Frame, area: Rect, app: &mut App) {
    let Some(entry) = app.navigator.selected_entry().cloned() else {
        let inner = column(frame, area, "Preview", theme::BORDER);
        rows::render(frame, inner, Vec::new(), theme::FILE_LIST_BASE);
        return;
    };
    let inner = column(frame, area, &entry.name, theme::BORDER);
    let preview = preview::cached(&mut app.preview_cache, &entry, app.navigator.show_hidden);

    let lines = match preview {
        Preview::Dir(entries) => entry_lines(entries, None, 0, inner.height),
        Preview::Message(text) => vec![notice(text)],
        Preview::Text(text) => text
            .lines()
            .take(inner.height as usize)
            .enumerate()
            .map(|(i, line)| {
                Line::from(vec![
                    Span::styled(format!("{:>4} ", i + 1), theme::LINE_NUMBER),
                    Span::raw(rows::expand_tabs(line, &[]).0),
                ])
            })
            .collect(),
    };
    rows::render(frame, inner, lines, Style::reset());
}

/// Draws a bordered column with a title and returns the area inside it
fn column(frame: &mut Frame, area: Rect, title: &str, border: Style) -> Rect {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(format!(" {title} "))
        .title_style(theme::PREVIEW_TITLE)
        .border_style(border);
    let inner = block.inner(area);
    frame.render_widget(block, area);
    inner
}

/// One styled line per visible entry, starting at `scroll`
fn entry_lines(
    entries: &[Entry],
    selected: Option<usize>,
    scroll: usize,
    height: u16,
) -> Vec<Line<'static>> {
    entries
        .iter()
        .enumerate()
        .skip(scroll)
        .take(height as usize)
        .map(|(i, entry)| {
            let is_selected = selected == Some(i);
            let mut label = entry.name.clone();
            if entry.is_dir() {
                label.push('/');
            }
            if entry.is_symlink {
                label.push_str(" @");
            }
            let style = if is_selected {
                theme::SELECTED_FILE
            } else {
                match entry.kind {
                    EntryKind::Dir => theme::DIRECTORY,
                    EntryKind::File => theme::FILE_LIST_BASE,
                    EntryKind::BrokenLink => theme::BROKEN_LINK,
                }
            };
            let prefix = if is_selected { "> " } else { "  " };
            Line::from(Span::styled(format!("{prefix}{label}"), style))
        })
        .collect()
}

fn notice(text: &str) -> Line<'static> {
    Line::from(Span::styled(format!("  {text}"), theme::NO_RESULTS))
}
