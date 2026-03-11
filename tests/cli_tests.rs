use assert_cmd::Command;
use predicates::prelude::*;
use std::process;
use tempfile::TempDir;

fn setup_test_repo() -> TempDir {
    let dir = TempDir::new().unwrap();
    let path = dir.path();
    process::Command::new("git")
        .args(["init"])
        .current_dir(path)
        .output()
        .unwrap();
    process::Command::new("git")
        .args(["config", "user.email", "test@test.com"])
        .current_dir(path)
        .output()
        .unwrap();
    process::Command::new("git")
        .args(["config", "user.name", "Test"])
        .current_dir(path)
        .output()
        .unwrap();
    std::fs::write(path.join("test.txt"), "hello").unwrap();
    process::Command::new("git")
        .args(["add", "."])
        .current_dir(path)
        .output()
        .unwrap();
    process::Command::new("git")
        .args(["commit", "-m", "feat: initial commit"])
        .current_dir(path)
        .output()
        .unwrap();
    dir
}

#[test]
fn test_help_flag() {
    let mut cmd = Command::cargo_bin("git-vibe").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Vibe check"));
}

#[test]
fn test_version_flag() {
    let mut cmd = Command::cargo_bin("git-vibe").unwrap();
    cmd.arg("--version").assert().success();
}

#[test]
fn test_invalid_path() {
    let mut cmd = Command::cargo_bin("git-vibe").unwrap();
    cmd.args(["--path", "/nonexistent/path/that/does/not/exist"])
        .assert()
        .failure();
}

#[test]
fn test_run_on_git_repo() {
    let repo = setup_test_repo();
    let mut cmd = Command::cargo_bin("git-vibe").unwrap();
    cmd.args(["--path", repo.path().to_str().unwrap()])
        .assert()
        .success();
}

#[test]
fn test_json_output() {
    let repo = setup_test_repo();
    let mut cmd = Command::cargo_bin("git-vibe").unwrap();
    cmd.args(["--path", repo.path().to_str().unwrap(), "--json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("vibe_score"));
}

#[test]
fn test_period_flag() {
    let repo = setup_test_repo();
    let mut cmd = Command::cargo_bin("git-vibe").unwrap();
    cmd.args([
        "--path",
        repo.path().to_str().unwrap(),
        "--period",
        "3m",
    ])
    .assert()
    .success();
}

#[test]
fn test_invalid_period() {
    let repo = setup_test_repo();
    let mut cmd = Command::cargo_bin("git-vibe").unwrap();
    cmd.args([
        "--path",
        repo.path().to_str().unwrap(),
        "--period",
        "abc",
    ])
    .assert()
    .failure();
}
