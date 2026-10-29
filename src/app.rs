use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::path::PathBuf;
use crate::config::OxidianConfig;
use crate::editor::open_in_neovim;
use crate::theme::Theme;
use crate::ui::components::*;
use crate::ui::layout::ActivePanel;
use crate::vault::{Note, VaultIndex};

pub struct App {
    pub vault: VaultIndex,
    pub theme: Theme,
    pub config: OxidianConfig,
    pub mode: AppMode,
    pub active_panel: ActivePanel,
    pub show_explorer: bool,
    pub show_backlinks: bool,

    // Component states
    pub bufferline: BufferlineState,
    pub file_tree: FileTreeState,
    pub note_viewer: NoteViewerState,
    pub backlinks: BacklinksState,
    pub fuzzy_finder: FuzzyFinderState,
    pub vault_switcher: VaultSwitcherState,
    pub setup_wizard: Option<SetupWizardState>,

    // Navigation history stack (Ctrl-o / Ctrl-i)
    pub history_back: Vec<PathBuf>,
    pub history_forward: Vec<PathBuf>,

    // Leader key state (<Space>)
    pub leader_active: bool,
    pub should_quit: bool,
}

impl App {
    pub fn new(vault: VaultIndex, config: OxidianConfig, is_first_run: bool) -> Self {
        let theme = match config.theme.as_str() {
            "Gruvbox Material" => Theme::gruvbox_material(),
            "Everforest" => Theme::everforest(),
            _ => Theme::catppuccin_mocha(),
        };

        let mut file_tree = FileTreeState::new();
        file_tree.rebuild(&vault);

        let mut bufferline = BufferlineState::new();
        let note_viewer = NoteViewerState::new();
        let mut backlinks = BacklinksState::new();

        // If files exist, open the first note or daily note
        if let Some(first_file) = vault.files.first() {
            bufferline.open_note(first_file.clone());
            let note = vault.notes.get(first_file);
            backlinks.update(note, &vault);
        }

        let setup_wizard = if is_first_run {
            Some(SetupWizardState::new())
        } else {
            None
        };

        Self {
            vault,
            theme,
            config,
            mode: AppMode::Normal,
            active_panel: ActivePanel::Note,
            show_explorer: true,
            show_backlinks: true,
            bufferline,
            file_tree,
            note_viewer,
            backlinks,
            fuzzy_finder: FuzzyFinderState::new(),
            vault_switcher: VaultSwitcherState::new(),
            setup_wizard,
            history_back: Vec::new(),
            history_forward: Vec::new(),
            leader_active: false,
            should_quit: false,
        }
    }

    pub fn current_note(&self) -> Option<&Note> {
        let path = self.bufferline.current_path()?;
        self.vault.notes.get(path)
    }

    pub fn open_note_path(&mut self, path: PathBuf, record_history: bool) {
        if record_history {
            if let Some(curr) = self.bufferline.current_path() {
                self.history_back.push(curr.clone());
                self.history_forward.clear();
            }
        }

        self.bufferline.open_note(path.clone());
        self.note_viewer.reset();
        self.file_tree.select_path(&path, &self.vault);

        let note = self.vault.notes.get(&path);
        self.backlinks.update(note, &self.vault);
    }

    pub fn reload_vault(&mut self, new_root: PathBuf, new_name: String) {
        self.vault = VaultIndex::build(new_root, new_name);
        self.file_tree.rebuild(&self.vault);
        self.bufferline = BufferlineState::new();
        if let Some(first) = self.vault.files.first() {
            self.open_note_path(first.clone(), false);
        }
    }

    pub fn open_daily_note(&mut self) {
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        let filename = format!("{}.md", today);

        // Check if note exists in vault
        if let Some(path) = self.vault.resolve_wikilink(&today) {
            self.open_note_path(path.clone(), true);
            return;
        }

        // Create daily note in root
        let new_path = self.vault.root.join(&filename);
        let content = format!("# Daily Note: {}\n\n- [ ] \n\n## Notes\n\n", today);
        let _ = std::fs::write(&new_path, content);

        // Reload vault to index the new note
        self.reload_vault(self.vault.root.clone(), self.vault.name.clone());
        self.open_note_path(new_path, true);
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        // Handle Setup Wizard if active
        if let Some(ref mut wizard) = self.setup_wizard {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc if wizard.step == SetupStep::SelectVault => {
                    self.should_quit = true;
                }
                KeyCode::Esc | KeyCode::Char('b') => {
                    match wizard.step {
                        SetupStep::SelectVault => { self.should_quit = true; }
                        SetupStep::SelectTheme => { wizard.step = SetupStep::SelectVault; }
                        SetupStep::Confirm => { wizard.step = SetupStep::SelectTheme; }
                    }
                }
                KeyCode::Char('j') | KeyCode::Down => {
                    wizard.next_item();
                    if wizard.step == SetupStep::SelectTheme {
                        self.theme = wizard.selected_theme();
                    }
                }
                KeyCode::Char('k') | KeyCode::Up => {
                    wizard.prev_item();
                    if wizard.step == SetupStep::SelectTheme {
                        self.theme = wizard.selected_theme();
                    }
                }
                KeyCode::Enter | KeyCode::Char(' ') => {
                    match wizard.step {
                        SetupStep::SelectVault => {
                            wizard.step = SetupStep::SelectTheme;
                            self.theme = wizard.selected_theme();
                        }
                        SetupStep::SelectTheme => {
                            wizard.step = SetupStep::Confirm;
                        }
                        SetupStep::Confirm => {
                            if let Ok(cfg) = wizard.save_config() {
                                self.config = cfg;
                                if let Some(path) = wizard.selected_vault() {
                                    let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("Vault").to_string();
                                    self.reload_vault(path, name);
                                }
                            }
                            self.setup_wizard = None;
                        }
                    }
                }
                _ => {}
            }
            return;
        }

        // Handle Active Modals
        match self.mode {
            AppMode::FuzzyFind | AppMode::GrepFind => {
                self.handle_fuzzy_finder_key(key);
                return;
            }
            AppMode::VaultSwitch => {
                self.handle_vault_switcher_key(key);
                return;
            }
            AppMode::Help => {
                if matches!(key.code, KeyCode::Char('?') | KeyCode::Char('q') | KeyCode::Esc | KeyCode::Enter) {
                    self.mode = AppMode::Normal;
                }
                return;
            }
            _ => {}
        }

        // Leader Key (<Space>) sequences
        if self.leader_active {
            self.leader_active = false;
            match key.code {
                KeyCode::Char('f') => {
                    self.mode = AppMode::FuzzyFind;
                    self.fuzzy_finder.open(&self.vault, false);
                }
                KeyCode::Char('g') => {
                    self.mode = AppMode::GrepFind;
                    self.fuzzy_finder.open(&self.vault, true);
                }
                KeyCode::Char('v') => {
                    self.mode = AppMode::VaultSwitch;
                    self.vault_switcher.refresh();
                }
                KeyCode::Char('b') => {
                    self.show_backlinks = !self.show_backlinks;
                }
                KeyCode::Char('e') => {
                    self.show_explorer = !self.show_explorer;
                }
                KeyCode::Char('d') => {
                    self.open_daily_note();
                }
                KeyCode::Char('x') => {
                    if let Some(next_path) = self.bufferline.close_current() {
                        self.open_note_path(next_path, false);
                    }
                }
                _ => {}
            }
            return;
        }

        // Global Normal Keybindings
        match key.code {
            KeyCode::Char(' ') => {
                self.leader_active = true;
                return;
            }
            KeyCode::Char('q') => {
                self.should_quit = true;
                return;
            }
            KeyCode::Char('?') | KeyCode::F(1) => {
                self.mode = AppMode::Help;
                return;
            }
            KeyCode::Char('p') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.mode = AppMode::FuzzyFind;
                self.fuzzy_finder.open(&self.vault, false);
                return;
            }
            KeyCode::Tab => {
                self.bufferline.next_tab();
                if let Some(path) = self.bufferline.current_path().cloned() {
                    self.open_note_path(path, false);
                }
                return;
            }
            KeyCode::BackTab => {
                self.bufferline.prev_tab();
                if let Some(path) = self.bufferline.current_path().cloned() {
                    self.open_note_path(path, false);
                }
                return;
            }
            KeyCode::Char('t') => {
                self.theme = self.theme.cycle_next();
                return;
            }
            KeyCode::Char('o') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                // Jump back in note history
                if let Some(prev) = self.history_back.pop() {
                    if let Some(curr) = self.bufferline.current_path() {
                        self.history_forward.push(curr.clone());
                    }
                    self.open_note_path(prev, false);
                }
                return;
            }
            KeyCode::Char('i') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                // Jump forward in note history
                if let Some(next) = self.history_forward.pop() {
                    if let Some(curr) = self.bufferline.current_path() {
                        self.history_back.push(curr.clone());
                    }
                    self.open_note_path(next, false);
                }
                return;
            }
            KeyCode::Char('e') => {
                // Open current note in real Neovim!
                if let Some(note) = self.current_note() {
                    let path = note.path.clone();
                    let _ = open_in_neovim(&path, &self.config.editor);
                    // Reload note after editing
                    self.reload_vault(self.vault.root.clone(), self.vault.name.clone());
                    self.open_note_path(path, false);
                }
                return;
            }
            _ => {}
        }

        // Panel Switcher: 'h' and 'l'
        match key.code {
            KeyCode::Char('h') if self.active_panel == ActivePanel::Backlinks => {
                self.active_panel = ActivePanel::Note;
                return;
            }
            KeyCode::Char('h') if self.active_panel == ActivePanel::Note && self.show_explorer => {
                self.active_panel = ActivePanel::Explorer;
                return;
            }
            KeyCode::Char('l') if self.active_panel == ActivePanel::Explorer => {
                self.active_panel = ActivePanel::Note;
                return;
            }
            KeyCode::Char('l') if self.active_panel == ActivePanel::Note && self.show_backlinks => {
                self.active_panel = ActivePanel::Backlinks;
                return;
            }
            _ => {}
        }

        // Panel-specific controls
        match self.active_panel {
            ActivePanel::Explorer => {
                match key.code {
                    KeyCode::Char('j') | KeyCode::Down => self.file_tree.next(),
                    KeyCode::Char('k') | KeyCode::Up => self.file_tree.prev(),
                    KeyCode::Enter | KeyCode::Char('o') => {
                        if let Some(path) = self.file_tree.toggle_or_open(&self.vault) {
                            self.open_note_path(path, true);
                            self.active_panel = ActivePanel::Note;
                        }
                    }
                    _ => {}
                }
            }
            ActivePanel::Note => {
                let total_lines = self.current_note().map(|n| n.lines.len()).unwrap_or(0);
                match key.code {
                    KeyCode::Char('j') | KeyCode::Down => self.note_viewer.scroll_down(total_lines, 30),
                    KeyCode::Char('k') | KeyCode::Up => self.note_viewer.scroll_up(),
                    KeyCode::Char('d') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        self.note_viewer.page_down(total_lines, 25);
                    }
                    KeyCode::Char('u') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        self.note_viewer.page_up(25);
                    }
                    KeyCode::Char('g') => {
                        self.note_viewer.goto_top();
                    }
                    KeyCode::Char('G') => {
                        self.note_viewer.goto_bottom(total_lines, 30);
                    }
                    KeyCode::Enter => {
                        // Check if wiki-link under cursor to follow!
                        if let Some(note) = self.current_note() {
                            if let Some(target) = get_link_under_cursor(note, self.note_viewer.cursor_line) {
                                if let Some(target_path) = self.vault.resolve_wikilink(&target) {
                                    let path = target_path.clone();
                                    self.open_note_path(path, true);
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
            ActivePanel::Backlinks => {
                match key.code {
                    KeyCode::Char('j') | KeyCode::Down => self.backlinks.next(),
                    KeyCode::Char('k') | KeyCode::Up => self.backlinks.prev(),
                    KeyCode::Enter => {
                        if let Some(path) = self.backlinks.current_selected().cloned() {
                            self.open_note_path(path, true);
                            self.active_panel = ActivePanel::Note;
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    fn handle_fuzzy_finder_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => {
                self.mode = AppMode::Normal;
            }
            KeyCode::Char('j') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.fuzzy_finder.next();
            }
            KeyCode::Char('k') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.fuzzy_finder.prev();
            }
            KeyCode::Down => self.fuzzy_finder.next(),
            KeyCode::Up => self.fuzzy_finder.prev(),
            KeyCode::Enter => {
                if let Some(selected_path) = self.fuzzy_finder.current_selected() {
                    self.open_note_path(selected_path, true);
                    self.active_panel = ActivePanel::Note;
                }
                self.mode = AppMode::Normal;
            }
            KeyCode::Backspace => {
                self.fuzzy_finder.query.pop();
                self.fuzzy_finder.update_matches(&self.vault);
            }
            KeyCode::Char(c) => {
                self.fuzzy_finder.query.push(c);
                self.fuzzy_finder.update_matches(&self.vault);
            }
            _ => {}
        }
    }

    fn handle_vault_switcher_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc | KeyCode::Char('q') => {
                self.mode = AppMode::Normal;
            }
            KeyCode::Char('j') | KeyCode::Down => self.vault_switcher.next(),
            KeyCode::Char('k') | KeyCode::Up => self.vault_switcher.prev(),
            KeyCode::Enter => {
                if let Some(v) = self.vault_switcher.selected_vault().cloned() {
                    self.reload_vault(v.path, v.name);
                }
                self.mode = AppMode::Normal;
            }
            _ => {}
        }
    }
}
