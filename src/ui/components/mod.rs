pub mod bufferline;
pub mod file_tree;
pub mod note_viewer;
pub mod backlinks_panel;
pub mod statusline;
pub mod fuzzy_finder;
pub mod vault_switcher;
pub mod help_popup;
pub mod setup_wizard;

pub use bufferline::{render_bufferline, BufferlineState};
pub use file_tree::{render_file_tree, FileTreeState};
pub use note_viewer::{render_note_viewer, NoteViewerState, get_link_under_cursor};
pub use backlinks_panel::{render_backlinks_panel, BacklinksState};
pub use statusline::{render_statusline, AppMode};
pub use fuzzy_finder::{render_fuzzy_finder, FuzzyFinderState};
pub use vault_switcher::{render_vault_switcher, VaultSwitcherState};
pub use help_popup::render_help_popup;
pub use setup_wizard::{render_setup_wizard, SetupWizardState, SetupStep};
