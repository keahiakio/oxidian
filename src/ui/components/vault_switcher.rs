use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, ListState},
    Frame,
};
use crate::theme::Theme;
use crate::vault::discovery::{discover_obsidian_vaults, DiscoveredVault};

pub struct VaultSwitcherState {
    pub vaults: Vec<DiscoveredVault>,
    pub list_state: ListState,
}

impl VaultSwitcherState {
    pub fn new() -> Self {
        let vaults = discover_obsidian_vaults();
        let mut list_state = ListState::default();
        if !vaults.is_empty() {
            list_state.select(Some(0));
        }
        Self { vaults, list_state }
    }

    pub fn refresh(&mut self) {
        self.vaults = discover_obsidian_vaults();
        if !self.vaults.is_empty() && self.list_state.selected().is_none() {
            self.list_state.select(Some(0));
        }
    }

    pub fn next(&mut self) {
        if self.vaults.is_empty() {
            return;
        }
        let i = match self.list_state.selected() {
            Some(i) => (i + 1) % self.vaults.len(),
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    pub fn prev(&mut self) {
        if self.vaults.is_empty() {
            return;
        }
        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.vaults.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    pub fn selected_vault(&self) -> Option<&DiscoveredVault> {
        let idx = self.list_state.selected()?;
        self.vaults.get(idx)
    }
}

pub fn render_vault_switcher(
    f: &mut Frame,
    area: Rect,
    state: &mut VaultSwitcherState,
    current_vault_path: &std::path::Path,
    theme: &Theme,
) {
    let modal_width = (area.width * 60 / 100).max(50).min(area.width);
    let modal_height = (area.height * 50 / 100).max(12).min(area.height);

    let modal_x = (area.width.saturating_sub(modal_width)) / 2;
    let modal_y = (area.height.saturating_sub(modal_height)) / 2;
    let modal_rect = Rect::new(modal_x, modal_y, modal_width, modal_height);

    f.render_widget(Clear, modal_rect);

    let block = Block::default()
        .borders(Borders::ALL)
        .title(Span::styled(
            " 󰉋 SWITCH OBSIDIAN VAULT (<Space>v) ",
            Style::default().fg(theme.accent).add_modifier(Modifier::BOLD),
        ))
        .border_style(Style::default().fg(theme.border_active))
        .style(Style::default().bg(theme.bg));

    let items: Vec<ListItem> = state
        .vaults
        .iter()
        .enumerate()
        .map(|(idx, v)| {
            let is_selected = state.list_state.selected() == Some(idx);
            let is_current = v.path == current_vault_path;

            let prefix = if is_selected { " 󰁕 " } else { "   " };
            let current_marker = if is_current { " [Active]" } else { "" };

            let name_style = if is_selected {
                Style::default()
                    .fg(theme.accent)
                    .bg(theme.bg_highlight)
                    .add_modifier(Modifier::BOLD)
            } else if is_current {
                Style::default().fg(theme.success).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(theme.fg)
            };

            let line = Line::from(vec![
                Span::raw(prefix),
                Span::styled("󰉋 ", Style::default().fg(theme.primary)),
                Span::styled(&v.name, name_style),
                Span::styled(current_marker, Style::default().fg(theme.success)),
                Span::styled(format!(" ({})", v.path.display()), Style::default().fg(theme.fg_muted)),
            ]);

            ListItem::new(line)
        })
        .collect();

    let list = List::new(items)
        .block(block)
        .highlight_style(Style::default().bg(theme.bg_highlight));

    f.render_stateful_widget(list, modal_rect, &mut state.list_state);
}
