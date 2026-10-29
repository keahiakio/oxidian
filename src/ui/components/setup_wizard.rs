use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph},
    Frame,
};
use std::path::PathBuf;
use crate::config::OxidianConfig;
use crate::theme::{Theme, ThemeVariant};
use crate::vault::discovery::{discover_obsidian_vaults, DiscoveredVault};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SetupStep {
    SelectVault,
    SelectTheme,
    Confirm,
}

pub struct SetupWizardState {
    pub step: SetupStep,
    pub discovered_vaults: Vec<DiscoveredVault>,
    pub vault_list_state: ListState,
    pub selected_vault_idx: usize,
    pub theme_idx: usize,
    pub themes: Vec<(&'static str, ThemeVariant)>,
    pub custom_path_input: String,
}

impl SetupWizardState {
    pub fn new() -> Self {
        let vaults = discover_obsidian_vaults();
        let mut vault_list_state = ListState::default();
        if !vaults.is_empty() {
            vault_list_state.select(Some(0));
        }

        let themes = vec![
            ("Catppuccin Mocha (Terminal / Kitty aesthetic)", ThemeVariant::CatppuccinMocha),
            ("Gruvbox Material (Retro groove)", ThemeVariant::GruvboxMaterial),
            ("Everforest (Soft natural green)", ThemeVariant::Everforest),
            ("OneDark (Classic dark)", ThemeVariant::OneDark),
        ];

        Self {
            step: SetupStep::SelectVault,
            discovered_vaults: vaults,
            vault_list_state,
            selected_vault_idx: 0,
            theme_idx: 0,
            themes,
            custom_path_input: String::new(),
        }
    }

    pub fn next_item(&mut self) {
        match self.step {
            SetupStep::SelectVault => {
                let count = self.discovered_vaults.len() + 1; // +1 for custom path
                if count > 0 {
                    self.selected_vault_idx = (self.selected_vault_idx + 1) % count;
                    self.vault_list_state.select(Some(self.selected_vault_idx));
                }
            }
            SetupStep::SelectTheme => {
                self.theme_idx = (self.theme_idx + 1) % self.themes.len();
            }
            SetupStep::Confirm => {}
        }
    }

    pub fn prev_item(&mut self) {
        match self.step {
            SetupStep::SelectVault => {
                let count = self.discovered_vaults.len() + 1;
                if count > 0 {
                    if self.selected_vault_idx == 0 {
                        self.selected_vault_idx = count - 1;
                    } else {
                        self.selected_vault_idx -= 1;
                    }
                    self.vault_list_state.select(Some(self.selected_vault_idx));
                }
            }
            SetupStep::SelectTheme => {
                if self.theme_idx == 0 {
                    self.theme_idx = self.themes.len() - 1;
                } else {
                    self.theme_idx -= 1;
                }
            }
            SetupStep::Confirm => {}
        }
    }

    pub fn selected_vault(&self) -> Option<PathBuf> {
        if self.selected_vault_idx < self.discovered_vaults.len() {
            Some(self.discovered_vaults[self.selected_vault_idx].path.clone())
        } else if !self.custom_path_input.is_empty() {
            Some(PathBuf::from(&self.custom_path_input))
        } else {
            dirs::home_dir().map(|h| h.join("Documents").join("punkeahi_obsidian"))
        }
    }

    pub fn selected_theme(&self) -> Theme {
        match self.themes[self.theme_idx].1 {
            ThemeVariant::CatppuccinMocha => Theme::catppuccin_mocha(),
            ThemeVariant::GruvboxMaterial => Theme::gruvbox_material(),
            ThemeVariant::Everforest => Theme::everforest(),
            ThemeVariant::OneDark => Theme::catppuccin_mocha(),
        }
    }

    pub fn save_config(&self) -> Result<OxidianConfig, std::io::Error> {
        let vault_path = self.selected_vault()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_default();

        let cfg = OxidianConfig {
            default_vault_path: Some(vault_path),
            theme: self.themes[self.theme_idx].0.split('(').next().unwrap().trim().to_string(),
            editor: "nvim".to_string(),
            show_backlinks: true,
            relative_line_numbers: true,
        };

        cfg.save()?;
        Ok(cfg)
    }
}

pub fn render_setup_wizard(
    f: &mut Frame,
    area: Rect,
    state: &mut SetupWizardState,
    theme: &Theme,
) {
    let modal_width = (area.width * 80 / 100).max(65).min(area.width);
    let modal_height = (area.height * 85 / 100).max(22).min(area.height);

    let modal_x = (area.width.saturating_sub(modal_width)) / 2;
    let modal_y = (area.height.saturating_sub(modal_height)) / 2;
    let modal_rect = Rect::new(modal_x, modal_y, modal_width, modal_height);

    f.render_widget(Clear, modal_rect);

    let main_block = Block::default()
        .borders(Borders::ALL)
        .title(Span::styled(
            " 󰋜 OXIDIAN FIRST-TIME SETUP 󰋜 ",
            Style::default().fg(theme.accent).add_modifier(Modifier::BOLD),
        ))
        .border_style(Style::default().fg(theme.border_active))
        .style(Style::default().bg(theme.bg));

    let inner_rect = main_block.inner(modal_rect);
    f.render_widget(main_block, modal_rect);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(7), // ASCII Art banner
            Constraint::Length(3), // Progress breadcrumb
            Constraint::Min(8),    // Step content
            Constraint::Length(3), // Navigation instruction footer
        ])
        .split(inner_rect);

    // 1. ASCII Art Banner
    let banner_lines = vec![
        Line::from(Span::styled("   ██████╗ ██╗  ██╗██╗██████╗ ██╗ █████╗ ███╗   ██╗", Style::default().fg(theme.accent).add_modifier(Modifier::BOLD))),
        Line::from(Span::styled("  ██╔═══██╗╚██╗██╔╝██║██╔══██╗██║██╔══██╗████╗  ██║", Style::default().fg(theme.primary))),
        Line::from(Span::styled("  ██║   ██║ ╚███╔╝ ██║██║  ██║██║███████║██╔██╗ ██║", Style::default().fg(theme.info))),
        Line::from(Span::styled("  ██║   ██║ ██╔██╗ ██║██║  ██║██║██╔══██║██║╚██╗██║", Style::default().fg(theme.success))),
        Line::from(Span::styled("  ╚██████╔╝██╔╝ ██╗██║██████╔╝██║██║  ██║██║ ╚████║", Style::default().fg(theme.secondary))),
        Line::from(Span::styled("   Obsidian CLI & Neovim TUI in Rust", Style::default().fg(theme.fg_muted))),
    ];
    let banner_p = Paragraph::new(banner_lines);
    f.render_widget(banner_p, chunks[0]);

    // 2. Step Breadcrumb
    let step_spans = match state.step {
        SetupStep::SelectVault => vec![
            Span::styled(" [1. Select Vault] ", Style::default().bg(theme.primary).fg(theme.bg_dark).add_modifier(Modifier::BOLD)),
            Span::styled(" 󰁕 2. Choose Theme 󰁕 3. Confirm", Style::default().fg(theme.fg_muted)),
        ],
        SetupStep::SelectTheme => vec![
            Span::styled(" 1. Select Vault 󰁕 ", Style::default().fg(theme.success)),
            Span::styled(" [2. Choose Theme] ", Style::default().bg(theme.accent).fg(theme.bg_dark).add_modifier(Modifier::BOLD)),
            Span::styled(" 󰁕 3. Confirm", Style::default().fg(theme.fg_muted)),
        ],
        SetupStep::Confirm => vec![
            Span::styled(" 1. Select Vault 󰁕 2. Choose Theme 󰁕 ", Style::default().fg(theme.success)),
            Span::styled(" [3. Confirm & Launch] ", Style::default().bg(theme.success).fg(theme.bg_dark).add_modifier(Modifier::BOLD)),
        ],
    };
    let step_p = Paragraph::new(Line::from(step_spans));
    f.render_widget(step_p, chunks[1]);

    // 3. Step Content
    match state.step {
        SetupStep::SelectVault => {
            let mut items = Vec::new();
            for (idx, v) in state.discovered_vaults.iter().enumerate() {
                let is_sel = state.selected_vault_idx == idx;
                let bullet = if is_sel { " 󰄲 " } else { " 󰄱 " };
                let style = if is_sel {
                    Style::default().fg(theme.accent).bg(theme.bg_highlight).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(theme.fg)
                };

                let line = Line::from(vec![
                    Span::styled(bullet, Style::default().fg(theme.primary)),
                    Span::styled(format!("󰉋 {} ", v.name), style),
                    Span::styled(format!("({})", v.path.display()), Style::default().fg(theme.fg_muted)),
                ]);
                items.push(ListItem::new(line));
            }

            // Custom Path item
            let is_custom_sel = state.selected_vault_idx == state.discovered_vaults.len();
            let custom_bullet = if is_custom_sel { " 󰄲 " } else { " 󰄱 " };
            let custom_style = if is_custom_sel {
                Style::default().fg(theme.accent).bg(theme.bg_highlight).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(theme.fg_muted)
            };
            items.push(ListItem::new(Line::from(vec![
                Span::styled(custom_bullet, Style::default().fg(theme.primary)),
                Span::styled("󰉖 Specify custom vault path...", custom_style),
            ])));

            let vault_block = Block::default()
                .borders(Borders::ALL)
                .title(" 󰈙 Detected Installed Obsidian Vaults ")
                .border_style(Style::default().fg(theme.border));

            let list = List::new(items)
                .block(vault_block)
                .highlight_style(Style::default().bg(theme.bg_highlight));

            f.render_stateful_widget(list, chunks[2], &mut state.vault_list_state);
        }
        SetupStep::SelectTheme => {
            let items: Vec<ListItem> = state
                .themes
                .iter()
                .enumerate()
                .map(|(idx, (name, _var))| {
                    let is_sel = state.theme_idx == idx;
                    let bullet = if is_sel { " 󰄲 " } else { " 󰄱 " };
                    let style = if is_sel {
                        Style::default().fg(theme.accent).bg(theme.bg_highlight).add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(theme.fg)
                    };

                    let line = Line::from(vec![
                        Span::styled(bullet, Style::default().fg(theme.accent)),
                        Span::styled(format!(" {} ", name), style),
                    ]);
                    ListItem::new(line)
                })
                .collect();

            let theme_block = Block::default()
                .borders(Borders::ALL)
                .title("  Select Aesthetic Theme (Matches Neovim & Kitty) ")
                .border_style(Style::default().fg(theme.border));

            let list = List::new(items).block(theme_block);
            f.render_widget(list, chunks[2]);
        }
        SetupStep::Confirm => {
            let vault_path = state.selected_vault()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|| "None".to_string());
            let theme_name = state.themes[state.theme_idx].0;

            let confirm_lines = vec![
                Line::from(vec![
                    Span::styled("  Default Vault:  ", Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)),
                    Span::styled(vault_path, Style::default().fg(theme.fg)),
                ]),
                Line::from(""),
                Line::from(vec![
                    Span::styled("  UI Theme:       ", Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)),
                    Span::styled(theme_name, Style::default().fg(theme.fg)),
                ]),
                Line::from(""),
                Line::from(vec![
                    Span::styled("  Editor:         ", Style::default().fg(theme.success).add_modifier(Modifier::BOLD)),
                    Span::styled("nvim (Uses your ~/.config/nvim)", Style::default().fg(theme.fg)),
                ]),
                Line::from(""),
                Line::from(vec![
                    Span::styled("  Config Path:    ", Style::default().fg(theme.info).add_modifier(Modifier::BOLD)),
                    Span::styled(OxidianConfig::config_path().display().to_string(), Style::default().fg(theme.fg_muted)),
                ]),
                Line::from(""),
                Line::from(Span::styled("  Press <Enter> to save configuration and start Oxidian!", Style::default().fg(theme.success).add_modifier(Modifier::BOLD))),
            ];

            let confirm_block = Block::default()
                .borders(Borders::ALL)
                .title(" 󰄬 Ready to Launch ")
                .border_style(Style::default().fg(theme.border));

            let p = Paragraph::new(confirm_lines).block(confirm_block);
            f.render_widget(p, chunks[2]);
        }
    }

    // 4. Navigation instructions footer
    let footer_text = match state.step {
        SetupStep::SelectVault => " [j/k or ↑/↓]: Navigate  |  [Enter/Space]: Select Vault & Continue  |  [q]: Quit",
        SetupStep::SelectTheme => " [j/k or ↑/↓]: Choose Theme  |  [Enter]: Confirm Theme  |  [Esc/b]: Back",
        SetupStep::Confirm => " [Enter]: Launch Oxidian!  |  [Esc/b]: Back to Themes  |  [q]: Quit",
    };

    let footer_p = Paragraph::new(Line::from(Span::styled(
        footer_text,
        Style::default().fg(theme.fg_muted).bg(theme.bg_dark),
    )));
    f.render_widget(footer_p, chunks[3]);
}
