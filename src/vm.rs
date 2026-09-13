use crate::error::QvmError;
use crate::process;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VmName(String);

impl VmName {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for VmName {
    type Error = QvmError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(QvmError::InvalidVmName(value));
        }

        let bytes = value.as_bytes();
        // First char: [a-zA-Z0-9]
        if !bytes[0].is_ascii_alphanumeric() {
            return Err(QvmError::InvalidVmName(value));
        }

        // Remaining chars: [a-zA-Z0-9_-]
        for &b in &bytes[1..] {
            if !b.is_ascii_alphanumeric() && b != b'_' && b != b'-' {
                return Err(QvmError::InvalidVmName(value));
            }
        }

        Ok(VmName(value))
    }
}

impl TryFrom<&str> for VmName {
    type Error = QvmError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_string())
    }
}

pub fn vm_dir(base: &Path, name: &str) -> PathBuf {
    base.join(name)
}

pub fn vm_pid_file(base: &Path, name: &str) -> PathBuf {
    vm_dir(base, name).join(format!("{}.pid", name))
}

pub fn vm_log_file(base: &Path, name: &str) -> PathBuf {
    vm_dir(base, name).join("qemu.log")
}

pub fn vm_port_file(base: &Path, name: &str) -> PathBuf {
    vm_dir(base, name).join("spice.port")
}

pub fn vm_rdp_port_file(base: &Path, name: &str) -> PathBuf {
    vm_dir(base, name).join("rdp.port")
}

pub fn vm_tpm_pid_file(base: &Path, name: &str) -> PathBuf {
    vm_dir(base, name).join("swtpm.pid")
}

pub fn vm_tpm_socket(base: &Path, name: &str) -> PathBuf {
    vm_dir(base, name).join("swtpm.sock")
}

pub fn vm_is_running(base: &Path, name: &str) -> bool {
    let pidfile = vm_pid_file(base, name);
    if let Some(pid) = process::read_pid_file(&pidfile) {
        if process::is_process_alive(pid) {
            return process::check_process_comm(pid, "qemu-system-x86_64");
        }
    }
    false
}

pub fn tpm_is_running(base: &Path, name: &str) -> bool {
    let pidfile = vm_tpm_pid_file(base, name);
    if let Some(pid) = process::read_pid_file(&pidfile) {
        return process::is_process_alive(pid);
    }
    false
}

pub fn cleanup_stale_files(base: &Path, name: &str) {
    let pidfile = vm_pid_file(base, name);
    if pidfile.exists() && !vm_is_running(base, name) {
        let _ = std::fs::remove_file(pidfile);
        let _ = std::fs::remove_file(vm_port_file(base, name));
        let _ = std::fs::remove_file(vm_rdp_port_file(base, name));
    }

    let tpm_pid = vm_tpm_pid_file(base, name);
    if tpm_pid.exists() && !tpm_is_running(base, name) {
        let _ = std::fs::remove_file(tpm_pid);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vm_name_validation() {
        assert!(VmName::try_from("win11").is_ok());
        assert!(VmName::try_from("vm-1_test").is_ok());
        assert!(VmName::try_from("-invalid").is_err());
        assert!(VmName::try_from("").is_err());
        assert!(VmName::try_from("bad name").is_err());
        assert!(VmName::try_from(".hidden").is_err());
    }
}
