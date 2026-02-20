# cheetsheet

A terminal cheatsheet viewer written in Rust. Look up command references instantly from your terminal.

```
$ cheetsheet tmux
```

## Installation

```bash
cargo install --path .
```

## Usage

```bash
# Look up a command
cheetsheet tmux
cheetsheet git
cheetsheet docker

# Use a custom sheets directory
cheetsheet tmux --config-dir ~/my-sheets
```

## Adding Cheatsheets

Cheatsheets are markdown files stored in `~/.config/cheetsheet/`.

```bash
mkdir -p ~/.config/cheetsheet
# Create a cheatsheet for tmux
vim ~/.config/cheetsheet/tmux.md
```

Example `tmux.md`:

```markdown
# Tmux Cheat Sheet

預設 prefix 鍵是 `Ctrl+b`，以下用 `<prefix>` 表示。

## Session 管理

| 操作 | 指令 |
|------|------|
| 新建 session | `tmux new -s <name>` |
| 列出 sessions | `tmux ls` |
| 附加 session | `tmux attach -t <name>` |
| 離開（不關閉）| `<prefix> d` |
| 切換 session | `<prefix> s` |
```

## Config Directory Resolution

1. `--config-dir` flag (highest priority)
2. `$XDG_CONFIG_HOME/cheetsheet/`
3. `~/.config/cheetsheet/` (default)

## Development

```bash
cargo test
cargo clippy
cargo fmt
```
