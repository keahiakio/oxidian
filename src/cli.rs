use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "oxidian")]
#[command(author = "keahiakio")]
#[command(version = "0.1.0")]
#[command(about = "A fast CLI & Neovim-aesthetic TUI for Obsidian vaults in Rust", long_about = None)]
pub struct Cli {
    /// Vault to open (by name e.g. 'punkeahi_obsidian' or path)
    #[arg(short, long)]
    pub vault: Option<String>,

    /// Force launch initial setup wizard
    #[arg(long)]
    pub setup: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// List all discovered Obsidian vaults on this device
    Vaults,

    /// Fuzzy find notes across the vault from CLI
    Find {
        /// Search query
        query: String,
        /// Optional vault name or path
        #[arg(short, long)]
        vault: Option<String>,
    },

    /// List incoming backlinks for a specific note
    Backlinks {
        /// Target note title or path
        note: String,
        /// Optional vault name or path
        #[arg(short, long)]
        vault: Option<String>,
    },

    /// Open or display today's daily note
    Daily {
        /// Open directly in Neovim
        #[arg(short, long)]
        edit: bool,
        /// Optional vault name or path
        #[arg(short, long)]
        vault: Option<String>,
    },
}
