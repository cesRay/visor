# visor

A fast, ranger-style **file navigator** for the terminal, with content search and filename search built in. Written in Rust.

Move through directories with the arrow keys, preview files as you go, search inside files with `:match` or by name with `:file`, and open anything in vim. Press `Enter` on a directory (or `q`) and your shell lands in that directory when visor closes.

```
  visor  ~/proj/src
┌ proj ──────┐┌ src ──────────────┐┌ main.rs ─────────────────────────┐
│  docs/     ││> ui/              ││   1 fn main() {                  │
│> src/      ││  app.rs           ││   2     // TODO: wire app        │
│  README.md ││  main.rs          ││   3 }                            │
└────────────┘└───────────────────┘└──────────────────────────────────┘
  [←/→]move  [Enter]cd here / open  [q]quit here  [Esc]cancel  [:]match / file
```

## Installation

### Homebrew

```bash
brew tap cesRay/tap
brew trust --formula cesray/tap/visor   # Homebrew 7+ only loads trusted third-party formulae
brew install visor
```

On older Homebrew versions without `brew trust`, run `brew install cesRay/tap/visor` instead.

Homebrew also installs vim, which visor uses to open every file. After the first install, `brew upgrade visor` keeps it up to date.

### From source

Requires [Rust](https://www.rust-lang.org/tools/install) 1.88+ and vim on your `PATH`.

```bash
git clone https://github.com/cesRay/visor.git
cd visor
cargo install --path .
```

### Shell integration (required to change directory)

A program cannot change the directory of the shell that started it. visor ships a small shell function that does the `cd` for it. Add the line for your shell to its config file, then open a new terminal:

| Shell | File | Line |
|-------|------|------|
| zsh | `~/.zshrc` | `eval "$(visor init zsh)"` |
| bash | `~/.bashrc` | `eval "$(visor init bash)"` |
| fish | `~/.config/fish/config.fish` | `visor init fish \| source` |

Without it, visor still works, but on exit it only prints the chosen directory.

## Usage

```bash
visor                    # open the navigator in the current directory
visor ~/proj             # open the navigator in ~/proj
visor -m TODO            # open straight into :match results for "TODO"
visor -f app -x rs       # open straight into :file results, .rs files only
visor -m "fn main" -c -e # case-sensitive, whole-word search
```

### Options

| Flag | Description |
|------|-------------|
| `[DIR]` | Directory to open (defaults to `.`) |
| `-m, --match <KEYWORD>` | Start with `:match` results for this keyword or regex |
| `-f, --file <NAME>` | Start with `:file` results for this name or regex |
| `-x, --ext <EXT>` | Only search files with this extension (e.g. `rs`, `py`) |
| `-c, --case-sensitive` | Start with case-sensitive search |
| `-e, --exact` | Start with exact (whole-word) search |
| `init <SHELL>` | Print the shell function for `zsh`, `bash` or `fish` |

To open a directory that is literally called `init`, write `visor ./init`.
| `-h, --help` | Print help |
| `-V, --version` | Print version |

## Keybindings

### Navigator

| Key | Action |
|-----|--------|
| `↑` `↓` / `k` `j` | Move the selection |
| `→` / `l` | Go into a directory, or open a file in vim |
| `←` / `h` | Go to the parent directory |
| `Enter` | On a directory: **exit visor and cd into it**. On a file: open it in vim |
| `q` | Exit visor and cd into the selected directory (or the current one if a file is selected) |
| `Esc` / `Ctrl+C` | Exit visor without changing directory |
| `g` / `G` | Jump to the top / bottom |
| `.` | Show or hide hidden files |
| `:` | Open the command line |

### Commands

| Command | Action |
|---------|--------|
| `:match <keyword>` | Search file contents below the current directory |
| `:file <name>` | Search file names below the current directory |
| `:q` | Exit without changing directory |

Both searches are case-insensitive regular expressions by default, skip hidden files and respect `.gitignore`. `:file` matches the file name only, not the directories above it. `Esc` closes the command line.

### Results

| Key | Action |
|-----|--------|
| `↑` `↓` / `k` `j` | Move the selection |
| `Enter` / `→` | Open in vim (`:match` results open at the first matching line) |
| `PgUp` / `PgDn` | Scroll the preview |
| `c` / `e` | Toggle case-sensitive / exact match and search again |
| `:` | Start a new search |
| `Esc` / `←` | Back to the navigator |
| `q` | Exit visor and cd into the selected file's directory |

visor always opens files in vim, whatever `$EDITOR` is set to. When vim closes, you are back in visor.

## Development

```bash
cargo build                       # build
cargo run -- ~/some/dir           # run locally
cargo test                        # unit and render tests
cargo clippy --all-targets        # lint
```

### Project structure

Every module is a folder. `index.rs` holds the module's logic, and sibling files are named by the kind of item they hold (`types.rs`, `constants.rs`, `tests.rs`). Modules are wired with `#[path = "..."]` attributes; `main.rs` only declares them.

```
src/
  main.rs            crate root: module declarations, calls cli::run
  cli/               argument parsing, start-up checks, exit handling
  app/               App state, run loop, mode dispatch
  input/             key → Action mapping per view
  navigator/         directory listing, movement, exit target
  command/           parsing of :match / :file / :q
  search/            background search worker
    content/         keyword search inside files
    filename/        file name search
    matcher/         shared regex builder (case, exact)
  results/           results view state (selection, preview scroll)
  preview/           file and directory previews (binary / size checks)
  editor/            finding and launching vim
  shell/             `visor init` wrappers and --cwd-file handling
  ui/                drawing only
    navigator/       three-column navigator
    results/         results list + highlighted preview
    status_bar/      header, command line, hints
    rows/            shared row renderer
    layout/  theme/
packaging/homebrew/  formula source and bump script for cesRay/homebrew-tap
```

### Releasing

1. Bump `version` in `Cargo.toml`, commit, then tag and push: `git tag v0.2.0 && git push --tags`.
2. Run `packaging/homebrew/bump.sh 0.2.0` to update the formula's URL and SHA-256.
3. Copy `packaging/homebrew/visor.rb` to `Formula/visor.rb` in `cesRay/homebrew-tap` and push.

### Key dependencies

| Crate | Purpose |
|-------|---------|
| [ratatui](https://github.com/ratatui/ratatui) | Terminal UI framework |
| [crossterm](https://github.com/crossterm-rs/crossterm) | Terminal backend (input, raw mode) |
| [clap](https://github.com/clap-rs/clap) | CLI argument parsing |
| [grep-regex](https://github.com/BurntSushi/ripgrep) / [grep-searcher](https://github.com/BurntSushi/ripgrep) | Regex matching and line search (from ripgrep) |
| [ignore](https://github.com/BurntSushi/ripgrep) | Fast parallel directory walking that respects `.gitignore` |

## License

MIT
