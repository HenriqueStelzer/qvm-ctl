use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_help_exits_zero() {
    let mut cmd = Command::cargo_bin("qvm").unwrap();
    cmd.arg("help")
        .assert()
        .success()
        .stdout(predicate::str::contains("qvm"));

    let mut cmd2 = Command::cargo_bin("qvm").unwrap();
    cmd2.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("qvm"));

    let mut cmd3 = Command::cargo_bin("qvm").unwrap();
    cmd3.arg("-h")
        .assert()
        .success()
        .stdout(predicate::str::contains("qvm"));
}

#[test]
fn test_version_output() {
    let mut cmd = Command::cargo_bin("qvm").unwrap();
    cmd.arg("version")
        .assert()
        .success()
        .stdout(predicate::str::contains("1.2.0"));

    let mut cmd2 = Command::cargo_bin("qvm").unwrap();
    cmd2.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("1.2.0"));
}
