use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use std::cmp::min;
use crate::theme::Theme;
use crate::vault::{LineKind, Note};

pub struct NoteViewerState {
    pub scroll_y: usize,
    pub cursor_line: usize,
    pub selected_link_idx: Option<usize>,
}

impl NoteViewerState {
    pub fn new() -> Self {
        Self {
            scroll_y: 0,
            cursor_line: 0,
            selected_link_idx: None,
        }
    }

    pub fn reset(&mut self) {
        self.scroll_y = 0;
        self.cursor_line = 0;
        self.selected_link_idx = None;
    }

    pub fn scroll_down(&mut self, total_lines: usize, viewport_height: usize) {
        if total_lines == 0 {
            return;
        }
        if self.cursor_line < total_lines - 1 {
            self.cursor_line += 1;
            if self.cursor_line >= self.scroll_y + viewport_height {
                self.scroll_y = self.cursor_line - viewport_height + 1;
            }
        }
    }

    pub fn scroll_up(&mut self) {
        if self.cursor_line > 0 {
            self.cursor_line -= 1;
            if self.cursor_line < self.scroll_y {
                self.scroll_y = self.cursor_line;
            }
        }
    }

    pub fn page_down(&mut self, total_lines: usize, viewport_height: usize) {
        let step = viewport_height.saturating_sub(2).max(1);
        self.cursor_line = min(self.cursor_line + step, total_lines.saturating_sub(1));
        if self.cursor_line >= self.scroll_y + viewport_height {
            self.scroll_y = self.cursor_line - viewport_height + 1;
        }
    }

    pub fn page_up(&mut self, viewport_height: usize) {
        let step = viewport_height.saturating_sub(2).max(1);
        self.cursor_line = self.cursor_line.saturating_sub(step);
        if self.cursor_line < self.scroll_y {
            self.scroll_y = self.cursor_line;
        }
    }

    pub fn goto_top(&mut self) {
        self.cursor_line = 0;
        self.scroll_y = 0;
    }

    pub fn goto_bottom(&mut self, total_lines: usize, viewport_height: usize) {
        if total_lines > 0 {
            self.cursor_line = total_lines - 1;
            if total_lines > viewport_height {
                self.scroll_y = total_lines - viewport_height;
            }
        }
    }
}

pub fn render_note_viewer(
    f: &mut Frame,
    area: Rect,
    note: Option<&Note>,
    state: &mut NoteViewerState,
    theme: &Theme,
    is_focused: bool,
) {
    let border_color = if is_focused {
        theme.border_active
    } else {
        theme.border
    };

    let title_text = if let Some(n) = note {
        format!(" 󰈙 {} ", n.title)
    } else {
        " 󰈙 No Note Selected ".to_string()
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .title(Span::styled(
            title_text,
            Style::default()
                .fg(if is_focused { theme.accent } else { theme.fg })
                .add_modifier(Modifier::BOLD),
        ))
        .border_style(Style::default().fg(border_color))
        .style(Style::default().bg(theme.bg));

    let inner_area = block.inner(area);
    f.render_widget(block, area);

    let note = match note {
        Some(n) => n,
        None => {
            let p = Paragraph::new(vec![
                Line::from(""),
                Line::from(Span::styled(
                    "   Select a note from the explorer or press <Space>ff to fuzzy find.",
                    Style::default().fg(theme.fg_muted),
                )),
            ]);
            f.render_widget(p, inner_area);
            return;
        }
    };

    let total_lines = note.lines.len();
    let viewport_height = inner_area.height as usize;
    if viewport_height == 0 {
        return;
    }

    // Ensure cursor is within bounds
    if total_lines > 0 && state.cursor_line >= total_lines {
        state.cursor_line = total_lines - 1;
    }

    let start_idx = state.scroll_y;
    let end_idx = min(start_idx + viewport_height, total_lines);

    let mut rendered_lines = Vec::new();
    let line_num_width = format!("{}", total_lines).len().max(2);

    for (rel_idx, line_kind) in note.lines[start_idx..end_idx].iter().enumerate() {
        let abs_idx = start_idx + rel_idx;
        let is_cursor = abs_idx == state.cursor_line && is_focused;

        let line_num_str = format!("{:>width$} ", abs_idx + 1, width = line_num_width);
        let num_style = if is_cursor {
            Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(theme.fg_muted)
        };

        let mut spans = vec![
            Span::styled(line_num_str, num_style),
            Span::styled("│ ", Style::default().fg(theme.border)),
        ];

        let line_bg = if is_cursor {
            theme.bg_highlight
        } else {
            theme.bg
        };

        match line_kind {
            LineKind::Heading(level, text) => {
                let (prefix, color) = match level {
                    1 => ("󰉫 # ", theme.heading1),
                    2 => ("󰉬 ## ", theme.heading2),
                    3 => ("󰉭 ### ", theme.heading3),
                    _ => ("󰉮 #### ", theme.heading4),
                };
                spans.push(Span::styled(
                    prefix,
                    Style::default().fg(color).add_modifier(Modifier::BOLD),
                ));
                spans.extend(render_inline_spans(text, theme, color, true));
            }
            LineKind::Task(is_done, text) => {
                if *is_done {
                    spans.push(Span::styled("󰄲 ", Style::default().fg(theme.task_done)));
                    spans.extend(render_inline_spans(text, theme, theme.fg_muted, false));
                } else {
                    spans.push(Span::styled("󰄱 ", Style::default().fg(theme.task_todo)));
                    spans.extend(render_inline_spans(text, theme, theme.fg, false));
                }
            }
            LineKind::Bullet(indent, text) => {
                let pad = "  ".repeat(*indent as usize);
                let bullet_icon = match indent % 3 {
                    0 => "● ",
                    1 => "○ ",
                    _ => "◆ ",
                };
                spans.push(Span::raw(pad));
                spans.push(Span::styled(bullet_icon, Style::default().fg(theme.primary)));
                spans.extend(render_inline_spans(text, theme, theme.fg, false));
            }
            LineKind::CalloutHeader(kind, title) => {
                let (icon, color) = match kind.as_str() {
                    "NOTE" => ("󰋽 NOTE", theme.info),
                    "TIP" => ("󰌶 TIP", theme.success),
                    "WARNING" | "CAUTION" => ("󰀪 WARNING", theme.warning),
                    "IMPORTANT" | "DANGER" => ("󰳦 IMPORTANT", theme.danger),
                    _ => ("󰋽 INFO", theme.accent),
                };
                spans.push(Span::styled(
                    format!(" ▌ {} ", icon),
                    Style::default().fg(color).bg(theme.bg_dark).add_modifier(Modifier::BOLD),
                ));
                if !title.is_empty() {
                    spans.push(Span::styled(
                        format!("{} ", title),
                        Style::default().fg(theme.fg).bg(theme.bg_dark),
                    ));
                }
            }
            LineKind::Blockquote(text) => {
                spans.push(Span::styled("▎ ", Style::default().fg(theme.secondary)));
                spans.extend(render_inline_spans(text, theme, theme.fg_muted, false));
            }
            LineKind::CodeFence(lang) => {
                spans.push(Span::styled(
                    format!("``` {}", lang),
                    Style::default().fg(theme.fg_muted).bg(theme.code_bg),
                ));
            }
            LineKind::CodeLine(text) => {
                spans.push(Span::styled(
                    format!("  {} ", text),
                    Style::default().fg(theme.code_fg).bg(theme.code_bg),
                ));
            }
            LineKind::Divider => {
                spans.push(Span::styled(
                    "────────────────────────────────────────────────────────",
                    Style::default().fg(theme.border),
                ));
            }
            LineKind::Normal(text) => {
                spans.extend(render_inline_spans(text, theme, theme.fg, false));
            }
            LineKind::Numbered(indent, num, text) => {
                let pad = "  ".repeat(*indent as usize);
                spans.push(Span::raw(pad));
                spans.push(Span::styled(
                    format!("{}. ", num),
                    Style::default().fg(theme.primary),
                ));
                spans.extend(render_inline_spans(text, theme, theme.fg, false));
            }
        }

        let line = Line::from(spans).style(Style::default().bg(line_bg));
        rendered_lines.push(line);
    }

    let p = Paragraph::new(rendered_lines);
    f.render_widget(p, inner_area);
}

/// Parses inline elements: `[[wiki-links]]` and `#tags`
fn render_inline_spans<'a>(
    text: &'a str,
    theme: &Theme,
    base_color: Color,
    is_bold: bool,
) -> Vec<Span<'a>> {
    let mut spans = Vec::new();
    let mut curr_idx = 0;

    let base_style = {
        let mut s = Style::default().fg(base_color);
        if is_bold {
            s = s.add_modifier(Modifier::BOLD);
        }
        s
    };

    while curr_idx < text.len() {
        let remaining = &text[curr_idx..];

        // 1. Check for wikilink: [[Target|Alias]]
        if let Some(open_idx) = remaining.find("[[") {
            if let Some(close_idx) = remaining[open_idx + 2..].find("]]") {
                let abs_open = curr_idx + open_idx;
                let abs_close = abs_open + 2 + close_idx + 2;

                // Push text before link
                if open_idx > 0 {
                    spans.push(Span::styled(&text[curr_idx..abs_open], base_style));
                }

                let link_content = &text[abs_open + 2..abs_open + 2 + close_idx];
                let (_target, display) = if let Some(pipe) = link_content.find('|') {
                    (&link_content[..pipe], &link_content[pipe + 1..])
                } else {
                    (link_content, link_content)
                };

                // Stylize wikilink
                spans.push(Span::styled(
                    format!("󰌹 [[{}]]", display),
                    Style::default()
                        .fg(theme.link)
                        .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
                ));

                curr_idx = abs_close;
                continue;
            }
        }

        // 2. Check for tags: #tag
        if let Some(hash_idx) = remaining.find('#') {
            let abs_hash = curr_idx + hash_idx;
            // Ensure not a heading or preceded by word character
            let is_standalone = hash_idx == 0
                || remaining.as_bytes()[hash_idx - 1] == b' '
                || remaining.as_bytes()[hash_idx - 1] == b'\t';

            if is_standalone && abs_hash + 1 < text.len() {
                let end_tag = text[abs_hash + 1..]
                    .find(|c: char| !c.is_alphanumeric() && c != '_' && c != '-' && c != '/')
                    .map(|i| abs_hash + 1 + i)
                    .unwrap_or(text.len());

                if end_tag > abs_hash + 1 {
                    if hash_idx > 0 {
                        spans.push(Span::styled(&text[curr_idx..abs_hash], base_style));
                    }
                    spans.push(Span::styled(
                        &text[abs_hash..end_tag],
                        Style::default()
                            .fg(theme.tag)
                            .add_modifier(Modifier::BOLD),
                    ));
                    curr_idx = end_tag;
                    continue;
                }
            }
        }

        // Remaining plain text
        spans.push(Span::styled(remaining, base_style));
        break;
    }

    spans
}

/// Finds the wiki-link target on the current line if any
pub fn get_link_under_cursor(note: &Note, cursor_line: usize) -> Option<String> {
    for link in &note.outgoing_links {
        if link.line_idx == cursor_line {
            return Some(link.target.clone());
        }
    }
    // If not exact line, check nearby or first link in note
    None
}
