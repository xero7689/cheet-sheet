use anyhow::Result;
use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};
use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;
use syntect::util::{LinesWithEndings, as_24_bit_terminal_escaped};
use termimad::crossterm::style::{Attribute, Color::Yellow};
use termimad::{MadSkin, ansi, gray};

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

fn make_skin() -> MadSkin {
    let mut skin = MadSkin::default();
    skin.set_headers_fg(ansi(178)); // 橙黃色標題
    skin.bold.set_fg(Yellow);
    skin.italic.set_fg(ansi(147)); // 淡紫色
    skin.inline_code.set_fgbg(ansi(222), ansi(236)); // 暖黃 on 深灰
    skin.code_block.set_fgbg(gray(17), gray(3));
    skin.table.set_fg(ansi(117)); // 淡藍色表格
    skin.headers[0].add_attr(Attribute::Bold);
    skin.headers[1].add_attr(Attribute::Bold);
    skin
}

enum Segment {
    Text(String),
    Code { lang: String, code: String },
}

fn split_segments(content: &str) -> Vec<Segment> {
    let mut segments = Vec::new();
    let mut rest = content;

    while let Some(fence_start) = rest.find("```") {
        // Text before the fence
        let before = &rest[..fence_start];
        if !before.is_empty() {
            segments.push(Segment::Text(before.to_string()));
        }

        let after_fence = &rest[fence_start + 3..];

        // Find the end of the opening fence line to extract language
        let lang_end = after_fence.find('\n').unwrap_or(after_fence.len());
        let lang = after_fence[..lang_end].trim().to_string();
        let code_start = &after_fence[lang_end..].trim_start_matches('\n');

        // Find the closing fence
        if let Some(close) = code_start.find("\n```") {
            let code = &code_start[..close];
            segments.push(Segment::Code {
                lang,
                code: code.to_string(),
            });
            rest = &code_start[close + 4..]; // skip "\n```"
        } else {
            // Unclosed fence — treat remainder as text
            segments.push(Segment::Text(rest[fence_start..].to_string()));
            rest = "";
            break;
        }
    }

    if !rest.is_empty() {
        segments.push(Segment::Text(rest.to_string()));
    }

    segments
}

fn highlight_code(lang: &str, code: &str) {
    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let theme = &ts.themes["base16-ocean.dark"];

    let syntax = ss
        .find_syntax_by_token(lang)
        .unwrap_or_else(|| ss.find_syntax_plain_text());

    let mut hl = HighlightLines::new(syntax, theme);
    println!(); // blank line before code block
    for line in LinesWithEndings::from(code) {
        let ranges = hl.highlight_line(line, &ss).unwrap_or_default();
        let escaped = as_24_bit_terminal_escaped(&ranges, false);
        print!("  {escaped}");
    }
    print!("\x1b[0m");
    println!(); // blank line after code block
}

fn render_markdown(content: &str) {
    let skin = make_skin();
    for segment in split_segments(content) {
        match segment {
            Segment::Text(text) => skin.print_text(&text),
            Segment::Code { lang, code } => highlight_code(&lang, &code),
        }
    }
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

    #[test]
    fn test_split_segments_no_code() {
        let content = "# Title\n\nSome text\n";
        let segments = split_segments(content);
        assert_eq!(segments.len(), 1);
        assert!(matches!(&segments[0], Segment::Text(t) if t == content));
    }

    #[test]
    fn test_split_segments_with_code() {
        let content = "# Title\n\n```bash\necho hello\n```\n\nAfter\n";
        let segments = split_segments(content);
        assert_eq!(segments.len(), 3);
        assert!(matches!(&segments[0], Segment::Text(_)));
        assert!(matches!(&segments[1], Segment::Code { lang, code }
            if lang == "bash" && code == "echo hello"));
        assert!(matches!(&segments[2], Segment::Text(_)));
    }
}
