#[cfg(test)]
#[path = "tests.rs"]
mod tests;
#[path = "types.rs"]
pub mod types;

use types::{Command, ParseError};

/// Parses the text typed after `:`. The argument is the rest of the line,
/// so it may contain spaces. One pair of surrounding quotes is removed.
pub fn parse(input: &str) -> Result<Command, ParseError> {
    let input = input.trim();
    let (name, arg) = match input.split_once(char::is_whitespace) {
        // Trim again after unquoting, so `"  "` counts as missing
        Some((name, arg)) => (name, unquote(arg.trim()).trim()),
        None => (input, ""),
    };

    match name {
        "" => Err(ParseError::Empty),
        "q" | "quit" => Ok(Command::Quit),
        "match" | "m" if arg.is_empty() => Err(ParseError::MissingArgument(":match <keyword>")),
        "match" | "m" => Ok(Command::Match(arg.to_string())),
        "file" | "f" if arg.is_empty() => Err(ParseError::MissingArgument(":file <path>")),
        "file" | "f" => Ok(Command::File(arg.to_string())),
        other => Err(ParseError::Unknown(other.to_string())),
    }
}

fn unquote(s: &str) -> &str {
    for q in ['"', '\''] {
        if s.len() >= 2 && s.starts_with(q) && s.ends_with(q) {
            return &s[1..s.len() - 1];
        }
    }
    s
}
