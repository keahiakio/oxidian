use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};
use std::path::PathBuf;
use crate::theme::Theme;

pub struct BufferlineState {
    pub open_tabs: Vec<PathBuf>,
    pub active_tab: usize,
}

impl BufferlineState {
    pub fn new() -> Self {
        Self {
            open_tabs: Vec::new(),
            active_tab: 0,
        }
    }

    pub fn open_note(&mut self, path: PathBuf) {
        if let Some(pos) = self.open_tabs.iter().position(|p| p == &path) {
            self.active_tab = pos;
        } else {
            self.open_tabs.push(path);
            self.active_tab = self.open_tabs.len() - 1;
        }
    }

    pub fn close_current(&mut self) -> Option<PathBuf> {
        if self.open_tabs.is_empty() {
            return None;
        }
        self.open_tabs.remove(self.active_tab);
        if self.active_tab >= self.open_tabs.len() && !self.open_tabs.is_empty() {
            self.active_tab = self.open_tabs.len() - 1;
        }
        self.current_path().cloned()
    }

    pub fn next_tab(&mut self) {
        if !self.open_tabs.is_empty() {
            self.active_tab = (self.active_tab + 1) % self.open_tabs.len();
        }
    }

    pub fn prev_tab(&mut self) {
        if !self.open_tabs.is_empty() {
            if self.active_tab == 0 {
                self.active_tab = self.open_tabs.len() - 1;
            } else {
                self.active_tab -= 1;
            }
        }
    }

    pub fn current_path(&self) -> Option<&PathBuf> {
        self.open_tabs.get(self.active_tab)
    }
}

pub fn render_bufferline(
    f: &mut Frame,
    area: Rect,
    state: &BufferlineState,
    theme: &Theme,
    vault_name: &str,
) {
    let mut spans = Vec::new();

    // Vault indicator on far left
    spans.push(Span::styled(
        format!(" 󰉋 {} ", vault_name),
        Style::default()
            .bg(theme.bg_dark)
            .fg(theme.primary)
            .add_modifier(Modifier::BOLD),
    ));
    spans.push(Span::styled("▌", Style::default().fg(theme.border).bg(theme.bg_dark)));

    if state.open_tabs.is_empty() {
        spans.push(Span::styled(
            " [No note opened] ",
            Style::default().fg(theme.fg_muted).bg(theme.bg_dark),
        ));
    } else {
        for (i, path) in state.open_tabs.iter().enumerate() {
            let is_active = i == state.active_tab;
            let file_stem = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("Untitled");

            let (bg, fg, modif) = if is_active {
                (theme.tab_active_bg, theme.tab_active_fg, Modifier::BOLD)
            } else {
                (theme.tab_inactive_bg, theme.tab_inactive_fg, Modifier::empty())
            };

            // Tab prefix separator
            spans.push(Span::styled(
                if is_active { " 󰈙 " } else { "   " },
                Style::default().bg(bg).fg(if is_active { theme.accent } else { theme.fg_muted }),
            ));

            // Note title
            spans.push(Span::styled(
                format!("{} ", file_stem),
                Style::default().bg(bg).fg(fg).add_modifier(modif),
            ));

            // Close hint icon
            spans.push(Span::styled(
                if is_active { "󰅖 " } else { "  " },
                Style::default().bg(bg).fg(theme.fg_muted),
            ));

            // Tab boundary
            spans.push(Span::styled("│", Style::default().fg(theme.border).bg(theme.bg_dark)));
        }
    }

    let p = Paragraph::new(Line::from(spans)).style(Style::default().bg(theme.bg_dark));
    f.render_widget(p, area);
}
