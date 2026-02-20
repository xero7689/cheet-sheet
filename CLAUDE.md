# cheet-sheet

## Project Overview

A Rust CLI tool for displaying markdown cheatsheets in the terminal.

## Binary

- Package name: `cheet-sheet`
- Binary name: `cheetsheet`

## Key Files

- `src/main.rs` — All core logic (Args, run, resolve_config_dir, find_sheet, render_markdown)
- `tests/cli.rs` — Integration tests using assert_cmd

## Cheatsheet Storage

Sheets are stored as markdown files at `~/.config/cheetsheet/{command}.md`.

Priority order for config dir:
1. `--config-dir` CLI flag
2. `$XDG_CONFIG_HOME/cheetsheet/`
3. `~/.config/cheetsheet/` (default)

## Commands

```bash
# Build
cargo build

# Run
cargo run -- <command>
cargo run -- tmux --config-dir /path/to/sheets

# Test
cargo test

# Lint
cargo clippy
cargo fmt
```

## Dependencies

- `clap` — CLI argument parsing (derive feature)
- `anyhow` — Error handling
- `termimad` — Markdown rendering in terminal
- `dirs` — Cross-platform home directory resolution
