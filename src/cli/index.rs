#[path = "types.rs"]
mod types;

use anyhow::Context;
use clap::Parser;

use crate::app::App;
use crate::app::types::ExitOutcome;
use crate::editor::{self, constants::MISSING_EDITOR_MESSAGE};
use crate::search::types::{SearchKind, SearchOptions};
use crate::shell;
use types::{Cli, Commands};

pub fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(Commands::Init { shell }) = cli.command {
        print!("{}", shell::init_script(shell));
        return Ok(());
    }

    if editor::find().is_none() {
        eprintln!("{MISSING_EDITOR_MESSAGE}");
        std::process::exit(1);
    }

    let dir = match cli.dir {
        Some(dir) => dir,
        None => std::env::current_dir()?,
    };
    let start = dir
        .canonicalize()
        .with_context(|| format!("cannot open {}", dir.display()))?;
    anyhow::ensure!(start.is_dir(), "{} is not a directory", dir.display());

    let options = SearchOptions {
        case_sensitive: cli.case_sensitive,
        exact_match: cli.exact,
        extension_filter: cli.ext,
    };
    let mut app = App::new(start, options);
    if let Some(query) = cli.match_query {
        app.start_search(SearchKind::Content, query);
    } else if let Some(query) = cli.file {
        app.start_search(SearchKind::Filename, query);
    }

    let mut terminal = ratatui::init();
    let outcome = app.run(&mut terminal);
    ratatui::restore();

    if let ExitOutcome::ChangeDir(dir) = outcome? {
        shell::change_dir(&dir, cli.cwd_file.as_deref())
            .with_context(|| format!("could not hand {} to the shell", dir.display()))?;
    }
    Ok(())
}
