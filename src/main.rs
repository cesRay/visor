#[path = "app/index.rs"]
mod app;
#[path = "cli/index.rs"]
mod cli;
#[path = "command/index.rs"]
mod command;
#[path = "editor/index.rs"]
mod editor;
#[path = "input/index.rs"]
mod input;
#[path = "navigator/index.rs"]
mod navigator;
#[path = "preview/index.rs"]
mod preview;
#[path = "results/index.rs"]
mod results;
#[path = "search/index.rs"]
mod search;
#[path = "shell/index.rs"]
mod shell;
#[path = "ui/index.rs"]
mod ui;

fn main() -> anyhow::Result<()> {
    cli::run()
}
