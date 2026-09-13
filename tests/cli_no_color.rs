use assert_cmd::Command;
use tempfile::tempdir;

#[test]
fn test_piped_no_ansi() {
    let temp_qvm = tempdir().unwrap();

    let mut cmd = Command::cargo_bin("qvm").unwrap();
    let assert = cmd.env("QVM_DIR", temp_qvm.path()).arg("list").assert();

    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);
    assert!(!stdout.contains("\x1b["));
}
