use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::tempdir;

#[test]
fn test_list_empty() {
    let temp_qvm = tempdir().unwrap();

    let mut cmd = Command::cargo_bin("qvm").unwrap();
    cmd.env("QVM_DIR", temp_qvm.path())
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("No VMs found."));
}

#[test]
fn test_list_with_vm() {
    let temp_qvm = tempdir().unwrap();
    let fake_iso = temp_qvm.path().join("fake.iso");
    std::fs::write(&fake_iso, b"test").unwrap();

    let mut cmd = Command::cargo_bin("qvm").unwrap();
    cmd.env("QVM_DIR", temp_qvm.path())
        .args(["create", "testvm", &fake_iso.to_string_lossy()])
        .assert()
        .success();

    let mut list_cmd = Command::cargo_bin("qvm").unwrap();
    list_cmd
        .env("QVM_DIR", temp_qvm.path())
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("testvm"))
        .stdout(predicate::str::contains("stopped"));
}
