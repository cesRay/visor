#[cfg(test)]
#[path = "tests.rs"]
mod tests;
#[path = "types.rs"]
pub mod types;

use std::io;
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use crate::app::types::View;
use types::Action;

/// Waits up to `timeout` for a key press
pub fn poll(timeout: Duration) -> io::Result<Option<KeyEvent>> {
    if event::poll(timeout)?
        && let Event::Key(key) = event::read()?
        && key.kind == KeyEventKind::Press
    {
        return Ok(Some(key));
    }
    Ok(None)
}

/// Maps a key to an action for the current view
pub fn map(key: KeyEvent, view: View, command_open: bool) -> Action {
    if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
        return Action::Abort;
    }

    if command_open {
        return match key.code {
            KeyCode::Enter => Action::Confirm,
            KeyCode::Esc => Action::Cancel,
            KeyCode::Backspace => Action::Backspace,
            KeyCode::Char(c) => Action::Type(c),
            _ => Action::None,
        };
    }

    let shared = match key.code {
        KeyCode::Up | KeyCode::Char('k') => Action::MoveUp,
        KeyCode::Down | KeyCode::Char('j') => Action::MoveDown,
        KeyCode::Home | KeyCode::Char('g') => Action::MoveTop,
        KeyCode::End | KeyCode::Char('G') => Action::MoveBottom,
        KeyCode::Right | KeyCode::Char('l') => Action::Forward,
        KeyCode::Left | KeyCode::Char('h') => Action::Back,
        KeyCode::Enter => Action::Confirm,
        KeyCode::Char('q') => Action::Leave,
        KeyCode::Esc => Action::Cancel,
        KeyCode::Char(':') => Action::OpenCommand,
        _ => Action::None,
    };
    if shared != Action::None {
        return shared;
    }

    match (view, key.code) {
        (View::Navigator, KeyCode::Char('.')) => Action::ToggleHidden,
        (View::Results, KeyCode::Char('c')) => Action::ToggleCaseSensitive,
        (View::Results, KeyCode::Char('e')) => Action::ToggleExactMatch,
        (View::Results, KeyCode::PageUp) => Action::ScrollPreviewUp,
        (View::Results, KeyCode::PageDown) => Action::ScrollPreviewDown,
        _ => Action::None,
    }
}
