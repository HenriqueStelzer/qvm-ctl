use assert_cmd::Command;
use tempfile::tempdir;

#[test]
fn test_create_valid() {
    let temp_qvm = tempdir().unwrap();
    let fake_iso = temp_qvm.path().join("fake.iso");
    std::fs::write(&fake_iso, b"test").unwrap();

    let mut cmd = Command::cargo_bin("qvm").unwrap();
    cmd.env("QVM_DIR", temp_qvm.path())
        .args(["create", "testvm", &fake_iso.to_string_lossy()])
        .assert()
        .success();

    assert!(temp_qvm.path().join("testvm/vm.conf").is_file());
    assert!(temp_qvm.path().join("testvm/disk.qcow2").is_file());

    // Duplicate create should fail
    let mut cmd2 = Command::cargo_bin("qvm").unwrap();
    cmd2.env("QVM_DIR", temp_qvm.path())
        .args(["create", "testvm", &fake_iso.to_string_lossy()])
        .assert()
        .failure();
}

#[test]
fn test_create_failures() {
    let temp_qvm = tempdir().unwrap();

    // No args
    let mut cmd = Command::cargo_bin("qvm").unwrap();
    cmd.env("QVM_DIR", temp_qvm.path())
        .arg("create")
        .assert()
        .failure();

    // Missing ISO
    let mut cmd2 = Command::cargo_bin("qvm").unwrap();
    cmd2.env("QVM_DIR", temp_qvm.path())
        .args(["create", "badvm", "/nonexistent.iso"])
        .assert()
        .failure();

    // Invalid name
    let fake_iso = temp_qvm.path().join("fake.iso");
    std::fs::write(&fake_iso, b"test").unwrap();

    let mut cmd3 = Command::cargo_bin("qvm").unwrap();
    cmd3.env("QVM_DIR", temp_qvm.path())
        .args(["create", "-x", &fake_iso.to_string_lossy()])
        .assert()
        .failure();
}
