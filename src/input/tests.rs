use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use super::map;
use super::types::Action;
use crate::app::types::View;

fn key(code: KeyCode) -> KeyEvent {
    KeyEvent::new(code, KeyModifiers::NONE)
}

#[test]
fn navigator_keys() {
    let nav = |c| map(key(c), View::Navigator, false);
    assert_eq!(nav(KeyCode::Right), Action::Forward);
    assert_eq!(nav(KeyCode::Char('h')), Action::Back);
    assert_eq!(nav(KeyCode::Enter), Action::Confirm);
    assert_eq!(nav(KeyCode::Char('q')), Action::Leave);
    assert_eq!(nav(KeyCode::Esc), Action::Cancel);
    assert_eq!(nav(KeyCode::Char('.')), Action::ToggleHidden);
    assert_eq!(nav(KeyCode::Char(':')), Action::OpenCommand);
    assert_eq!(nav(KeyCode::Char('c')), Action::None);
}

#[test]
fn results_keys() {
    let res = |c| map(key(c), View::Results, false);
    assert_eq!(res(KeyCode::Char('c')), Action::ToggleCaseSensitive);
    assert_eq!(res(KeyCode::Char('e')), Action::ToggleExactMatch);
    assert_eq!(res(KeyCode::PageDown), Action::ScrollPreviewDown);
    assert_eq!(res(KeyCode::Char('.')), Action::None);
}

#[test]
fn command_line_captures_letters() {
    let cmd = |c| map(key(c), View::Navigator, true);
    assert_eq!(cmd(KeyCode::Char('q')), Action::Type('q'));
    assert_eq!(cmd(KeyCode::Char('j')), Action::Type('j'));
    assert_eq!(cmd(KeyCode::Enter), Action::Confirm);
    assert_eq!(cmd(KeyCode::Esc), Action::Cancel);
    assert_eq!(cmd(KeyCode::Backspace), Action::Backspace);
}

#[test]
fn ctrl_c_aborts_everywhere() {
    let ctrl_c = KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL);
    assert_eq!(map(ctrl_c, View::Navigator, false), Action::Abort);
    assert_eq!(map(ctrl_c, View::Results, true), Action::Abort);
}
