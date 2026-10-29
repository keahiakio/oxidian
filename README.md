# 󰈙 oxidian

> **A blazing-fast CLI & Neovim-aesthetic TUI for Obsidian vaults, built in Rust with Ratatui.**

`oxidian` gives you an instant, lightweight terminal interface for your Obsidian knowledge base with the exact look, feel, and vim muscle memory of your Neovim environment—with zero Electron overhead.

---

## ✨ Features

- **󰋜 First-Time Setup Wizard**: On initial run, `oxidian` scans your system, detects existing Obsidian vaults (e.g. `obsidiankitty`, `punkeahi_obsidian`), and lets you interactively choose your default vault and aesthetic theme.
- **󰉋 Zero-Config Vault Discovery**: Automatically discovers all vaults registered in Obsidian (`~/.config/obsidian/obsidian.json`). Switch between vaults on the fly with `<Space>v`.
- ** Neovim & Kitty Aesthetics**:
  - **Lualine-style Statusbar**: Mode pill (`NORMAL`, `FIND`), active vault, note title, backlink counts, word counts, cursor coordinates.
  - **Bufferline-style Tab Bar**: Displays active and open notes across the top with Nerd Font icons (`󰈙`, `󰈮`).
  - **Nvim-Tree File Explorer**: Collapsible folder tree with arrow navigation (`j`/`k`, `o`/`<CR>`).
  - **Render-Markdown Formatting**: Styled headers (`#`, `##`), checkboxes (`󰄱` / `󰄲`), callouts (`> [!NOTE]`, `> [!TIP]`, `> [!WARNING]`), and code blocks.
  - **Themes**: Pre-configured with **Catppuccin Mocha** (matches Kitty terminal), **Gruvbox Material**, **Everforest**, and **OneDark**. Cycle anytime with `t`.
- ** Telescope / Fzf Fuzzy Finding**: Floating modal picker for filename search (`<Space>ff` or `Ctrl-P`) and live grep (`<Space>fg`) with real-time preview.
- **󰌹 Obsidian Knowledge Graph**:
  - Live **Backlinks panel** (`<Space>b`) showing notes linking to the current file.
  - Interactive **Wiki-link Traversal**: Jump straight to `[[note]]` targets under cursor with `gd` or `<CR>`.
  - Jump back and forward in note history with `<Ctrl-o>` and `<Ctrl-i>`.
- **󰅩 Seamless Neovim Handoff**: Press `e` on any note to temporarily suspend the TUI and launch your actual **`nvim`** configuration (with all your LSP, markdown, and completion plugins). When you quit (`:q`), `oxidian` instantly resumes and reloads.
- **󰃭 Daily Notes**: Quickly open or create today's daily note with `<Space>d`.

---

## ⌨️ Keybindings

### Navigation & Panels
| Key | Action |
| --- | --- |
| `h` / `l` | Switch focus between **Explorer**, **Note Viewer**, and **Backlinks** |
| `j` / `k` or `↑` / `↓` | Move up / down in active panel |
| `gg` / `G` | Jump to top / bottom of current note |
| `<C-d>` / `<C-u>` | Page down / page up |
| `<Space>b` | Toggle Backlinks & Graph panel |
| `<Space>e` | Toggle File Explorer tree |

### Search & Discovery (Telescope style)
| Key | Action |
| --- | --- |
| `<Space>ff` or `<C-p>` | Fuzzy find notes in vault |
| `<Space>fg` | Live grep vault note contents |
| `<Space>v` | Switch between Obsidian vaults |
| `<Space>d` | Open or create today's Daily Note |

### Wiki-Links & Tabs
| Key | Action |
| --- | --- |
| `gd` or `<CR>` | Follow `[[wiki-link]]` under cursor |
| `<C-o>` | Jump back to previous note in history |
| `<C-i>` | Jump forward in note history |
| `Tab` / `S-Tab` | Next / previous open tab |
| `<Space>x` | Close current tab |

### Editing & Settings
| Key | Action |
| --- | --- |
| `e` | Open active note in **Neovim** (`nvim`) and return on `:q` |
| `t` | Cycle color themes (Catppuccin, Gruvbox, Everforest) |
| `?` or `<F1>` | Toggle which-key cheatsheet modal |
| `q` | Quit |

---

## 🚀 CLI Commands

Run headless queries directly from your shell:

```bash
# Launch interactive TUI
oxidian

# Launch directly into a specific vault
oxidian -v punkeahi_obsidian
oxidian -v obsidiankitty

# Force re-run the initial graphical setup wizard
oxidian --setup

# List all discovered Obsidian vaults on this machine
oxidian vaults

# Fuzzy find notes matching a query from shell
oxidian find "Capacity" -v punkeahi_obsidian

# Find incoming backlinks to a note
oxidian backlinks "Capacity Strategy" -v punkeahi_obsidian

# Open or view today's daily note
oxidian daily
oxidian daily --edit
```

---

## ⚙️ Configuration

Stored automatically in `~/.config/oxidian/config.json`:

```json
{
  "default_vault_path": "/home/keahiakio/Documents/obsidiankitty",
  "theme": "Catppuccin Mocha",
  "editor": "nvim",
  "show_backlinks": true,
  "relative_line_numbers": true
}
```

---

## 📦 Building & Installation

```bash
cargo build --release
cp target/release/oxidian ~/.local/bin/
```
