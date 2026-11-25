use std::path::PathBuf;
use std::time::Duration;

use ratatui::DefaultTerminal;
use ratatui::widgets::ListState;

use crate::input::{self, types::AppAction};
use crate::search;
use crate::search::types::{FileMatches, SearchOptions};
use crate::ui;

pub struct App {
    pub should_quit: bool,
    pub query: String,
    pub search_options: SearchOptions,
    pub results: Vec<FileMatches>,
    pub file_list_state: ListState,
    pub preview_scroll: u16,
    pub preview_cache: Option<(PathBuf, String)>,
    pub base_path: PathBuf,
    pub file_list_scroll: usize,
}

impl App {
    pub fn new(query: String, options: SearchOptions, base_path: PathBuf) -> Self {
        Self {
            should_quit: false,
            query,
            search_options: options,
            results: Vec::new(),
            file_list_state: ListState::default(),
            preview_scroll: 0,
            preview_cache: None,
            base_path,
            file_list_scroll: 0,
        }
    }

    pub fn selected_file(&self) -> Option<&FileMatches> {
        self.file_list_state
            .selected()
            .and_then(|i| self.results.get(i))
    }

    pub fn run_search(&mut self) {
        match search::content::search(&self.base_path, &self.query, &self.search_options) {
            Ok(results) => {
                self.results = results;
                if !self.results.is_empty() {
                    self.file_list_state.select(Some(0));
                    self.load_preview();
                    self.preview_scroll = self.compute_initial_scroll();
                } else {
                    self.file_list_state.select(None);
                    self.preview_cache = None;
                }
            }
            Err(_) => {
                self.results = Vec::new();
                self.file_list_state.select(None);
                self.preview_cache = None;
            }
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        // Load preview for the initially selected file
        self.load_preview();

        while !self.should_quit {
            terminal.draw(|frame| ui::draw(frame, self))?;

            match input::poll(Duration::from_millis(80))? {
                AppAction::Quit => self.should_quit = true,
                AppAction::MoveUp => self.select_previous_file(),
                AppAction::MoveDown => self.select_next_file(),
                AppAction::OpenFile => {
                    ratatui::restore();
                    self.open_selected_file();
                    *terminal = ratatui::init();
                }
                AppAction::ToggleCaseSensitive => {
                    self.search_options.case_sensitive = !self.search_options.case_sensitive;
                    self.run_search();
                }
                AppAction::ToggleExactMatch => {
                    self.search_options.exact_match = !self.search_options.exact_match;
                    self.run_search();
                }
                AppAction::ScrollPreviewUp => {
                    self.preview_scroll = self.preview_scroll.saturating_sub(5);
                }
                AppAction::ScrollPreviewDown => {
                    self.preview_scroll = self.preview_scroll.saturating_add(5);
                }
                AppAction::Tick | AppAction::None => {}
            }
        }
        Ok(())
    }

    fn select_next_file(&mut self) {
        if self.results.is_empty() {
            return;
        }
        let current = self.file_list_state.selected().unwrap_or(0);
        let next = if current + 1 < self.results.len() {
            current + 1
        } else {
            current
        };
        self.file_list_state.select(Some(next));
        self.load_preview();
        self.preview_scroll = self.compute_initial_scroll();
    }

    fn select_previous_file(&mut self) {
        if self.results.is_empty() {
            return;
        }
        let current = self.file_list_state.selected().unwrap_or(0);
        let prev = current.saturating_sub(1);
        self.file_list_state.select(Some(prev));
        self.load_preview();
        self.preview_scroll = self.compute_initial_scroll();
    }

    fn load_preview(&mut self) {
        if let Some(fm) = self.selected_file() {
            let path = fm.path.clone();
            if self.preview_cache.as_ref().map(|(p, _)| p) != Some(&path) {
                let too_large = std::fs::metadata(&path)
                    .map(|m| m.len() > 1_000_000)
                    .unwrap_or(false);

                if too_large {
                    self.preview_cache =
                        Some((path, "[File too large to preview]".to_string()));
                } else {
                    match std::fs::read_to_string(&path) {
                        Ok(content) => {
                            self.preview_cache = Some((path, content));
                        }
                        Err(_) => {
                            self.preview_cache =
                                Some((path, "[Unable to read file]".to_string()));
                        }
                    }
                }
            }
        }
    }

    fn compute_initial_scroll(&self) -> u16 {
        self.selected_file()
            .and_then(|fm| fm.matches.first())
            .map(|m| m.line_number.saturating_sub(5) as u16)
            .unwrap_or(0)
    }

    fn open_selected_file(&self) {
        if let Some(fm) = self.selected_file() {
            let line = fm.matches.first().map(|m| m.line_number).unwrap_or(1);
            let editor = std::env::var("EDITOR")
                .or_else(|_| std::env::var("VISUAL"))
                .unwrap_or_else(|_| "vim".to_string());
            let _ = std::process::Command::new(&editor)
                .arg(format!("+{line}"))
                .arg(&fm.path)
                .status();
        }
    }
}
