use assert_cmd::Command;
use tempfile::tempdir;

#[test]
fn test_launch_nonexistent() {
    let temp_qvm = tempdir().unwrap();

    let mut cmd = Command::cargo_bin("qvm").unwrap();
    cmd.env("QVM_DIR", temp_qvm.path())
        .args(["launch", "noexist"])
        .assert()
        .failure();
}

#[test]
fn test_launch_bad_flag() {
    let temp_qvm = tempdir().unwrap();

    let mut cmd = Command::cargo_bin("qvm").unwrap();
    cmd.env("QVM_DIR", temp_qvm.path())
        .args(["launch", "testvm", "--garbage"])
        .assert()
        .failure();
}
