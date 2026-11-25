#[path = "list/index.rs"]
mod list;
#[path = "preview/index.rs"]
mod preview;

use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};

use crate::app::App;

/// Results view: file list on the left, highlighted preview on the right
pub fn render(frame: &mut Frame, area: Rect, app: &mut App) {
    let [list_area, preview_area] =
        Layout::horizontal([Constraint::Percentage(30), Constraint::Percentage(70)]).areas(area);

    list::render(frame, list_area, app);
    preview::render(frame, preview_area, app);
}
