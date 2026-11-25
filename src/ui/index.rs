#[path = "layout/index.rs"]
mod layout;
#[path = "results/index.rs"]
mod results;
#[path = "status_bar/index.rs"]
mod status_bar;
#[path = "theme/index.rs"]
mod theme;

use ratatui::Frame;

use crate::app::App;

pub fn draw(frame: &mut Frame, app: &mut App) {
    let areas = layout::build(frame.area());

    status_bar::render_header(frame, areas.header, app);
    results::render(frame, areas.body, app);
    status_bar::render_footer(frame, areas.footer);
}
