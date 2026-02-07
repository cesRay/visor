mod app;
mod event;
mod search;
mod ui;

use std::io::Write;
use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use clap::Parser;

use app::App;
use search::types::SearchOptions;

const SPINNER_FRAMES: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

#[derive(Parser)]
#[command(name = "lens", version, about = "A CMD+SHIFT+F search tool for the terminal")]
struct Cli {
    /// The search keyword or pattern
    keyword: String,

    /// Directory to search in (defaults to current directory)
    #[arg(short = 'd', long)]
    dir: Option<PathBuf>,

    /// Filter by file extension (e.g. "rs", "py")
    #[arg(long, short = 'x')]
    ext: Option<String>,

    /// Enable case-sensitive search
    #[arg(long, short = 'c')]
    case_sensitive: bool,

    /// Enable exact/word-boundary match
    #[arg(long, short = 'e')]
    exact: bool,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let base_path = match cli.dir {
        Some(p) => p,
        None => std::env::current_dir()?,
    };

    let options = SearchOptions {
        case_sensitive: cli.case_sensitive,
        exact_match: cli.exact,
        extension_filter: cli.ext,
    };

    // Search on a background thread with an inline spinner
    let results = search_with_spinner(&cli.keyword, &base_path, &options)?;

    if results.is_empty() {
        println!(
            "No matches found for \"{}\" in {}",
            cli.keyword,
            base_path.display()
        );
        return Ok(());
    }

    let total: usize = results.iter().map(|f| f.matches.len()).sum();
    println!(
        "\x1b[32m✓\x1b[0m Found \x1b[1m{total}\x1b[0m matches in \x1b[1m{}\x1b[0m files",
        results.len()
    );

    let mut app = App::new(cli.keyword, options, base_path);
    app.results = results;
    app.file_list_state.select(Some(0));

    // Launch TUI
    let mut terminal = ratatui::init();
    let result = app.run(&mut terminal);
    ratatui::restore();
    result?;

    Ok(())
}

fn search_with_spinner(
    keyword: &str,
    base_path: &PathBuf,
    options: &SearchOptions,
) -> anyhow::Result<Vec<search::types::FileMatches>> {
    let base = base_path.clone();
    let query = keyword.to_string();
    let opts = options.clone();

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let results = search::engine::search(&base, &query, &opts).unwrap_or_default();
        let _ = tx.send(results);
    });

    let mut tick: usize = 0;
    let mut stdout = std::io::stdout();

    // Hide cursor for cleaner animation
    print!("\x1b[?25l");
    let _ = stdout.flush();

    loop {
        match rx.try_recv() {
            Ok(results) => {
                print!("\r\x1b[2K\x1b[?25h");
                let _ = stdout.flush();
                return Ok(results);
            }
            Err(mpsc::TryRecvError::Disconnected) => {
                print!("\r\x1b[2K\x1b[?25h");
                let _ = stdout.flush();
                return Ok(Vec::new());
            }
            Err(mpsc::TryRecvError::Empty) => {
                let frame = SPINNER_FRAMES[tick % SPINNER_FRAMES.len()];
                let dots = ".".repeat((tick / 3) % 4);
                print!(
                    "\r\x1b[2K\x1b[36m{frame}\x1b[0m Searching matches{dots:<3}  \x1b[2m\"{keyword}\"\x1b[0m"
                );
                let _ = stdout.flush();
                tick += 1;
                thread::sleep(Duration::from_millis(80));
            }
        }
    }
}
