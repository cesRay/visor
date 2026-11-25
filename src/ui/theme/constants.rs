use ratatui::style::{Color, Modifier, Style};

pub const BORDER: Style = Style::new().fg(Color::DarkGray);

pub const BORDER_FOCUSED: Style = Style::new().fg(Color::Rgb(100, 100, 200));

pub const FILE_LIST_BASE: Style = Style::reset().fg(Color::DarkGray);

pub const SELECTED_FILE: Style = Style::reset()
    .fg(Color::Rgb(255, 180, 50))
    .add_modifier(Modifier::BOLD);

pub const LINE_NUMBER: Style = Style::new().fg(Color::DarkGray);

pub const MATCH_HIGHLIGHT: Style = Style::new()
    .fg(Color::Black)
    .bg(Color::Yellow)
    .add_modifier(Modifier::BOLD);

pub const KEY_LABEL: Style = Style::new()
    .fg(Color::Cyan)
    .add_modifier(Modifier::BOLD);

pub const STATUS_TEXT: Style = Style::new().fg(Color::Gray);

pub const TOGGLE_ON: Style = Style::new()
    .fg(Color::Green)
    .add_modifier(Modifier::BOLD);

pub const TOGGLE_OFF: Style = Style::new().fg(Color::DarkGray);

pub const HEADER_QUERY: Style = Style::new()
    .fg(Color::Yellow)
    .add_modifier(Modifier::BOLD);

pub const MATCH_COUNT: Style = Style::new().fg(Color::Rgb(180, 180, 255));

pub const NO_RESULTS: Style = Style::new()
    .fg(Color::DarkGray)
    .add_modifier(Modifier::ITALIC);

pub const PREVIEW_TITLE: Style = Style::new()
    .fg(Color::Rgb(255, 180, 50))
    .add_modifier(Modifier::BOLD);
