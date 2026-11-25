#[path = "constants.rs"]
mod constants;
#[cfg(test)]
#[path = "tests.rs"]
mod tests;
#[path = "types.rs"]
pub mod types;

use std::path::PathBuf;
use std::sync::mpsc::TryRecvError;

use ratatui::DefaultTerminal;

use crate::command::{self, types::Command};
use crate::editor;
use crate::input::{self, types::Action};
use crate::navigator::types::{EntryKind, Navigator};
use crate::preview::types::PreviewCache;
use crate::results::types::Results;
use crate::search::{
    self,
    types::{SearchHandle, SearchKind, SearchOptions, SearchRequest},
};
use crate::ui;
use constants::TICK_RATE;
use types::{ExitOutcome, Message, View};

pub struct App {
    pub view: View,
    pub navigator: Navigator,
    pub results: Option<Results>,
    /// Text typed after `:`; `Some` while the command line is open
    pub command_line: Option<String>,
    /// The search running in the background, if any
    pub search: Option<SearchHandle>,
    pub options: SearchOptions,
    pub message: Option<Message>,
    pub preview_cache: Option<PreviewCache>,
    /// Counts redraws, for the spinner
    pub tick: usize,
    exit: Option<ExitOutcome>,
    /// File (and line) to open in vim once the current key is handled
    pending_open: Option<(PathBuf, Option<u64>)>,
}

impl App {
    pub fn new(start: PathBuf, options: SearchOptions) -> Self {
        Self {
            view: View::Navigator,
            navigator: Navigator::new(start),
            results: None,
            command_line: None,
            search: None,
            options,
            message: None,
            preview_cache: None,
            tick: 0,
            exit: None,
            pending_open: None,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<ExitOutcome> {
        loop {
            terminal.draw(|frame| ui::draw(frame, self))?;
            self.poll_search();

            if let Some(key) = input::poll(TICK_RATE)? {
                let action = input::map(key, self.view, self.command_line.is_some());
                self.handle(action);
            }
            self.tick = self.tick.wrapping_add(1);

            if let Some((path, line)) = self.pending_open.take() {
                self.open_in_editor(terminal, &path, line);
            }
            if let Some(outcome) = self.exit.take() {
                return Ok(outcome);
            }
        }
    }

    /// Starts a search below the navigator's current directory. A search
    /// that is still running is cancelled when its handle is replaced.
    pub fn start_search(&mut self, kind: SearchKind, query: String) {
        let request = SearchRequest {
            kind,
            query,
            base: self.navigator.cwd.clone(),
            options: self.options.clone(),
        };
        self.search = Some(search::spawn(request));
    }

    fn handle(&mut self, action: Action) {
        self.message = None;
        if action == Action::Abort {
            self.exit = Some(ExitOutcome::Quit);
            return;
        }
        if self.command_line.is_some() {
            self.handle_command_line(action);
            return;
        }
        if action == Action::OpenCommand {
            self.command_line = Some(String::new());
            return;
        }
        match self.view {
            View::Navigator => self.handle_navigator(action),
            View::Results => self.handle_results(action),
        }
    }

    fn handle_command_line(&mut self, action: Action) {
        let Some(line) = self.command_line.as_mut() else {
            return;
        };
        match action {
            Action::Type(c) => line.push(c),
            Action::Backspace if line.is_empty() => self.command_line = None,
            Action::Backspace => {
                line.pop();
            }
            Action::Cancel => self.command_line = None,
            Action::Confirm => {
                let text = self.command_line.take().unwrap_or_default();
                match command::parse(&text) {
                    Ok(Command::Match(query)) => self.start_search(SearchKind::Content, query),
                    Ok(Command::File(query)) => self.start_search(SearchKind::Filename, query),
                    Ok(Command::Quit) => self.exit = Some(ExitOutcome::Quit),
                    Err(e) => self.error(e.to_string()),
                }
            }
            _ => {}
        }
    }

    fn handle_navigator(&mut self, action: Action) {
        let nav = &mut self.navigator;
        match action {
            Action::MoveUp => nav.move_up(),
            Action::MoveDown => nav.move_down(),
            Action::MoveTop => nav.move_top(),
            Action::MoveBottom => nav.move_bottom(),
            Action::Back => nav.parent(),
            Action::ToggleHidden => nav.toggle_hidden(),
            Action::Forward | Action::Confirm => {
                let Some(entry) = nav.selected_entry().cloned() else {
                    return;
                };
                match entry.kind {
                    EntryKind::Dir if action == Action::Confirm => {
                        self.exit = Some(ExitOutcome::ChangeDir(entry.path));
                    }
                    EntryKind::Dir => {
                        if let Err(e) = nav.enter() {
                            self.error(e);
                        }
                    }
                    EntryKind::File => self.pending_open = Some((entry.path, None)),
                    EntryKind::BrokenLink => {
                        self.error(format!("{} is a broken symlink", entry.name))
                    }
                }
            }
            Action::Leave => self.exit = Some(ExitOutcome::ChangeDir(nav.exit_target())),
            Action::Cancel => self.exit = Some(ExitOutcome::Quit),
            _ => {}
        }
    }

    fn handle_results(&mut self, action: Action) {
        let Some(results) = self.results.as_mut() else {
            self.view = View::Navigator;
            return;
        };
        match action {
            Action::MoveUp => results.move_up(),
            Action::MoveDown => results.move_down(),
            Action::MoveTop => results.move_top(),
            Action::MoveBottom => results.move_bottom(),
            Action::ScrollPreviewUp => results.scroll_preview_up(),
            Action::ScrollPreviewDown => results.scroll_preview_down(),
            Action::Forward | Action::Confirm => self.pending_open = results.open_target(),
            Action::Back | Action::Cancel => {
                self.view = View::Navigator;
                self.results = None;
                self.search = None;
            }
            Action::Leave => {
                let dir = results
                    .exit_target()
                    .unwrap_or_else(|| self.navigator.cwd.clone());
                self.exit = Some(ExitOutcome::ChangeDir(dir));
            }
            Action::ToggleCaseSensitive | Action::ToggleExactMatch => {
                if action == Action::ToggleCaseSensitive {
                    self.options.case_sensitive = !self.options.case_sensitive;
                } else {
                    self.options.exact_match = !self.options.exact_match;
                }
                let mut request = results.request.clone();
                request.options = self.options.clone();
                self.search = Some(search::spawn(request));
            }
            _ => {}
        }
    }

    /// Moves a finished background search into the results view
    fn poll_search(&mut self) {
        let Some(handle) = self.search.as_ref() else {
            return;
        };
        let outcome = match handle.receiver.try_recv() {
            Ok(outcome) => outcome,
            Err(TryRecvError::Empty) => return,
            Err(TryRecvError::Disconnected) => {
                self.search = None;
                self.error("Search stopped unexpectedly".to_string());
                return;
            }
        };
        let Some(handle) = self.search.take() else {
            return;
        };
        let request = handle.request.clone();

        match outcome {
            Err(e) => self.error(e.to_string()),
            Ok(hits) if hits.is_empty() && self.view == View::Navigator => {
                self.error(format!("No matches for \"{}\"", request.query));
            }
            Ok(hits) => {
                self.results = Some(Results::new(request, hits));
                self.view = View::Results;
            }
        }
    }

    /// Hands the terminal to vim, then takes it back
    fn open_in_editor(
        &mut self,
        terminal: &mut DefaultTerminal,
        path: &std::path::Path,
        line: Option<u64>,
    ) {
        ratatui::restore();
        let result = editor::open(path, line);
        *terminal = ratatui::init();
        let _ = terminal.clear();

        if let Err(e) = result {
            self.error(format!("Could not start vim: {e}"));
        }
        // The file may have changed, or vim may have created new files
        self.navigator.refresh();
        self.preview_cache = None;
        if let Some(results) = self.results.as_mut() {
            results.reload_preview();
        }
    }

    fn error(&mut self, text: String) {
        self.message = Some(Message {
            text,
            is_error: true,
        });
    }
}
