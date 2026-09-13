use fd_lock::RwLock;
use std::fs::{File, OpenOptions};
use std::path::Path;
use std::process::{Command, Stdio};
use std::thread::sleep;
use std::time::Duration;

use crate::config::VmConfig;
use crate::display::{info, ok, warn};
use crate::error::QvmError;
use crate::port::{next_rdp_port, next_spice_port};
use crate::process;
use crate::qemu::{build_qemu_command, LaunchParams};
use crate::tpm;
use crate::vm;

pub fn run(base: &Path, name: String, no_iso: bool, headless: bool) -> Result<(), QvmError> {
    let vmdir = vm::vm_dir(base, &name);
    if !vmdir.is_dir() {
        return Err(QvmError::VmNotFound(name));
    }

    let config = VmConfig::load(&vmdir)?;

    // Lock scope: flock on .launch.lock held during check, spawn, and PID file write
    let lock_path = vmdir.join(".launch.lock");
    let lock_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(false)
        .open(&lock_path)?;

    let mut rw_lock = RwLock::new(lock_file);
    let _guard = match rw_lock.try_write() {
        Ok(guard) => guard,
        Err(_) => return Err(QvmError::VmAlreadyLaunching(name)),
    };

    vm::cleanup_stale_files(base, &name);

    if vm::vm_is_running(base, &name) {
        let port = std::fs::read_to_string(vm::vm_port_file(base, &name))
            .unwrap_or_else(|_| "?".to_string())
            .trim()
            .to_string();
        let rdp_p = std::fs::read_to_string(vm::vm_rdp_port_file(base, &name))
            .unwrap_or_else(|_| "?".to_string())
            .trim()
            .to_string();

        warn(format!(
            "Already running → spice://localhost:{} | rdp://localhost:{}",
            port, rdp_p
        ));
        return Ok(());
    }

    if !config.ovmf_code.is_file() {
        return Err(QvmError::OvmfCodeStoredNotFound(
            config.ovmf_code.display().to_string(),
        ));
    }
    if !config.ovmf_vars.is_file() {
        return Err(QvmError::OvmfVarsStoredNotFound(
            config.ovmf_vars.display().to_string(),
        ));
    }

    let spice_port = next_spice_port();
    std::fs::write(vm::vm_port_file(base, &name), format!("{}\n", spice_port))?;

    let rdp_port = next_rdp_port();
    std::fs::write(vm::vm_rdp_port_file(base, &name), format!("{}\n", rdp_port))?;

    let log_path = vm::vm_log_file(base, &name);
    let log_file = File::create(&log_path)?;

    tpm::start_tpm(base, &name)?;

    let params = LaunchParams {
        vcpus: config.vcpus,
        ram_mb: config.ram_mb,
        ovmf_code: config.ovmf_code,
        ovmf_vars: config.ovmf_vars,
        tpm_socket: vm::vm_tpm_socket(base, &name),
        spice_port,
        rdp_port,
        disk: config.disk,
        attach_iso: !no_iso,
        iso: config.iso.clone(),
        driver_iso: config.driver_iso.clone(),
    };

    if !no_iso && config.iso.is_file() {
        info("Windows ISO attached");
    } else if !no_iso && !config.iso.is_file() {
        warn(format!("ISO not found at {}", config.iso.display()));
        info("Booting from disk");
    } else {
        info("Booting from disk");
    }

    if let Some(ref d_iso) = config.driver_iso {
        if d_iso.is_file() {
            info("VirtIO driver ISO attached");
        }
    }

    info("Starting TPM 2.0...");

    let (prog, args) = build_qemu_command(&params);

    let child = Command::new(prog)
        .args(args)
        .stdout(Stdio::from(log_file.try_clone()?))
        .stderr(Stdio::from(log_file))
        .spawn();

    let child = match child {
        Ok(c) => c,
        Err(e) => {
            tpm::stop_tpm(base, &name);
            let _ = std::fs::remove_file(vm::vm_port_file(base, &name));
            let _ = std::fs::remove_file(vm::vm_rdp_port_file(base, &name));
            return Err(QvmError::Io(e));
        }
    };

    let pid = child.id() as i32;
    process::write_pid_file(&vm::vm_pid_file(base, &name), pid)?;

    sleep(Duration::from_secs(1));

    if !vm::vm_is_running(base, &name) {
        tpm::stop_tpm(base, &name);
        let _ = std::fs::remove_file(vm::vm_pid_file(base, &name));
        let _ = std::fs::remove_file(vm::vm_port_file(base, &name));
        let _ = std::fs::remove_file(vm::vm_rdp_port_file(base, &name));

        return Err(QvmError::QemuExitedImmediately(log_path));
    }

    ok(format!(
        "Running → PID {} | spice://localhost:{} | rdp://localhost:{}",
        pid, spice_port, rdp_port
    ));

    if headless {
        info("Running in headless mode (no SPICE viewer)");
        return Ok(());
    }

    // Attempt viewer
    let viewer_url = format!("spice://localhost:{}", spice_port);
    if crate::process::which("remote-viewer").is_some() {
        let _ = Command::new("remote-viewer").arg(&viewer_url).spawn();
    } else if crate::process::which("virt-viewer").is_some() {
        let _ = Command::new("virt-viewer")
            .args(["--connect", &viewer_url])
            .spawn();
    } else {
        warn("No SPICE viewer found.");
        info("Install: virt-viewer");
        info(format!("Connect to: {}", viewer_url));
    }

    Ok(())
}
