use clap::ValueEnum;

/// Shells that `visor init` can print a wrapper function for
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum Shell {
    Zsh,
    Bash,
    Fish,
}
