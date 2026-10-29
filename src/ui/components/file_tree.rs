use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame,
};
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use crate::theme::Theme;
use crate::vault::VaultIndex;

#[derive(Debug, Clone)]
pub struct TreeItem {
    pub path: PathBuf,
    pub name: String,
    pub is_dir: bool,
    pub depth: usize,
    pub is_expanded: bool,
}

pub struct FileTreeState {
    pub items: Vec<TreeItem>,
    pub list_state: ListState,
    pub expanded_dirs: HashSet<PathBuf>,
}

impl FileTreeState {
    pub fn new() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        Self {
            items: Vec::new(),
            list_state,
            expanded_dirs: HashSet::new(),
        }
    }

    pub fn rebuild(&mut self, vault: &VaultIndex) {
        let mut items = Vec::new();
        self.collect_dir(&vault.root, 0, &mut items);
        self.items = items;
        if self.items.is_empty() {
            self.list_state.select(None);
        } else if self.list_state.selected().unwrap_or(0) >= self.items.len() {
            self.list_state.select(Some(self.items.len().saturating_sub(1)));
        }
    }

    fn collect_dir(&self, dir: &Path, depth: usize, items: &mut Vec<TreeItem>) {
        let mut entries = match std::fs::read_dir(dir) {
            Ok(iter) => iter.filter_map(|e| e.ok()).collect::<Vec<_>>(),
            Err(_) => return,
        };

        // Sort: directories first, then alphabetical
        entries.sort_by(|a, b| {
            let a_is_dir = a.file_type().map(|t| t.is_dir()).unwrap_or(false);
            let b_is_dir = b.file_type().map(|t| t.is_dir()).unwrap_or(false);
            if a_is_dir != b_is_dir {
                b_is_dir.cmp(&a_is_dir)
            } else {
                a.file_name().cmp(&b.file_name())
            }
        });

        for entry in entries {
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();

            // Ignore hidden and trash folders
            if name.starts_with('.') || name == "Trash" || name == "node_modules" {
                continue;
            }

            let is_dir = entry.file_type().map(|t| t.is_dir()).unwrap_or(false);

            if is_dir {
                let is_expanded = self.expanded_dirs.contains(&path);
                items.push(TreeItem {
                    path: path.clone(),
                    name,
                    is_dir: true,
                    depth,
                    is_expanded,
                });
                if is_expanded {
                    self.collect_dir(&path, depth + 1, items);
                }
            } else if path.extension().and_then(|s| s.to_str()) == Some("md") {
                items.push(TreeItem {
                    path,
                    name,
                    is_dir: false,
                    depth,
                    is_expanded: false,
                });
            }
        }
    }

    pub fn next(&mut self) {
        if self.items.is_empty() {
            return;
        }
        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
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
        if self.items.is_empty() {
            return;
        }
        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    pub fn toggle_or_open(&mut self, vault: &VaultIndex) -> Option<PathBuf> {
        let idx = self.list_state.selected()?;
        let item = self.items.get(idx)?;
        if item.is_dir {
            let path = item.path.clone();
            if self.expanded_dirs.contains(&path) {
                self.expanded_dirs.remove(&path);
            } else {
                self.expanded_dirs.insert(path);
            }
            self.rebuild(vault);
            None
        } else {
            Some(item.path.clone())
        }
    }

    pub fn select_path(&mut self, target_path: &Path, vault: &VaultIndex) {
        // Expand parent directories if needed
        let mut curr = target_path.parent();
        while let Some(parent) = curr {
            if parent == vault.root {
                break;
            }
            self.expanded_dirs.insert(parent.to_path_buf());
            curr = parent.parent();
        }
        self.rebuild(vault);

        if let Some(pos) = self.items.iter().position(|it| it.path == target_path) {
            self.list_state.select(Some(pos));
        }
    }
}

pub fn render_file_tree(
    f: &mut Frame,
    area: Rect,
    state: &mut FileTreeState,
    theme: &Theme,
    is_focused: bool,
) {
    let border_color = if is_focused {
        theme.border_active
    } else {
        theme.border
    };

    let title_span = Span::styled(
        "  EXPLORER ",
        Style::default()
            .fg(if is_focused { theme.accent } else { theme.fg })
            .add_modifier(Modifier::BOLD),
    );

    let block = Block::default()
        .borders(Borders::ALL)
        .title(title_span)
        .border_style(Style::default().fg(border_color))
        .style(Style::default().bg(theme.bg));

    let items: Vec<ListItem> = state
        .items
        .iter()
        .enumerate()
        .map(|(idx, item)| {
            let is_selected = state.list_state.selected() == Some(idx);
            let indent = "  ".repeat(item.depth);

            let (icon, icon_style) = if item.is_dir {
                if item.is_expanded {
                    (" 󰉖 ", Style::default().fg(theme.primary))
                } else {
                    (" 󰉋 ", Style::default().fg(theme.primary))
                }
            } else {
                ("  󰈙 ", Style::default().fg(theme.info))
            };

            let name_style = if is_selected {
                Style::default()
                    .fg(if is_focused { theme.accent } else { theme.fg })
                    .bg(theme.bg_highlight)
                    .add_modifier(Modifier::BOLD)
            } else if item.is_dir {
                Style::default().fg(theme.fg).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(theme.fg)
            };

            let line = Line::from(vec![
                Span::raw(indent),
                Span::styled(icon, icon_style),
                Span::styled(&item.name, name_style),
            ]);

            ListItem::new(line)
        })
        .collect();

    let list = List::new(items)
        .block(block)
        .highlight_style(Style::default().bg(theme.bg_highlight));

    f.render_stateful_widget(list, area, &mut state.list_state);
}
