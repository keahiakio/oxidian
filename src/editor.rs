use std::path::Path;
use std::process::Command;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

pub fn open_in_neovim(file_path: &Path, editor_bin: &str) -> Result<(), std::io::Error> {
    // 1. Temporarily leave alternate screen and disable raw mode
    let mut stdout = std::io::stdout();
    let _ = disable_raw_mode();
    let _ = execute!(stdout, LeaveAlternateScreen);

    // 2. Launch Neovim
    let editor = if editor_bin.is_empty() {
        std::env::var("EDITOR").unwrap_or_else(|_| "nvim".to_string())
    } else {
        editor_bin.to_string()
    };

    let mut child = Command::new(&editor)
        .arg(file_path)
        .spawn()?;

    let _ = child.wait();

    // 3. Restore alternate screen and raw mode for Oxidian TUI
    let _ = enable_raw_mode();
    let _ = execute!(stdout, EnterAlternateScreen);

    Ok(())
}
