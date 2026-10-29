use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use std::path::PathBuf;
use crate::theme::Theme;
use crate::vault::{Note, VaultIndex};

pub struct BacklinksState {
    pub selected_idx: usize,
    pub items: Vec<PathBuf>,
}

impl BacklinksState {
    pub fn new() -> Self {
        Self {
            selected_idx: 0,
            items: Vec::new(),
        }
    }

    pub fn update(&mut self, current_note: Option<&Note>, vault: &VaultIndex) {
        self.items.clear();
        if let Some(note) = current_note {
            let backlinks = vault.get_backlinks(&note.path);
            for bl in backlinks {
                self.items.push(bl.path.clone());
            }
        }
        if self.items.is_empty() {
            self.selected_idx = 0;
        } else if self.selected_idx >= self.items.len() {
            self.selected_idx = self.items.len() - 1;
        }
    }

    pub fn next(&mut self) {
        if !self.items.is_empty() {
            self.selected_idx = (self.selected_idx + 1) % self.items.len();
        }
    }

    pub fn prev(&mut self) {
        if !self.items.is_empty() {
            if self.selected_idx == 0 {
                self.selected_idx = self.items.len() - 1;
            } else {
                self.selected_idx -= 1;
            }
        }
    }

    pub fn current_selected(&self) -> Option<&PathBuf> {
        self.items.get(self.selected_idx)
    }
}

pub fn render_backlinks_panel(
    f: &mut Frame,
    area: Rect,
    current_note: Option<&Note>,
    vault: &VaultIndex,
    state: &BacklinksState,
    theme: &Theme,
    is_focused: bool,
) {
    let border_color = if is_focused {
        theme.border_active
    } else {
        theme.border
    };

    let title_span = Span::styled(
        " 󰌹 BACKLINKS & GRAPH ",
        Style::default()
            .fg(if is_focused { theme.accent } else { theme.fg })
            .add_modifier(Modifier::BOLD),
    );

    let block = Block::default()
        .borders(Borders::ALL)
        .title(title_span)
        .border_style(Style::default().fg(border_color))
        .style(Style::default().bg(theme.bg));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let note = match current_note {
        Some(n) => n,
        None => return,
    };

    let mut lines = Vec::new();

    // 1. Tags section
    if !note.tags.is_empty() {
        lines.push(Line::from(vec![
            Span::styled(" 󰓹 Tags", Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)),
        ]));
        let mut tag_spans = vec![Span::raw("  ")];
        for tag in &note.tags {
            tag_spans.push(Span::styled(
                format!("#{} ", tag),
                Style::default().fg(theme.tag).add_modifier(Modifier::BOLD),
            ));
        }
        lines.push(Line::from(tag_spans));
        lines.push(Line::from(""));
    }

    // 2. Incoming Backlinks
    lines.push(Line::from(vec![
        Span::styled(
            format!(" 󰌹 Incoming ({})", state.items.len()),
            Style::default().fg(theme.heading2).add_modifier(Modifier::BOLD),
        ),
    ]));

    if state.items.is_empty() {
        lines.push(Line::from(Span::styled(
            "   (No backlinks found)",
            Style::default().fg(theme.fg_muted),
        )));
    } else {
        for (i, path) in state.items.iter().enumerate() {
            let is_selected = i == state.selected_idx;
            let title = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("Untitled");

            let style = if is_selected {
                Style::default()
                    .fg(if is_focused { theme.accent } else { theme.fg })
                    .bg(theme.bg_highlight)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(theme.link)
            };

            lines.push(Line::from(vec![
                Span::raw(if is_selected { " 󰁕 " } else { "   " }),
                Span::styled(format!("󰈙 {}", title), style),
            ]));
        }
    }

    lines.push(Line::from(""));

    // 3. Outgoing Links
    lines.push(Line::from(vec![
        Span::styled(
            format!(" 󰌷 Outgoing ({})", note.outgoing_links.len()),
            Style::default().fg(theme.heading3).add_modifier(Modifier::BOLD),
        ),
    ]));

    if note.outgoing_links.is_empty() {
        lines.push(Line::from(Span::styled(
            "   (No outgoing links)",
            Style::default().fg(theme.fg_muted),
        )));
    } else {
        for link in note.outgoing_links.iter().take(15) {
            let display = link.alias.as_deref().unwrap_or(&link.target);
            let exists = vault.resolve_wikilink(&link.target).is_some();

            let style = if exists {
                Style::default().fg(theme.fg)
            } else {
                Style::default().fg(theme.danger) // unresolved / phantom note
            };

            lines.push(Line::from(vec![
                Span::raw("   "),
                Span::styled(if exists { "󰈙 " } else { "󱄊 " }, style),
                Span::styled(display, style),
            ]));
        }
    }

    let p = Paragraph::new(lines);
    f.render_widget(p, inner);
}
