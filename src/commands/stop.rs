use nix::sys::signal::Signal;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

use crate::display::{info, ok, warn};
use crate::error::QvmError;
use crate::process;
use crate::tpm;
use crate::vm;

pub fn run(base: &Path, name: String) -> Result<(), QvmError> {
    if !vm::vm_is_running(base, &name) {
        tpm::stop_tpm(base, &name);
        let _ = std::fs::remove_file(vm::vm_pid_file(base, &name));
        let _ = std::fs::remove_file(vm::vm_port_file(base, &name));
        let _ = std::fs::remove_file(vm::vm_rdp_port_file(base, &name));

        return Err(QvmError::VmNotRunning(name));
    }

    let pidfile = vm::vm_pid_file(base, &name);
    let pid =
        process::read_pid_file(&pidfile).ok_or_else(|| QvmError::VmNotRunning(name.clone()))?;

    info(format!("Stopping {}...", name));

    process::send_signal(pid, Signal::SIGTERM);

    for _ in 0..10 {
        sleep(Duration::from_secs(1));
        if !vm::vm_is_running(base, &name) {
            break;
        }
    }

    if vm::vm_is_running(base, &name) {
        warn("SIGTERM ignored, sending SIGKILL...");
        process::send_signal(pid, Signal::SIGKILL);
        sleep(Duration::from_secs(1));
    }

    if vm::vm_is_running(base, &name) {
        return Err(QvmError::FailedToStopVm(name));
    }

    tpm::stop_tpm(base, &name);

    let _ = std::fs::remove_file(vm::vm_pid_file(base, &name));
    let _ = std::fs::remove_file(vm::vm_port_file(base, &name));
    let _ = std::fs::remove_file(vm::vm_rdp_port_file(base, &name));

    ok(format!("Stopped {}", name));
    Ok(())
}
