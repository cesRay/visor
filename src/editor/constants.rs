/// visor always opens files in vim, whatever $EDITOR says
pub const EDITOR_BIN: &str = "vim";

pub const MISSING_EDITOR_MESSAGE: &str =
    "visor: vim not found on PATH. Install it with: brew install vim";
