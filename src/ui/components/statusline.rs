use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};
use crate::theme::Theme;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppMode {
    Normal,
    Explorer,
    Backlinks,
    FuzzyFind,
    GrepFind,
    VaultSwitch,
    Help,
}

pub fn render_statusline(
    f: &mut Frame,
    area: Rect,
    mode: AppMode,
    vault_name: &str,
    current_title: Option<&str>,
    word_count: usize,
    backlinks_count: usize,
    cursor_line: usize,
    total_lines: usize,
    theme: &Theme,
) {
    let (mode_str, mode_bg, mode_fg) = match mode {
        AppMode::Normal => (" NORMAL ", theme.mode_normal_bg, theme.mode_normal_fg),
        AppMode::Explorer => (" EXPLORER ", theme.primary, theme.mode_normal_fg),
        AppMode::Backlinks => (" BACKLINKS ", theme.accent, theme.mode_normal_fg),
        AppMode::FuzzyFind => (" FIND-FILE ", theme.secondary, theme.mode_normal_fg),
        AppMode::GrepFind => (" GREP ", theme.warning, theme.mode_normal_fg),
        AppMode::VaultSwitch => (" VAULT ", theme.info, theme.mode_normal_fg),
        AppMode::Help => (" HELP ", theme.danger, theme.mode_normal_fg),
    };

    let title_str = current_title.unwrap_or("No file");
    let progress_pct = if total_lines > 0 {
        ((cursor_line + 1) * 100) / total_lines
    } else {
        100
    };

    let left_spans = vec![
        // Mode pill
        Span::styled(
            mode_str,
            Style::default().bg(mode_bg).fg(mode_fg).add_modifier(Modifier::BOLD),
        ),
        // Vault name
        Span::styled(
            format!(" 󰘬 {} ", vault_name),
            Style::default().bg(theme.bg_highlight).fg(theme.fg).add_modifier(Modifier::BOLD),
        ),
        // File title
        Span::styled(
            format!(" 󰈙 {} ", title_str),
            Style::default().bg(theme.bg_dark).fg(theme.primary),
        ),
    ];

    let right_spans = vec![
        Span::styled(
            format!(" 󰌹 {} ", backlinks_count),
            Style::default().bg(theme.bg_dark).fg(theme.accent),
        ),
        Span::styled(
            format!(" 󰧭 {} words ", word_count),
            Style::default().bg(theme.bg_highlight).fg(theme.fg_muted),
        ),
        Span::styled(
            format!("  {}:{} ({}%) ", cursor_line + 1, total_lines, progress_pct),
            Style::default().bg(theme.tab_active_bg).fg(theme.fg).add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!("  {} ", theme.name()),
            Style::default().bg(mode_bg).fg(mode_fg).add_modifier(Modifier::BOLD),
        ),
    ];

    let left_width: usize = left_spans.iter().map(|s| s.content.chars().count()).sum();
    let right_width: usize = right_spans.iter().map(|s| s.content.chars().count()).sum();

    let total_width = area.width as usize;
    let fill_space = if total_width > left_width + right_width {
        " ".repeat(total_width - left_width - right_width)
    } else {
        " ".to_string()
    };

    let mut all_spans = left_spans;
    all_spans.push(Span::styled(fill_space, Style::default().bg(theme.bg_dark)));
    all_spans.extend(right_spans);

    let p = Paragraph::new(Line::from(all_spans)).style(Style::default().bg(theme.bg_dark));
    f.render_widget(p, area);
}
