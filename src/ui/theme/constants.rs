use ratatui::style::{Color, Modifier, Style};

pub const BORDER: Style = Style::new().fg(Color::DarkGray);

pub const BORDER_FOCUSED: Style = Style::new().fg(Color::Rgb(100, 100, 200));

pub const FILE_LIST_BASE: Style = Style::reset().fg(Color::DarkGray);

/// The selection your keys move: the middle navigator column and the results list
pub const ACTIVE_SELECTION: Style = Style::reset()
    .fg(Color::Rgb(0xCF, 0x42, 0x5C))
    .add_modifier(Modifier::BOLD);

/// Marks the current directory inside the parent column; it is not movable
pub const PARENT_SELECTION: Style = Style::reset()
    .fg(Color::Rgb(255, 180, 50))
    .add_modifier(Modifier::BOLD);

pub const LINE_NUMBER: Style = Style::new().fg(Color::DarkGray);

pub const MATCH_HIGHLIGHT: Style = Style::new()
    .fg(Color::Black)
    .bg(Color::Yellow)
    .add_modifier(Modifier::BOLD);

pub const KEY_LABEL: Style = Style::new().fg(Color::Cyan).add_modifier(Modifier::BOLD);

pub const STATUS_TEXT: Style = Style::new().fg(Color::Gray);

pub const TOGGLE_ON: Style = Style::new().fg(Color::Green).add_modifier(Modifier::BOLD);

pub const TOGGLE_OFF: Style = Style::new().fg(Color::DarkGray);

pub const HEADER_QUERY: Style = Style::new().fg(Color::Yellow).add_modifier(Modifier::BOLD);

pub const MATCH_COUNT: Style = Style::new().fg(Color::Rgb(180, 180, 255));

pub const NO_RESULTS: Style = Style::new()
    .fg(Color::DarkGray)
    .add_modifier(Modifier::ITALIC);

pub const PREVIEW_TITLE: Style = Style::new()
    .fg(Color::Rgb(255, 180, 50))
    .add_modifier(Modifier::BOLD);

pub const DIRECTORY: Style = Style::reset()
    .fg(Color::Rgb(110, 150, 255))
    .add_modifier(Modifier::BOLD);

pub const BROKEN_LINK: Style = Style::reset().fg(Color::Rgb(200, 80, 80));

pub const PATH_TEXT: Style = Style::new()
    .fg(Color::Rgb(255, 180, 50))
    .add_modifier(Modifier::BOLD);

pub const ERROR_TEXT: Style = Style::new().fg(Color::Rgb(230, 90, 90));

pub const COMMAND_TEXT: Style = Style::new().fg(Color::White);

pub const SPINNER: Style = Style::new().fg(Color::Cyan);
