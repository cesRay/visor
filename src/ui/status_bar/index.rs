#[path = "constants.rs"]
mod constants;

use std::path::Path;

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use crate::app::App;
use crate::app::types::View;
use crate::search::types::SearchKind;
use crate::ui::theme;
use constants::{NAVIGATOR_HINTS, RESULTS_HINTS, SPINNER_FRAMES};

pub fn render_header(frame: &mut Frame, area: Rect, app: &App) {
    let mut spans = vec![Span::styled("  visor  ", theme::TITLE)];

    match (app.view, &app.results) {
        (View::Results, Some(results)) => {
            let request = &results.request;
            let command = match request.kind {
                SearchKind::Content => ":match",
                SearchKind::Filename => ":file",
            };
            let count = match request.kind {
                SearchKind::Content => format!(
                    "{} matches in {} files",
                    results.total_matches(),
                    results.hits.len()
                ),
                SearchKind::Filename => format!("{} files", results.hits.len()),
            };
            spans.extend([
                Span::styled(format!("{command} "), theme::STATUS_TEXT),
                Span::styled(format!("\"{}\"", request.query), theme::HEADER_QUERY),
                Span::styled("  |  ", theme::STATUS_TEXT),
                Span::styled(count, theme::MATCH_COUNT),
                Span::styled("  |  case:", theme::STATUS_TEXT),
                toggle(request.options.case_sensitive),
                Span::styled("  exact:", theme::STATUS_TEXT),
                toggle(request.options.exact_match),
                Span::styled("  |  in ", theme::STATUS_TEXT),
                Span::styled(display_path(&request.base), theme::PATH_TEXT),
            ]);
        }
        _ => {
            spans.push(Span::styled(
                display_path(&app.navigator.cwd),
                theme::PATH_TEXT,
            ));
            if app.navigator.show_hidden {
                spans.push(Span::styled("  [hidden shown]", theme::STATUS_TEXT));
            }
        }
    }

    frame.render_widget(Paragraph::new(Line::from(spans)), area);
}

/// The footer shows, in order of priority: the command line, a running
/// search, the last message, or the key hints for the current view
pub fn render_footer(frame: &mut Frame, area: Rect, app: &App) {
    let spans = if let Some(text) = &app.command_line {
        vec![
            Span::styled("  :", theme::KEY_LABEL),
            Span::styled(text.clone(), theme::COMMAND_TEXT),
            Span::styled("█", theme::COMMAND_TEXT),
        ]
    } else if let Some(search) = &app.search {
        let frame_char = SPINNER_FRAMES[app.tick % SPINNER_FRAMES.len()];
        vec![
            Span::styled(format!("  {frame_char} "), theme::SPINNER),
            Span::styled("Searching ", theme::STATUS_TEXT),
            Span::styled(format!("\"{}\"", search.request.query), theme::HEADER_QUERY),
            Span::styled("…", theme::STATUS_TEXT),
        ]
    } else if let Some(message) = &app.message {
        let style = if message.is_error {
            theme::ERROR_TEXT
        } else {
            theme::STATUS_TEXT
        };
        vec![Span::styled(format!("  {}", message.text), style)]
    } else {
        let hints = match app.view {
            View::Navigator => NAVIGATOR_HINTS,
            View::Results => RESULTS_HINTS,
        };
        let mut spans = vec![Span::raw(" ")];
        for (key, label) in hints {
            spans.push(Span::styled(format!(" {key}"), theme::KEY_LABEL));
            spans.push(Span::styled(format!("{label} "), theme::STATUS_TEXT));
        }
        spans
    };

    frame.render_widget(Paragraph::new(Line::from(spans)), area);
}

fn toggle(on: bool) -> Span<'static> {
    if on {
        Span::styled("ON", theme::TOGGLE_ON)
    } else {
        Span::styled("OFF", theme::TOGGLE_OFF)
    }
}

/// The path with the home directory shown as `~`
fn display_path(path: &Path) -> String {
    if let Some(home) = std::env::var_os("HOME").map(std::path::PathBuf::from)
        && let Ok(rest) = path.strip_prefix(&home)
    {
        return if rest.as_os_str().is_empty() {
            "~".to_string()
        } else {
            format!("~/{}", rest.display())
        };
    }
    path.display().to_string()
}
