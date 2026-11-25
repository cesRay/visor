pub const SPINNER_FRAMES: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

pub const NAVIGATOR_HINTS: &[(&str, &str)] = &[
    ("[←/→]", "move"),
    ("[Enter]", "cd here / open"),
    ("[q]", "quit here"),
    ("[Esc]", "cancel"),
    ("[:]", "match / file"),
    ("[.]", "hidden"),
];

pub const RESULTS_HINTS: &[(&str, &str)] = &[
    ("[j/k]", "navigate"),
    ("[Enter]", "open"),
    ("[c]", "ase"),
    ("[e]", "xact"),
    ("[PgUp/PgDn]", "scroll"),
    ("[Esc]", "back"),
    ("[q]", "quit here"),
];
