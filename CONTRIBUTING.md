# Contributing to Oxidian

Thank you for your interest in improving `oxidian`! We appreciate contributions of all kinds, including bug fixes, performance optimizations, documentation improvements, theme additions, and feature proposals.

---

## Code of Conduct

This project is governed by the Contributor Covenant [Code of Conduct](CODE_OF_CONDUCT.md). By contributing, you agree to treat everyone with respect and kindness.

---

## Design Principles

When contributing code or UX changes to `oxidian`, please keep our core tenets in mind:

1. **Instantaneous Performance**: `oxidian` is built in Rust to be lightning fast. Indexing thousands of markdown notes and interactive fuzzy searching should feel instantaneous.
2. **Neovim & Vim Aesthetic**: Keybindings and visual styling follow modal Vim conventions (e.g. `j`/`k`, `/`, `n`/`N`, `<Space>ff`, `<Space>fg`, `:q`, `e`).
3. **Graceful Degradation**: Features like sixel image rendering or terminal color schemes should degrade gracefully if the terminal does not support them.
4. **Minimal External Dependencies**: Keep the dependency graph lean and permissive (MIT / Apache-2.0).

---

## Development & Local Testing

### Prerequisites
- Rust stable toolchain (2021 edition)
- Cargo

### Building Locally
```bash
git clone https://github.com/<your-username>/oxidian.git
cd oxidian
cargo build
```

### Running Tests & Verification
Run the unit test suite:
```bash
cargo test
```

Check code formatting and run clippy lints:
```bash
cargo fmt --check
cargo clippy --all-targets --all-features
```

Run oxidian against a local test vault or auto-discovered vaults:
```bash
cargo run --
# or with a specific vault:
cargo run -- -v /path/to/test/vault
```

---

## Coding Standards

- Write clean, idiomatic Rust following standard naming and module layout conventions.
- Use `cargo fmt` to format code before committing.
- Ensure any new features include appropriate unit tests and documentation updates.
- Write commit messages following [Conventional Commits](https://www.conventionalcommits.org/) (e.g., `feat:`, `fix:`, `perf:`, `docs:`, `refactor:`).

---

## Pull Request Process

1. Create a descriptive feature branch from `master`:
   ```bash
   git checkout -b feature/my-cool-feature
   ```
2. Make your changes and commit them cleanly.
3. Verify that tests, formatting, and clippy pass:
   ```bash
   cargo test
   cargo fmt --check
   ```
4. Push your branch to your fork and submit a Pull Request to `keahiakio/oxidian:master`.
5. Complete the PR template checklist to facilitate review.
