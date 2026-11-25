use std::fmt;

/// A command typed after `:`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    /// `:match <keyword>`: search file contents
    Match(String),
    /// `:file <name>`: search file names
    File(String),
    /// `:q`: quit without changing directory
    Quit,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    Empty,
    /// The command needs an argument; holds the usage text
    MissingArgument(&'static str),
    Unknown(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::Empty => write!(f, "Type a command: :match <keyword>, :file <name>, :q"),
            ParseError::MissingArgument(usage) => write!(f, "Usage: {usage}"),
            ParseError::Unknown(name) => write!(f, "Unknown command: :{name}"),
        }
    }
}
