use std::path::PathBuf;

/// Which view fills the body of the screen
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum View {
    Navigator,
    Results,
}

/// How visor ended
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExitOutcome {
    /// Esc, Ctrl+C or `:q`: the shell stays where it was
    Quit,
    /// `q` or Enter on a directory: the shell cds here
    ChangeDir(PathBuf),
}

/// A one-line notice in the status bar, cleared by the next key press
#[derive(Debug, Clone)]
pub struct Message {
    pub text: String,
    pub is_error: bool,
}
