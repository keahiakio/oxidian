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

- **(｡♥‿♥｡) First-Time Onboarding Wizard**: On first run, `oxidian` auto-scans your system, detects existing vaults (`obsidiankitty`, `punkeahi_obsidian`), and lets you interactively configure your default vault and theme.
- **(=^･ω･^=) Zero-Config Vault Discovery**: Automatically reads `~/.config/obsidian/obsidian.json`. Switch between vaults on the fly with `<Space>v`.
- **(⌐■_■) Neovim & Kitty Aesthetics**:
  - **Lualine-style Statusbar**: Mode pill (`NORMAL`, `FIND`), active vault, note title, backlink counts, word counts, cursor coordinates.
  - **Bufferline-style Tab Bar**: Displays active and open notes across the top with Nerd Font icons.
  - **Nvim-Tree File Explorer**: Collapsible folder tree with arrow navigation (`j`/`k`, `o`/`<CR>`).
  - **Render-Markdown Formatting**: Styled headers (`#`, `##`), checkboxes (`[ ]` / `[x]`), callouts (`> [!NOTE]`, `> [!TIP]`, `> [!WARNING]`), and code blocks.
  - **Themes**: Pre-configured with **Catppuccin Mocha** (matches Kitty terminal), **Gruvbox Material**, **Everforest**, and **OneDark**. Cycle anytime with `t`.
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
┌──────────────────────────────────────────────────────────────────────────────────────────────────┐
│ [󰉋 punkeahi_obsidian] │  󰈙 2026-W28 󰅖 │  󰈙 Capacity Strategy 󰅖 │  󰈙 Revy Proposal 󰅖           │ Bufferline
├────────────────────┬─────────────────────────────────────────────┬───────────────────────────────┤
│  EXPLORER         │ 󰈙 Capacity Strategy (Locker+V3 Document)    │ 󰌹 BACKLINKS & GRAPH           │
│  󰉖 Weekly Notes   │   1 │ 󰉫 # Capacity Strategy                 │ 󰓹 Tags                        │
│     󰈙 2026-W28     │   2 │                                             │   #capacity  #logistics       │
│     󰈙 2026-01-26   │   3 │ 󰄲 [x] Audit SOP Completed                   │                               │
│  󰉖 Capacity       │   4 │ 󰄱 [ ] Finalize expansion rollouts           │ 󰌹 Incoming (2)                │
│   󰈙 Strategy (V3)  │   5 │                                             │   󰁕 󰈙 2026-01-26              │
│   󰈙 SOP Audit      │   6 │ 󰌹 [[1P Capacity Audit SOP]]                 │   󰁕 󰈙 Weekly Notes/2026-W28   │
│  󰉋 expansion_ops  │   7 │                                             │                               │
│  󰉋 Templates      │   8 │ ▌ 󰋽 NOTE High volume site review           │ 󰌷 Outgoing (3)                │
│                    │   9 │   Ensure dock capacity checks pass prior    │   󰈙 1P Capacity Audit SOP     │
│                    │  10 │   to peak cutoff windows.                   │   󰈙 Revy Operational Plan     │
│                    │  11 │                                             │   󱄊 Phantom Note (Uncreated)  │
├────────────────────┴─────────────────────────────────────────────┴───────────────────────────────┤
│ NORMAL  󰘬 punkeahi_obsidian  󰈙 Capacity Strategy │ 󰌹 2 │ 󰧭 850 words │  6:11 (54%) │  Catppuccin │ Lualine
└──────────────────────────────────────────────────────────────────────────────────────────────────┘
```

---

### 2. Telescope Fuzzy Finder (`<Space>ff` or `<Ctrl-p>`)
```text
  ┌─────────────────────────────  TELESCOPE FIND FILE (32 matches) ──────────────────────────────┐
  │ ┌───────────────────────────────────────────────────────────────────────────────────────────┐ │
  │ │ ❯ Capacity█                                                                               │ │
  │ └───────────────────────────────────────────────────────────────────────────────────────────┘ │
  │ 󰁕 󰈙 Capacity Strategy (Locker+V3)  (Capacity/Strategy.md)  │ # Capacity Strategy (Locker+V3)  │
  │   󰈙 1P Capacity Audit SOP          (Capacity/SOP.md)       │ Path: Capacity/Strategy.md       │
  │   󰈙 Capacity Analysis              (Wiki/CapacityAnalysis) │ Words: 850                       │
  │   󰈙 Campus Capacity Sync           (Capacity/Campus.md)    │ ──────────────────────────────── │
  │   󰈙 CM Approval Request            (Claude/Capacity/CM.md) │ # Overview                       │
  │   󰈙 LSCI Saint Charles Capacity    (Capacity/LSCI.md)      │ This document outlines the       │
  │   󰈙 Revy Operational Plan          (Capacity/Revy.md)      │ Locker+V3 capacity automation... │
  └───────────────────────────────────────────────────────────┴──────────────────────────────────┘
```

---

### 3. First-Run Setup & Vault Suggestion Wizard (`oxidian --setup`)
```text
  ┌────────────────────────────── 󰋜 OXIDIAN FIRST-TIME SETUP 󰋜 ───────────────────────────────────┐
  │   ██████╗ ██╗  ██╗██╗██████╗ ██╗ █████╗ ███╗   ██╗                                            │
  │  ██╔═══██╗╚██╗██╔╝██║██╔══██╗██║██╔══██╗████╗  ██║                                            │
  │  ██║   ██║ ╚███╔╝ ██║██║  ██║██║███████║██╔██╗ ██║                                            │
  │  Obsidian CLI & Neovim TUI in Rust                                                            │
  │                                                                                               │
  │  [1. Select Vault] 󰁕 2. Choose Theme 󰁕 3. Confirm                                             │
  │ ┌───────────────────────────────────────────────────────────────────────────────────────────┐ │
  │ │ 󰈙 Detected Installed Obsidian Vaults:                                                     │ │
  │ │   󰄲 󰉋 obsidiankitty     (/home/keahiakio/Documents/obsidiankitty)                         │ │
  │ │   󰄱 󰉋 punkeahi_obsidian (/home/keahiakio/Documents/punkeahi_obsidian)                       │ │
  │ │   󰄱 󰉖 Specify custom vault path...                                                        │ │
  │ └───────────────────────────────────────────────────────────────────────────────────────────┘ │
  │  [j/k or ↑/↓]: Navigate  |  [Enter/Space]: Select Vault & Continue  |  [q]: Quit              │
  └───────────────────────────────────────────────────────────────────────────────────────────────┘
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

## ＼(＾▽＾)／ Configuration

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

## (ﾉ´ヮ`)ﾉ*: ･ﾟ Building & Installation

```bash
cargo build --release
install -m 755 target/release/oxidian ~/.local/bin/
```
