#[path = "types.rs"]
pub mod types;

use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};

use types::AppAction;

pub fn poll(timeout: Duration) -> std::io::Result<AppAction> {
    if event::poll(timeout)? {
        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                return Ok(AppAction::None);
            }
            return Ok(match key.code {
                KeyCode::Char('q') | KeyCode::Esc => AppAction::Quit,
                KeyCode::Up | KeyCode::Char('k') => AppAction::MoveUp,
                KeyCode::Down | KeyCode::Char('j') => AppAction::MoveDown,
                KeyCode::Enter | KeyCode::Right | KeyCode::Char('l') => AppAction::OpenFile,
                KeyCode::Char('c') => AppAction::ToggleCaseSensitive,
                KeyCode::Char('e') => AppAction::ToggleExactMatch,
                KeyCode::PageUp => AppAction::ScrollPreviewUp,
                KeyCode::PageDown => AppAction::ScrollPreviewDown,
                _ => AppAction::None,
            });
        }
    }
    Ok(AppAction::Tick)
}
