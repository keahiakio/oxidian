use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};
use crate::app::App;
use crate::ui::components::*;

pub fn render_main_ui(f: &mut Frame, app: &mut App) {
    let size = f.area();

    // If setup wizard is active, render full-screen setup modal
    if let Some(ref mut wizard) = app.setup_wizard {
        render_setup_wizard(f, size, wizard, &app.theme);
        return;
    }

    // Main layout:
    // Top (1): Bufferline / Tabs
    // Center (Min): Content (Explorer, Note Viewer, Backlinks)
    // Bottom (1): Statusline
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Top bufferline
            Constraint::Min(5),    // Main workspace
            Constraint::Length(1), // Bottom statusline
        ])
        .split(size);

    // 1. Bufferline
    render_bufferline(
        f,
        chunks[0],
        &app.bufferline,
        &app.theme,
        &app.vault.name,
    );

    // 2. Middle workspace splits:
    // [Explorer (Optional)] | [Note Viewer (Min)] | [Backlinks (Optional)]
    let mut h_constraints = Vec::new();
    if app.show_explorer {
        h_constraints.push(Constraint::Length(30));
    }
    h_constraints.push(Constraint::Min(30));
    if app.show_backlinks {
        h_constraints.push(Constraint::Length(34));
    }

    let h_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(h_constraints)
        .split(chunks[1]);

    let mut chunk_idx = 0;

    // Explorer
    if app.show_explorer {
        let is_focused = app.active_panel == ActivePanel::Explorer;
        render_file_tree(f, h_chunks[chunk_idx], &mut app.file_tree, &app.theme, is_focused);
        chunk_idx += 1;
    }

    // Note Viewer
    let note_rect = h_chunks[chunk_idx];
    let is_note_focused = app.active_panel == ActivePanel::Note;
    let curr_path = app.bufferline.current_path().cloned();
    let current_note = curr_path.as_ref().and_then(|p| app.vault.notes.get(p));

    // Cache metadata for statusline to avoid borrowing conflicts
    let current_title = current_note.map(|n| n.title.clone());
    let word_count = current_note.map(|n| n.word_count).unwrap_or(0);
    let total_lines = current_note.map(|n| n.lines.len()).unwrap_or(0);

    render_note_viewer(
        f,
        note_rect,
        current_note,
        &mut app.note_viewer,
        &app.theme,
        is_note_focused,
    );
    chunk_idx += 1;

    // Backlinks Panel
    if app.show_backlinks {
        let is_bl_focused = app.active_panel == ActivePanel::Backlinks;
        let note_ref = curr_path.as_ref().and_then(|p| app.vault.notes.get(p));
        render_backlinks_panel(
            f,
            h_chunks[chunk_idx],
            note_ref,
            &app.vault,
            &app.backlinks,
            &app.theme,
            is_bl_focused,
        );
    }

    // 3. Statusline
    let bl_count = app.backlinks.items.len();

    render_statusline(
        f,
        chunks[2],
        app.mode,
        &app.vault.name,
        current_title.as_deref(),
        word_count,
        bl_count,
        app.note_viewer.cursor_line,
        total_lines,
        &app.theme,
    );

    // 4. Overlays / Modals
    match app.mode {
        AppMode::FuzzyFind | AppMode::GrepFind => {
            render_fuzzy_finder(f, size, &mut app.fuzzy_finder, &app.vault, &app.theme);
        }
        AppMode::VaultSwitch => {
            render_vault_switcher(
                f,
                size,
                &mut app.vault_switcher,
                &app.vault.root,
                &app.theme,
            );
        }
        AppMode::Help => {
            render_help_popup(f, size, &app.theme);
        }
        _ => {}
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActivePanel {
    Explorer,
    Note,
    Backlinks,
}
