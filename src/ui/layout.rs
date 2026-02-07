use ratatui::layout::{Constraint, Layout, Rect};

pub struct AppLayout {
    pub header: Rect,
    pub file_list: Rect,
    pub preview: Rect,
    pub footer: Rect,
}

pub fn build(area: Rect) -> AppLayout {
    let [header, body, footer] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Fill(1),
        Constraint::Length(1),
    ])
    .areas(area);

    let [file_list, preview] = Layout::horizontal([
        Constraint::Percentage(30),
        Constraint::Percentage(70),
    ])
    .areas(body);

    AppLayout {
        header,
        file_list,
        preview,
        footer,
    }
}
