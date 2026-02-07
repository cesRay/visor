pub mod file_list;
pub mod layout;
pub mod preview;
pub mod status_bar;
pub mod theme;

use ratatui::Frame;

use crate::app::App;

pub fn draw(frame: &mut Frame, app: &mut App) {
    let areas = layout::build(frame.area());

    status_bar::render_header(frame, areas.header, app);
    file_list::render(frame, areas.file_list, app);
    preview::render(frame, areas.preview, app);
    status_bar::render_footer(frame, areas.footer);
}
