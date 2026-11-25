use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(name = "visor", version, about = "A ranger-style file navigator with content and filename search")]
pub struct Cli {
    /// The search keyword or pattern
    pub keyword: String,

    /// Directory to search in (defaults to current directory)
    #[arg(short = 'd', long)]
    pub dir: Option<PathBuf>,

    /// Filter by file extension (e.g. "rs", "py")
    #[arg(long, short = 'x')]
    pub ext: Option<String>,

    /// Enable case-sensitive search
    #[arg(long, short = 'c')]
    pub case_sensitive: bool,

    /// Enable exact/word-boundary match
    #[arg(long, short = 'e')]
    pub exact: bool,
}
