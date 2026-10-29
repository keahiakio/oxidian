mod app;
mod cli;
mod config;
mod editor;
mod theme;
mod ui;
mod vault;

use clap::Parser;
use crossterm::{
    event::{self, Event, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io::{stdout, Result};
use std::time::Duration;

use app::App;
use cli::{Cli, Commands};
use config::OxidianConfig;
use vault::{discover_obsidian_vaults, resolve_vault, VaultIndex};

fn main() -> Result<()> {
    let args = Cli::parse();

    // Check subcommands first
    if let Some(cmd) = args.command {
        match cmd {
            Commands::Vaults => {
                let vaults = discover_obsidian_vaults();
                println!("󰉋 Discovered Obsidian Vaults ({} found):\n", vaults.len());
                for (i, v) in vaults.iter().enumerate() {
                    let active_marker = if v.is_open { " [Last Active]" } else { "" };
                    println!(
                        "  {}. \x1b[1;36m{}\x1b[0m{}\n     Path: \x1b[90m{}\x1b[0m",
                        i + 1,
                        v.name,
                        active_marker,
                        v.path.display()
                    );
                }
                return Ok(());
            }
            Commands::Find { query, vault } => {
                let v = resolve_vault(vault.as_deref()).map_err(|e| std::io::Error::new(std::io::ErrorKind::NotFound, e))?;
                let index = VaultIndex::build(v.path, v.name);
                let matcher = fuzzy_matcher::skim::SkimMatcherV2::default();
                use fuzzy_matcher::FuzzyMatcher;

                println!(" Searching in vault '{}' for '{}':\n", index.name, query);
                let mut matches = Vec::new();
                for (_path, note) in &index.notes {
                    let search_str = format!("{} {}", note.title, note.rel_path.display());
                    if let Some(score) = matcher.fuzzy_match(&search_str, &query) {
                        matches.push((score, note));
                    }
                }
                matches.sort_by(|a, b| b.0.cmp(&a.0));

                for (_score, note) in matches.iter().take(20) {
                    println!("  󰈙 \x1b[1;34m{}\x1b[0m (\x1b[90m{}\x1b[0m)", note.title, note.rel_path.display());
                }
                return Ok(());
            }
            Commands::Backlinks { note, vault } => {
                let v = resolve_vault(vault.as_deref()).map_err(|e| std::io::Error::new(std::io::ErrorKind::NotFound, e))?;
                let index = VaultIndex::build(v.path, v.name);

                let target_path = index.resolve_wikilink(&note)
                    .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, format!("Note '{}' not found", note)))?;

                let target_note = &index.notes[target_path];
                let backlinks = index.get_backlinks(target_path);

                println!("󰌹 Backlinks to \x1b[1;35m{}\x1b[0m ({} notes):\n", target_note.title, backlinks.len());
                for bl in backlinks {
                    println!("  󰈙 \x1b[1;34m{}\x1b[0m (\x1b[90m{}\x1b[0m)", bl.title, bl.rel_path.display());
                }
                return Ok(());
            }
            Commands::Daily { edit, vault } => {
                let v = resolve_vault(vault.as_deref()).map_err(|e| std::io::Error::new(std::io::ErrorKind::NotFound, e))?;
                let today = chrono::Local::now().format("%Y-%m-%d").to_string();
                let filename = format!("{}.md", today);
                let path = v.path.join(&filename);

                if !path.exists() {
                    let content = format!("# Daily Note: {}\n\n- [ ] \n\n## Notes\n\n", today);
                    let _ = std::fs::write(&path, content);
                    println!("󰄬 Created new daily note at: {}", path.display());
                }

                if edit {
                    let editor = std::env::var("EDITOR").unwrap_or_else(|_| "nvim".to_string());
                    std::process::Command::new(&editor).arg(&path).status()?;
                } else {
                    println!("{}", std::fs::read_to_string(&path)?);
                }
                return Ok(());
            }
        }
    }

    // Load configuration or detect first run
    let saved_config = OxidianConfig::load();
    let is_first_run = saved_config.is_none() || args.setup;
    let config = saved_config.unwrap_or_default();

    // Determine starting vault
    let vault_arg = args.vault.as_deref().or(config.default_vault_path.as_deref());
    let resolved = resolve_vault(vault_arg)
        .unwrap_or_else(|_| {
            let all = discover_obsidian_vaults();
            all.into_iter().next().unwrap_or_else(|| vault::DiscoveredVault {
                id: "default".to_string(),
                name: "Obsidian Vault".to_string(),
                path: std::env::current_dir().unwrap_or_default(),
                last_opened: 0,
                is_open: true,
            })
        });

    let vault_index = VaultIndex::build(resolved.path, resolved.name);

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Setup panic hook to ensure terminal is restored on crash
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = disable_raw_mode();
        let _ = execute!(std::io::stdout(), LeaveAlternateScreen);
        original_hook(panic_info);
    }));

    // Initialize App
    let mut app = App::new(vault_index, config, is_first_run);

    // Main event loop
    loop {
        terminal.draw(|f| {
            ui::render_main_ui(f, &mut app);
        })?;

        if app.should_quit {
            break;
        }

        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    app.handle_key(key);
                }
            }
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
