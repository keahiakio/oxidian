pub mod discovery;
pub mod note;
pub mod index;

pub use discovery::{discover_obsidian_vaults, resolve_vault, DiscoveredVault};
pub use note::{LineKind, Note, WikiLink};
pub use index::VaultIndex;
