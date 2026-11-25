use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::shell::types::Shell;

#[derive(Parser)]
#[command(
    name = "visor",
    version,
    about = "A ranger-style file navigator with content and filename search"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Directory to open (defaults to the current directory)
    pub dir: Option<PathBuf>,

    /// Open straight into :match results for this keyword or pattern
    #[arg(
        short = 'm',
        long = "match",
        value_name = "KEYWORD",
        conflicts_with = "file"
    )]
    pub match_query: Option<String>,

    /// Open straight into :file results for this (fuzzy) relative path
    #[arg(short = 'f', long = "file", value_name = "PATH")]
    pub file: Option<String>,

    /// Only search files with this extension (e.g. "rs", "py")
    #[arg(long, short = 'x')]
    pub ext: Option<String>,

    /// Start with case-sensitive search
    #[arg(long, short = 'c')]
    pub case_sensitive: bool,

    /// Start with exact (whole-word) search
    #[arg(long, short = 'e')]
    pub exact: bool,

    /// File where the shell wrapper reads the directory to cd into
    #[arg(long, hide = true, global = true)]
    pub cwd_file: Option<PathBuf>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Print the shell function that lets visor change your directory.
    /// Add `eval "$(visor init zsh)"` to ~/.zshrc
    Init { shell: Shell },
}
