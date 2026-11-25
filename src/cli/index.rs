#[path = "types.rs"]
mod types;

use std::io::IsTerminal;

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
    for query in [&cli.match_query, &cli.file].into_iter().flatten() {
        anyhow::ensure!(!query.trim().is_empty(), "the search text cannot be blank");
    }

    let mut app = App::new(start, options);
    if let Some(query) = cli.match_query {
        app.start_search(SearchKind::Content, query);
    } else if let Some(query) = cli.file {
        app.start_search(SearchKind::Filename, query);
    }

    // ratatui::init panics without a terminal (e.g. output piped to a file)
    anyhow::ensure!(
        std::io::stdout().is_terminal(),
        "visor needs an interactive terminal"
    );
    let mut terminal = ratatui::init();
    let outcome = app.run(&mut terminal);
    ratatui::restore();

    if let ExitOutcome::ChangeDir(dir) = outcome? {
        // Paths inside visor are canonical (e.g. /private/tmp for /tmp). If the
        // target is where the shell already is, do nothing, so the shell keeps
        // its own spelling of the path.
        let here = std::env::current_dir().and_then(|d| d.canonicalize()).ok();
        if here.as_deref() == Some(dir.as_path()) {
            return Ok(());
        }
        shell::change_dir(&dir, cli.cwd_file.as_deref())
            .with_context(|| format!("could not hand {} to the shell", dir.display()))?;
    }
    Ok(())
}
