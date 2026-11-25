#[path = "app/index.rs"]
mod app;
#[path = "cli/index.rs"]
mod cli;
#[path = "input/index.rs"]
mod input;
#[path = "search/index.rs"]
mod search;
#[path = "ui/index.rs"]
mod ui;

fn main() -> anyhow::Result<()> {
    cli::run()
}
