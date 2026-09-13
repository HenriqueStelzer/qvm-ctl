use assert_cmd::Command;
use tempfile::tempdir;

#[test]
fn test_disable_nonexistent() {
    let temp_qvm = tempdir().unwrap();

    let mut cmd = Command::cargo_bin("qvm").unwrap();
    cmd.env("QVM_DIR", temp_qvm.path())
        .args(["disable", "noexist"])
        .assert()
        .failure();
}

#[test]
fn test_disable_with_confirm() {
    let temp_qvm = tempdir().unwrap();
    let fake_iso = temp_qvm.path().join("fake.iso");
    std::fs::write(&fake_iso, b"test").unwrap();

    let mut cmd = Command::cargo_bin("qvm").unwrap();
    cmd.env("QVM_DIR", temp_qvm.path())
        .args(["create", "testvm", &fake_iso.to_string_lossy()])
        .assert()
        .success();

    let vm_path = temp_qvm.path().join("testvm");
    assert!(vm_path.is_dir());

    let mut dis_cmd = Command::cargo_bin("qvm").unwrap();
    dis_cmd
        .env("QVM_DIR", temp_qvm.path())
        .args(["disable", "testvm"])
        .write_stdin("testvm\n")
        .assert()
        .success();

    assert!(!vm_path.exists());
}
