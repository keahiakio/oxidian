use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph},
    Frame,
};
use std::path::PathBuf;
use crate::theme::Theme;
use crate::vault::VaultIndex;

pub struct FuzzyFinderState {
    pub query: String,
    pub matches: Vec<(PathBuf, i64)>, // path and score
    pub list_state: ListState,
    pub is_grep: bool,
}

impl FuzzyFinderState {
    pub fn new() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        Self {
            query: String::new(),
            matches: Vec::new(),
            list_state,
            is_grep: false,
        }
    }

    pub fn open(&mut self, vault: &VaultIndex, is_grep: bool) {
        self.query.clear();
        self.is_grep = is_grep;
        self.update_matches(vault);
    }

    pub fn update_matches(&mut self, vault: &VaultIndex) {
        let matcher = SkimMatcherV2::default();
        let mut results = Vec::new();

        if self.query.is_empty() {
            for path in &vault.files {
                results.push((path.clone(), 0));
            }
        } else if self.is_grep {
            let q_lower = self.query.to_lowercase();
            for (path, note) in &vault.notes {
                if let Some(score) = matcher.fuzzy_match(&note.raw_content, &q_lower) {
                    results.push((path.clone(), score));
                }
            }
            results.sort_by(|a, b| b.1.cmp(&a.1));
        } else {
            for path in &vault.files {
                let note = vault.notes.get(path);
                let text_to_match = if let Some(n) = note {
                    format!("{} {}", n.title, n.rel_path.display())
                } else {
                    path.to_string_lossy().to_string()
                };

                if let Some(score) = matcher.fuzzy_match(&text_to_match, &self.query) {
                    results.push((path.clone(), score));
                }
            }
            results.sort_by(|a, b| b.1.cmp(&a.1));
        }

        self.matches = results;
        if self.matches.is_empty() {
            self.list_state.select(None);
        } else {
            self.list_state.select(Some(0));
        }
    }

    pub fn next(&mut self) {
        if self.matches.is_empty() {
            return;
        }
        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= self.matches.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    pub fn prev(&mut self) {
        if self.matches.is_empty() {
            return;
        }
        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.matches.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    pub fn current_selected(&self) -> Option<PathBuf> {
        let idx = self.list_state.selected()?;
        self.matches.get(idx).map(|m| m.0.clone())
    }
}

pub fn render_fuzzy_finder(
    f: &mut Frame,
    area: Rect,
    state: &mut FuzzyFinderState,
    vault: &VaultIndex,
    theme: &Theme,
) {
    // Center popup modal
    let popup_width = (area.width * 80 / 100).max(60).min(area.width);
    let popup_height = (area.height * 75 / 100).max(18).min(area.height);

    let popup_x = (area.width.saturating_sub(popup_width)) / 2;
    let popup_y = (area.height.saturating_sub(popup_height)) / 2;
    let popup_rect = Rect::new(popup_x, popup_y, popup_width, popup_height);

    f.render_widget(Clear, popup_rect);

    let title_text = if state.is_grep {
        format!(" 󰈞 LIVE GREP ({} matches) ", state.matches.len())
    } else {
        format!("  TELESCOPE FIND FILE ({} notes) ", state.matches.len())
    };

    let main_block = Block::default()
        .borders(Borders::ALL)
        .title(Span::styled(
            title_text,
            Style::default().fg(theme.accent).add_modifier(Modifier::BOLD),
        ))
        .border_style(Style::default().fg(theme.border_active))
        .style(Style::default().bg(theme.bg));

    let inner_rect = main_block.inner(popup_rect);
    f.render_widget(main_block, popup_rect);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Query input box
            Constraint::Min(5),    // Split list & preview
        ])
        .split(inner_rect);

    // Input box
    let input_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.primary))
        .style(Style::default().bg(theme.bg_dark));

    let input_inner = input_block.inner(chunks[0]);
    f.render_widget(input_block, chunks[0]);

    let input_p = Paragraph::new(Line::from(vec![
        Span::styled(" ❯ ", Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)),
        Span::styled(&state.query, Style::default().fg(theme.fg).add_modifier(Modifier::BOLD)),
        Span::styled("█", Style::default().fg(theme.primary)),
    ]));
    f.render_widget(input_p, input_inner);

    // Split bottom into [Matches List (45%)] and [Note Preview (55%)]
    let content_split = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(45), Constraint::Percentage(55)])
        .split(chunks[1]);

    // Matches List
    let items: Vec<ListItem> = state
        .matches
        .iter()
        .enumerate()
        .map(|(idx, (path, _score))| {
            let is_sel = state.list_state.selected() == Some(idx);
            let note = vault.notes.get(path);
            let title = note.map(|n| n.title.as_str()).unwrap_or("Untitled");
            let rel = note
                .map(|n| n.rel_path.display().to_string())
                .unwrap_or_else(|| path.display().to_string());

            let style = if is_sel {
                Style::default()
                    .bg(theme.bg_highlight)
                    .fg(theme.accent)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(theme.fg)
            };

            let prefix = if is_sel { " 󰁕 󰈙 " } else { "   󰈙 " };

            let line = Line::from(vec![
                Span::styled(prefix, Style::default().fg(theme.primary)),
                Span::styled(title, style),
                Span::styled(format!(" ({})", rel), Style::default().fg(theme.fg_muted)),
            ]);

            ListItem::new(line)
        })
        .collect();

    let list_block = Block::default()
        .borders(Borders::RIGHT)
        .border_style(Style::default().fg(theme.border));

    let list = List::new(items)
        .block(list_block)
        .highlight_style(Style::default().bg(theme.bg_highlight));

    f.render_stateful_widget(list, content_split[0], &mut state.list_state);

    // Note Preview
    let preview_block = Block::default()
        .borders(Borders::NONE)
        .style(Style::default().bg(theme.bg_dark));

    let preview_inner = preview_block.inner(content_split[1]);
    f.render_widget(preview_block, content_split[1]);

    if let Some(selected_path) = state.current_selected() {
        if let Some(note) = vault.notes.get(&selected_path) {
            let mut preview_lines = Vec::new();
            preview_lines.push(Line::from(vec![
                Span::styled(format!(" # {} ", note.title), Style::default().fg(theme.heading1).add_modifier(Modifier::BOLD)),
            ]));
            preview_lines.push(Line::from(Span::styled(
                format!(" Path: {} | Words: {}", note.rel_path.display(), note.word_count),
                Style::default().fg(theme.fg_muted),
            )));
            preview_lines.push(Line::from(Span::styled(" ─────────────────────────────────", Style::default().fg(theme.border))));

            for line in note.raw_content.lines().take(preview_inner.height as usize) {
                preview_lines.push(Line::from(vec![
                    Span::raw(" "),
                    Span::styled(line, Style::default().fg(theme.fg)),
                ]));
            }

            let p = Paragraph::new(preview_lines);
            f.render_widget(p, preview_inner);
        }
    }
}
