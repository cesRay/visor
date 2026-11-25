#[path = "layout/index.rs"]
mod layout;
#[path = "navigator/index.rs"]
mod navigator;
#[path = "results/index.rs"]
mod results;
#[path = "rows/index.rs"]
mod rows;
#[path = "status_bar/index.rs"]
mod status_bar;
#[cfg(test)]
#[path = "tests.rs"]
mod tests;
#[path = "theme/index.rs"]
mod theme;

use ratatui::Frame;

use crate::app::App;
use crate::app::types::View;

pub fn draw(frame: &mut Frame, app: &mut App) {
    let areas = layout::build(frame.area());

    status_bar::render_header(frame, areas.header, app);
    match (app.view, app.results.as_mut()) {
        (View::Results, Some(results)) => results::render(frame, areas.body, results),
        _ => navigator::render(frame, areas.body, app),
    }
    status_bar::render_footer(frame, areas.footer, app);
}
