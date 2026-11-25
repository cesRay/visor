#[path = "types.rs"]
mod types;

use ratatui::layout::{Constraint, Layout, Rect};

use types::AppLayout;

pub fn build(area: Rect) -> AppLayout {
    let [header, body, footer] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Fill(1),
        Constraint::Length(1),
    ])
    .areas(area);

    AppLayout {
        header,
        body,
        footer,
    }
}
