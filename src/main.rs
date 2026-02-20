use anyhow::Result;
use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Parser)]
#[command(author, version, about = "Terminal cheatsheet viewer")]
struct Args {
    /// Command name to look up (e.g., tmux, git, docker)
    #[arg(value_name = "COMMAND")]
    command: String,

    /// Custom config directory (default: ~/.config/cheetsheet)
    #[arg(short, long, value_name = "DIR")]
    config_dir: Option<String>,
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

fn run(args: Args) -> Result<()> {
    let config_dir = resolve_config_dir(args.config_dir.as_deref());
    let sheet_path = find_sheet(&config_dir, &args.command)?;
    let content = fs::read_to_string(&sheet_path)?;
    render_markdown(&content);
    Ok(())
}

fn resolve_config_dir(custom: Option<&str>) -> PathBuf {
    if let Some(dir) = custom {
        return PathBuf::from(dir);
    }
    if let Ok(xdg) = std::env::var("XDG_CONFIG_HOME") {
        return PathBuf::from(xdg).join("cheetsheet");
    }
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".config")
        .join("cheetsheet")
}

fn find_sheet(config_dir: &Path, command: &str) -> Result<PathBuf> {
    let path = config_dir.join(format!("{command}.md"));
    if path.exists() {
        Ok(path)
    } else {
        anyhow::bail!(
            "No cheatsheet found for '{command}'.\nExpected: {}\nTip: create a markdown file at that path to get started.",
            path.display()
        )
    }
}

fn render_markdown(content: &str) {
    termimad::print_text(content);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_resolve_config_dir_custom() {
        let dir = resolve_config_dir(Some("/tmp/custom"));
        assert_eq!(dir, PathBuf::from("/tmp/custom"));
    }

    #[test]
    fn test_find_sheet_missing() {
        let tmp = TempDir::new().unwrap();
        let result = find_sheet(tmp.path(), "nonexistent");
        assert!(result.is_err());
        let msg = result.unwrap_err().to_string();
        assert!(msg.contains("No cheatsheet found for 'nonexistent'"));
    }

    #[test]
    fn test_find_sheet_found() {
        let tmp = TempDir::new().unwrap();
        let sheet = tmp.path().join("tmux.md");
        fs::write(&sheet, "# tmux\n").unwrap();
        let result = find_sheet(tmp.path(), "tmux");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), sheet);
    }
}
