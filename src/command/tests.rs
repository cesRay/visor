use super::parse;
use super::types::{Command, ParseError};

#[test]
fn parses_match_and_file_with_spaces_and_quotes() {
    assert_eq!(parse("match TODO"), Ok(Command::Match("TODO".into())));
    assert_eq!(
        parse("  match  fn main  "),
        Ok(Command::Match("fn main".into()))
    );
    assert_eq!(parse("match \"a b\""), Ok(Command::Match("a b".into())));
    assert_eq!(parse("file app"), Ok(Command::File("app".into())));
    assert_eq!(parse("f 'x.rs'"), Ok(Command::File("x.rs".into())));
}

#[test]
fn parses_quit() {
    assert_eq!(parse("q"), Ok(Command::Quit));
    assert_eq!(parse("quit"), Ok(Command::Quit));
}

#[test]
fn reports_errors() {
    assert_eq!(parse("   "), Err(ParseError::Empty));
    assert_eq!(
        parse("match"),
        Err(ParseError::MissingArgument(":match <keyword>"))
    );
    assert_eq!(
        parse("file   "),
        Err(ParseError::MissingArgument(":file <name>"))
    );
    assert_eq!(parse("open x"), Err(ParseError::Unknown("open".into())));
}
