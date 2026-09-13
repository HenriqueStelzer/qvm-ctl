use crate::error::QvmError;
use crate::process;
use crate::vm;
use nix::sys::signal::Signal;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

pub fn start_tpm(base: &Path, name: &str) -> Result<(), QvmError> {
    let vmdir = vm::vm_dir(base, name);
    let socket = vm::vm_tpm_socket(base, name);
    let state_dir = vmdir.join("tpm");

    fs::create_dir_all(&state_dir)?;
    let _ = fs::remove_file(&socket);

    let status = Command::new("swtpm")
        .arg("socket")
        .arg("--tpm2")
        .arg("--tpmstate")
        .arg(format!("dir={}", state_dir.display()))
        .arg("--ctrl")
        .arg(format!("type=unixio,path={}", socket.display()))
        .arg("--daemon")
        .status()
        .map_err(|_| QvmError::MissingDependency("swtpm".to_string()))?;

    if !status.success() {
        return Err(QvmError::FailedToStartTpm);
    }

    // Wait for socket
    let mut ready = false;
    for _ in 0..20 {
        if socket.exists() {
            ready = true;
            break;
        }
        sleep(Duration::from_millis(100));
    }

    if !ready {
        return Err(QvmError::FailedToStartTpm);
    }

    // Find PID via pgrep matching socket path (preserving bash behavior)
    let pgrep_pattern = format!("swtpm.*path={}", socket.display());
    if let Ok(out) = Command::new("pgrep").args(["-f", &pgrep_pattern]).output() {
        if out.status.success() {
            let pid_str = String::from_utf8_lossy(&out.stdout);
            if let Some(first_line) = pid_str.lines().next() {
                if let Ok(pid) = first_line.trim().parse::<i32>() {
                    let _ = process::write_pid_file(&vm::vm_tpm_pid_file(base, name), pid);
                }
            }
        }
    }

    Ok(())
}

pub fn stop_tpm(base: &Path, name: &str) {
    let socket = vm::vm_tpm_socket(base, name);
    if socket.exists() {
        let _ = Command::new("swtpm_ioctl")
            .args(["--unix", &socket.to_string_lossy(), "--save"])
            .output();
    }

    let pidfile = vm::vm_tpm_pid_file(base, name);
    if let Some(pid) = process::read_pid_file(&pidfile) {
        process::send_signal(pid, Signal::SIGTERM);
        let _ = fs::remove_file(&pidfile);
    }

    let _ = fs::remove_file(&socket);
}
