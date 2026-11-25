use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::widgets::{Block, Borders};

use crate::ui::theme;
use crate::app::App;

pub fn render(frame: &mut Frame, area: Rect, app: &mut App) {
    // Render the border block
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Files ")
        .border_style(theme::BORDER_FOCUSED);
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let selected = app.file_list_state.selected().unwrap_or(0);
    let inner_height = inner.height as usize;

    // Compute scroll offset to keep selected item visible
    let scroll = if inner_height == 0 {
        0
    } else if selected < app.file_list_scroll {
        selected
    } else if selected >= app.file_list_scroll + inner_height {
        selected - inner_height + 1
    } else {
        app.file_list_scroll
    };
    app.file_list_scroll = scroll;

    let buf = frame.buffer_mut();

    for row in 0..inner_height {
        let file_idx = scroll + row;
        let y = inner.y + row as u16;

        // Determine text and style for this row
        let (text, style) = if file_idx < app.results.len() {
            let fm = &app.results[file_idx];
            let label = format!("{} ({})", fm.relative_path, fm.matches.len());
            if file_idx == selected {
                (format!("> {label}"), theme::SELECTED_FILE)
            } else {
                (format!("  {label}"), theme::FILE_LIST_BASE)
            }
        } else {
            (String::new(), theme::FILE_LIST_BASE)
        };

        // Write each cell explicitly
        let chars: Vec<char> = text.chars().collect();
        for col in 0..inner.width {
            let x = inner.x + col;
            let cell = &mut buf[(x, y)];
            cell.reset();
            cell.set_style(style);
            if (col as usize) < chars.len() {
                cell.set_char(chars[col as usize]);
            } else {
                cell.set_char(' ');
            }
        }
    }

    // Show message if no results
    if app.results.is_empty() {
        let msg = "  No matches found";
        let y = inner.y;
        for (col, ch) in msg.chars().enumerate() {
            if col as u16 >= inner.width {
                break;
            }
            let cell = &mut buf[(inner.x + col as u16, y)];
            cell.reset();
            cell.set_char(ch);
            cell.set_style(Style::reset().fg(ratatui::style::Color::DarkGray));
        }
    }
}
