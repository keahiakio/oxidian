use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};
use crate::theme::Theme;

pub fn render_help_popup(f: &mut Frame, area: Rect, theme: &Theme) {
    let modal_width = (area.width * 70 / 100).max(60).min(area.width);
    let modal_height = (area.height * 75 / 100).max(18).min(area.height);

    let modal_x = (area.width.saturating_sub(modal_width)) / 2;
    let modal_y = (area.height.saturating_sub(modal_height)) / 2;
    let modal_rect = Rect::new(modal_x, modal_y, modal_width, modal_height);

    f.render_widget(Clear, modal_rect);

    let block = Block::default()
        .borders(Borders::ALL)
        .title(Span::styled(
            " 󰋖 OXIDIAN CHEATSHEET & KEYBINDINGS (?) ",
            Style::default().fg(theme.accent).add_modifier(Modifier::BOLD),
        ))
        .border_style(Style::default().fg(theme.border_active))
        .style(Style::default().bg(theme.bg));

    let inner = block.inner(modal_rect);
    f.render_widget(block, modal_rect);

    let mut lines: Vec<Line<'static>> = Vec::new();

    let mut add_section = |title: &'static str, items: &[(&'static str, &'static str)]| {
        lines.push(Line::from(vec![
            Span::styled(format!("  {} ", title), Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)),
        ]));
        for (key, desc) in items {
            lines.push(Line::from(vec![
                Span::raw("   "),
                Span::styled(format!("{:>14}", key), Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)),
                Span::raw("  "),
                Span::styled(desc.to_string(), Style::default().fg(theme.fg)),
            ]));
        }
        lines.push(Line::from(""));
    };

    add_section("NAVIGATION & PANELS", &[
        ("h / l", "Switch focus between Explorer, Note, and Backlinks"),
        ("j / k or ↑/↓", "Move cursor up/down in current panel"),
        ("gg / G", "Jump to top / bottom of note"),
        ("Ctrl-d / Ctrl-u", "Page down / page up"),
        ("<Space>b", "Toggle Backlinks & Graph panel"),
        ("<Space>e", "Toggle File Explorer panel"),
    ]);

    add_section("FIND & SEARCH (TELESCOPE)", &[
        ("<Space>ff or <C-p>", "Fuzzy find notes in vault"),
        ("<Space>fg", "Live grep note contents across vault"),
        ("<Space>v", "Switch between Obsidian vaults"),
        ("<Space>d", "Open/create today's Daily Note"),
    ]);

    add_section("KNOWLEDGE GRAPH & LINKS", &[
        ("gd or <CR>", "Follow [[wiki-link]] under cursor to target note"),
        ("<C-o> or <BS>", "Jump back to previous note in history"),
        ("<C-i>", "Jump forward in note history"),
        ("Tab / S-Tab", "Next / Previous open note tab"),
        ("<Space>x", "Close current note tab"),
    ]);

    add_section("EDITING & EXTERNAL HANDOFF", &[
        ("e", "Open current note in real Neovim (nvim) & resume on :q"),
        ("t", "Cycle color theme (Catppuccin, Gruvbox, Everforest)"),
        ("? or <F1>", "Toggle this help cheatsheet"),
        ("q or <C-c>", "Quit Oxidian"),
    ]);

    let p = Paragraph::new(lines);
    f.render_widget(p, inner);
}
