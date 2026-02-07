# lens

A fast, aesthetic **CMD+SHIFT+F** search tool for the terminal. Built in Rust.

`lens` searches for a keyword across all files in your current directory and opens a two-panel TUI — file list on the left, match-highlighted preview on the right — just like VSCode's search, but in your terminal.

```
+----------------------------------------------+
| lens: "keyword"  | 42 matches in 7 files     |
+------------------+---------------------------|
|                  |                            |
|  src/main.rs (2) |  14: fn main() {          |
|  src/app.rs  (1) |  15:   let keyword = ..   |
|> src/search.rs(3)|  16:   search(keyword);   |
|  src/ui/mod.rs(1)|  17: }                    |
|                  |  ...                       |
|                  |  42: // keyword again      |
|                  |                            |
+------------------+---------------------------|
| [c]ase [e]xact [q]uit  arrows:navigate       |
+----------------------------------------------+
```

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.85+)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Install from source

```bash
git clone https://github.com/cesRay/lens.git
cd lens
cargo install --path .
```

This installs the `lens` binary to `~/.cargo/bin/`, which should already be in your `PATH` after installing Rust.

### Verify

```bash
lens --version
```

## Usage

```bash
# Search the current directory
lens "keyword"

# Search a specific directory
lens "keyword" -d /path/to/project

# Filter by file extension
lens "keyword" -x rs

# Case-sensitive search
lens "keyword" -c

# Exact word match
lens "keyword" -e

# Combine flags
lens "TODO" -x py -c -e
```

### Options

| Flag | Description |
|------|-------------|
| `-d, --dir <DIR>` | Directory to search in (defaults to `.`) |
| `-x, --ext <EXT>` | Filter by file extension (e.g. `rs`, `py`, `ts`) |
| `-c, --case-sensitive` | Enable case-sensitive search |
| `-e, --exact` | Enable exact/word-boundary match |
| `-h, --help` | Print help |
| `-V, --version` | Print version |

## Keybindings

Once inside the TUI:

| Key | Action |
|-----|--------|
| `j` / `Down` | Next file |
| `k` / `Up` | Previous file |
| `Enter` / `Right` / `l` | Open file in `$EDITOR` at the first match line |
| `PgUp` | Scroll preview up |
| `PgDn` | Scroll preview down |
| `c` | Toggle case sensitivity (re-runs search) |
| `e` | Toggle exact match (re-runs search) |
| `q` / `Esc` | Quit |

The file opens in your `$EDITOR` (falls back to `$VISUAL`, then `vim`).

## Development

### Build

```bash
cargo build
```

### Run locally

```bash
cargo run -- "keyword"
```

### Project structure

```
src/
  main.rs               Entry point, CLI parsing, search spinner
  app.rs                App state machine, event dispatch, run loop
  event.rs              Crossterm event polling, key-to-action mapping
  search.rs             Module root for search
  search/
    types.rs            SearchOptions, FileMatches, MatchInfo
    engine.rs           Parallel file walking + grep matching
  ui.rs                 Module root, top-level draw()
  ui/
    layout.rs           3-region layout (header, panels, footer)
    file_list.rs        Left panel: scrollable file list
    preview.rs          Right panel: file content with match highlighting
    status_bar.rs       Header (query/stats) + footer (keybindings)
    theme.rs            Color palette and style constants
```

### Key dependencies

| Crate | Purpose |
|-------|---------|
| [ratatui](https://github.com/ratatui/ratatui) | Terminal UI framework |
| [crossterm](https://github.com/crossterm-rs/crossterm) | Terminal backend (input, raw mode) |
| [clap](https://github.com/clap-rs/clap) | CLI argument parsing |
| [grep-regex](https://github.com/BurntSushi/ripgrep) | Regex matching (from ripgrep) |
| [grep-searcher](https://github.com/BurntSushi/ripgrep) | Line-oriented file search (from ripgrep) |
| [ignore](https://github.com/BurntSushi/ripgrep) | Fast parallel directory walking, respects `.gitignore` |

## License

MIT
