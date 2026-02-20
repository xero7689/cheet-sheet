use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

fn cmd() -> Command {
    assert_cmd::cargo_bin_cmd!("cheetsheet")
}

#[test]
fn test_help() {
    for flag in &["-h", "--help"] {
        cmd()
            .arg(flag)
            .assert()
            .success()
            .stdout(predicate::str::contains("Usage"));
    }
}

#[test]
fn test_missing_sheet() {
    let tmp = TempDir::new().unwrap();
    cmd()
        .args(["nonexistent-cmd-xyz", "--config-dir", tmp.path().to_str().unwrap()])
        .assert()
        .failure()
        .stderr(predicate::str::contains("No cheatsheet found for 'nonexistent-cmd-xyz'"));
}

#[test]
fn test_found_sheet() {
    let tmp = TempDir::new().unwrap();
    let sheet = tmp.path().join("tmux.md");
    fs::write(&sheet, "# Tmux\n\n**prefix**: `Ctrl+b`\n").unwrap();

    cmd()
        .args(["tmux", "--config-dir", tmp.path().to_str().unwrap()])
        .assert()
        .success();
}
