use std::path::Path;
use std::process::Command;
use crossterm::{
    cursor::{Hide, Show},
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};

pub fn open_in_neovim(file_path: &Path, editor_bin: &str) -> Result<(), std::io::Error> {
    let mut stdout = std::io::stdout();

    // 1. Leave alternate screen, show cursor, disable raw mode
    let _ = disable_raw_mode();
    let _ = execute!(stdout, LeaveAlternateScreen, Show);

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

    // 3. Restore alternate screen, raw mode, hide cursor, and clear screen buffer
    let _ = enable_raw_mode();
    let _ = execute!(
        stdout,
        EnterAlternateScreen,
        Hide,
        Clear(ClearType::All)
    );

    Ok(())
}
