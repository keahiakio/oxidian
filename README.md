# (｡•̀ᴗ-)✧ oxidian

```text
   ██████╗ ██╗  ██╗██╗██████╗ ██╗ █████╗ ███╗   ██╗
  ██╔═══██╗╚██╗██╔╝██║██╔══██╗██║██╔══██╗████╗  ██║
  ██║   ██║ ╚███╔╝ ██║██║  ██║██║███████║██╔██╗ ██║
  ██║   ██║ ██╔██╗ ██║██║  ██║██║██╔══██║██║╚██╗██║
  ╚██████╔╝██╔╝ ██╗██║██████╔╝██║██║  ██║██║ ╚████║
   ╚═════╝ ╚═╝  ╚═╝╚═╝╚═════╝ ╚═╝╚═╝  ╚═╝╚═╝  ╚═══╝
         Obsidian CLI & Neovim TUI in Rust
```

> **A blazing-fast CLI & Neovim-aesthetic TUI for Obsidian vaults, built in Rust with Ratatui.**  
> *(づ｡◕‿‿◕｡)づ All the power of your Obsidian graph with zero Electron overhead.*

`oxidian` gives you an instant, lightweight terminal interface for your Obsidian knowledge base with the exact look, feel, and vim muscle memory of your Neovim environment.

---

## (•̀ᴗ•́)و Features

- **(｡♥‿♥｡) First-Time Onboarding Wizard**: On first run, `oxidian` auto-scans your system, detects existing vaults (`work_vault`, `personal_notes`), and lets you interactively configure your default vault and theme.
- **(=^･ω･^=) Zero-Config Vault Discovery**: Automatically reads `~/.config/obsidian/obsidian.json`. Switch between vaults on the fly with `<Space>v`.
- **(⌐■_■) Neovim Aesthetics**:
  - **Lualine-style Statusbar**: Mode pill (`NORMAL`, `FIND`), active vault, note title, backlink counts, word counts, cursor coordinates.
  - **Bufferline-style Tab Bar**: Displays active and open notes across the top with close indicators.
  - **Nvim-Tree File Explorer**: Collapsible folder tree with arrow navigation (`j`/`k`, `o`/`<CR>`).
  - **Render-Markdown Formatting**: Styled headers (`#`, `##`), checkboxes (`[ ]` / `[x]`), callouts (`> [!NOTE]`, `> [!TIP]`, `> [!WARNING]`), and code blocks.
  - **Themes**: Pre-configured with **Catppuccin Mocha**, **Gruvbox Material**, **Everforest**, and **OneDark**. Cycle anytime with `t`.
- **[o_o] Telescope / Fzf Fuzzy Finding**: Floating modal picker for filename search (`<Space>ff` or `Ctrl-P`) and live grep (`<Space>fg`) with real-time split preview.
- **(* ^ ω ^) Obsidian Knowledge Graph**:
  - Live **Backlinks panel** (`<Space>b`) showing notes linking to the current file.
  - Interactive **Wiki-link Traversal**: Jump straight to `[[note]]` targets under cursor with `gd` or `<CR>`.
  - Jump back and forward in note history with `<Ctrl-o>` and `<Ctrl-i>`.
- **(ﾉ◕ヮ◕)ﾉ*:･ﾟ Seamless Neovim Handoff**: Press `e` on any note to temporarily suspend the TUI and launch your actual **`nvim`** configuration (with all your LSP, markdown, and completion plugins). When you quit (`:q`), `oxidian` cleanly flushes the buffer cache, restores the alternate screen, and repaints in-place.
- **(・∀・) Daily Notes**: Quickly open or create today's daily note with `<Space>d`.

---

## (★ω★) Graphical TUI in Action

### 1. Main Workspace Layout
```text
+--------------------------------------------------------------------------------+
| [Vault: work_vault] | 2026-W42 [x] | System Architecture [x] | API Design [x]  |
+--------------------+---------------------------------------+-------------------+
| EXPLORER           | System Architecture                   | BACKLINKS & GRAPH |
| v Projects         |   1 | # System Architecture           | Tags              |
|     Architecture   |   2 |                                 |   #infra  #design |
|     Database       |   3 | [x] Draft initial RFC           |                   |
| v Weekly Notes     |   4 | [ ] Review edge cache latency   | Incoming (2)      |
|     2026-W42       |   5 |                                 |   -> 2026-W42     |
|     2026-W41       |   6 | [[API Specification]]           |   -> API Design   |
| > Reference        |   7 |                                 |                   |
| > Templates        |   8 | > [!NOTE] Performance Target    | Outgoing (3)      |
|                    |   9 |   Ensure p99 latency stays      |   * API Spec      |
|                    |  10 |   under 15ms at edge nodes.     |   * Edge Cache    |
|                    |  11 |                                 |   ? Phantom Note  |
+--------------------+---------------------------------------+-------------------+
| NORMAL  work_vault  System Architecture | Links: 2 | 850w | 6:11 (54%) | Theme |
+--------------------------------------------------------------------------------+
```

---

### 2. Telescope Fuzzy Finder (`<Space>ff` or `<Ctrl-p>`)
```text
+----------------------- TELESCOPE FIND FILE (24 matches) -----------------------+
| > Architecture_                                                                |
+-------------------------------------------------+------------------------------+
| >> System Architecture   (Projects/Arch.md)     | # System Architecture        |
|    Security Architecture (Projects/Sec.md)      | Path: Projects/Arch.md       |
|    Architecture Review   (Weekly/2026-W40.md)   | Words: 850                   |
|    Network Topology      (Infra/Network.md)     | ---------------------------- |
|    Storage Cluster RFC   (Infra/Storage.md)     | Core design document for the |
|    Database Schema       (Data/Schema.md)       | high-throughput distributed  |
|    API Gateway Config    (Services/Gateway.md)  | edge routing tier...         |
+-------------------------------------------------+------------------------------+
```

---

### 3. First-Run Setup & Vault Suggestion Wizard (`oxidian --setup`)
```text
+--------------------------- OXIDIAN FIRST-TIME SETUP ---------------------------+
|   ██████╗ ██╗  ██╗██╗██████╗ ██╗ █████╗ ███╗   ██╗                             |
|  ██╔═══██╗╚██╗██╔╝██║██╔══██╗██║██╔══██╗████╗  ██║                             |
|  ██║   ██║ ╚███╔╝ ██║██║  ██║██║███████║██╔██╗ ██║                             |
|  Obsidian CLI & Neovim TUI in Rust                                             |
|                                                                                |
|  [1. Select Vault] -> 2. Choose Theme -> 3. Confirm                            |
| +----------------------------------------------------------------------------+ |
| | Detected Installed Obsidian Vaults:                                        | |
| |   [*] work_vault       (/home/user/vaults/work_vault)                      | |
| |   [ ] personal_notes   (/home/user/vaults/personal_notes)                  | |
| |   [ ] Specify custom vault path...                                         | |
| +----------------------------------------------------------------------------+ |
|  [j/k or Up/Down]: Navigate  |  [Enter/Space]: Select & Continue  |  [q]: Quit |
+--------------------------------------------------------------------------------+
```

---

## (•‿•) Keybindings

### Navigation & Panels
| Key | Action |
| --- | --- |
| `h` / `l` | Switch focus between **Explorer**, **Note Viewer**, and **Backlinks** |
| `j` / `k` or `↑` / `↓` | Move up / down in active panel |
| `gg` / `G` | Jump to top / bottom of current note |
| `<C-d>` / `<C-u>` | Half-page down / up |
| `<Space>e` | Toggle File Explorer tree |
| `<Space>b` | Toggle Backlinks & Graph panel |

### Search & Discovery (Telescope style)
| Key | Action |
| --- | --- |
| `<Space>ff` or `<C-p>` | Fuzzy find notes in vault (with live preview) |
| `<Space>fg` | Live grep full-text across vault note contents |
| `<Space>v` | Switch between Obsidian vaults |
| `<Space>d` | Open or create today's Daily Note (`YYYY-MM-DD.md`) |

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
| `t` | Cycle color themes (*Catppuccin*, *Gruvbox*, *Everforest*) |
| `?` or `<F1>` | Toggle which-key cheatsheet modal |
| `q` | Quit Oxidian |

---

## (ง •̀_•́)ง CLI Commands

Run headless queries directly from your shell or pipe into scripts:

```bash
# Launch interactive TUI
oxidian

# Launch directly into a specific vault
oxidian -v work_vault
oxidian -v personal_notes

# Force re-run the initial graphical setup wizard
oxidian --setup

# List all discovered Obsidian vaults on this machine
oxidian vaults

# Fuzzy find notes matching a query from shell
oxidian find "Architecture" -v work_vault

# Find incoming backlinks to a note
oxidian backlinks "System Architecture" -v work_vault

# Open or view today's daily note
oxidian daily
oxidian daily --edit
```

---

## ＼(＾▽＾)／ Configuration

Stored automatically in `~/.config/oxidian/config.json`:

```json
{
  "default_vault_path": "/home/user/vaults/work_vault",
  "theme": "Catppuccin Mocha",
  "editor": "nvim",
  "show_backlinks": true,
  "relative_line_numbers": true
}
```

---

## (ﾉ´ヮ`)ﾉ*: ･ﾟ Building & Installation

```bash
cargo build --release
install -m 755 target/release/oxidian ~/.local/bin/
```

---

## (•‿•) License

Dual-licensed under either of:

- **MIT License** ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)
- **Apache License, Version 2.0** ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)

at your option.

All third-party dependencies used in this crate are 100% compliant and permissive (MIT, Apache-2.0, or BSD-compatible).
