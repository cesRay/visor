/// What a key press means in the current view
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    MoveUp,
    MoveDown,
    MoveTop,
    MoveBottom,
    /// `→` / `l`: go into a directory, or open a file
    Forward,
    /// `←` / `h`: go to the parent directory, or back to the navigator
    Back,
    /// `Enter`: leave visor in a directory, or open a file
    Confirm,
    /// `q`: leave visor and cd into the selected directory
    Leave,
    /// `Esc`: quit without cd, go back, or close the command line
    Cancel,
    /// `Ctrl+C`: quit without cd from anywhere
    Abort,
    ToggleHidden,
    ToggleCaseSensitive,
    ToggleExactMatch,
    ScrollPreviewUp,
    ScrollPreviewDown,
    OpenCommand,
    /// A character typed into the command line
    Type(char),
    Backspace,
    None,
}
