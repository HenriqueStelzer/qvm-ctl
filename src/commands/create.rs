use crate::config::{EnvDefaults, VmConfig};
use crate::display::{info, ok, warn};
use crate::error::QvmError;
use crate::ovmf::find_ovmf;
use crate::vm::{vm_dir, VmName};
use std::fs;
use std::path::Path;
use std::process::Command;

pub fn run(
    base: &Path,
    name_raw: String,
    iso: String,
    driver_iso: Option<String>,
) -> Result<(), QvmError> {
    let vm_name = VmName::try_from(name_raw)?;
    let name = vm_name.as_str();

    let iso_path = Path::new(&iso);
    if !iso_path.is_file() {
        return Err(QvmError::IsoNotFound(iso_path.to_path_buf()));
    }

    let driver_iso_path = driver_iso.as_ref().map(Path::new);
    if let Some(p) = driver_iso_path {
        if !p.is_file() {
            return Err(QvmError::VirtioIsoNotFound(p.to_path_buf()));
        }
    }

    let vmdir = vm_dir(base, name);
    if vmdir.exists() {
        return Err(QvmError::VmAlreadyExists(name.to_string()));
    }

    // Verify qemu dependencies
    which::which("qemu-system-x86_64")
        .map_err(|_| QvmError::MissingDependency("qemu-system-x86_64".to_string()))?;
    which::which("qemu-img").map_err(|_| QvmError::MissingDependency("qemu-img".to_string()))?;

    let ovmf = find_ovmf()?;
    let defaults = EnvDefaults::load();

    fs::create_dir_all(&vmdir)?;

    let disk_path = vmdir.join("disk.qcow2");
    let img_status = Command::new("qemu-img")
        .args([
            "create",
            "-f",
            "qcow2",
            &disk_path.to_string_lossy(),
            &defaults.disk_size,
            "-q",
        ])
        .status()?;

    if !img_status.success() {
        return Err(QvmError::Custom(
            "Failed to create disk image with qemu-img".to_string(),
        ));
    }

    let vars_dest = vmdir.join("ovmf-vars.fd");
    fs::copy(&ovmf.vars, &vars_dest)?;

    // Timestamp
    let now = chrono_now_iso();

    let config = VmConfig {
        name: name.to_string(),
        iso: iso_path.to_path_buf(),
        driver_iso: driver_iso_path.map(|p| p.to_path_buf()),
        disk: disk_path,
        ovmf_code: ovmf.code,
        ovmf_vars: vars_dest,
        ram_mb: defaults.ram_mb,
        vcpus: defaults.vcpus,
        created: now,
        rdp_user: None,
        rdp_pass: None,
    };

    config.save(&vmdir)?;

    ok(format!("Created VM: {}", name));
    info(format!("RAM: {} MB", defaults.ram_mb));
    info(format!("vCPUs: {}", defaults.vcpus));
    info(format!("Disk: {}", defaults.disk_size));

    if ovmf.secure_boot {
        ok("Secure Boot-capable OVMF detected");
    } else {
        warn("Secure Boot OVMF not detected");
    }

    if let Some(ref d_iso) = driver_iso {
        info(format!("VirtIO drivers: {}", d_iso));
    }

    Ok(())
}

fn chrono_now_iso() -> String {
    // Generate ISO 8601 string without external heavy crate or via date command
    let output = Command::new("date").arg("-Iseconds").output();
    if let Ok(out) = output {
        if out.status.success() {
            return String::from_utf8_lossy(&out.stdout).trim().to_string();
        }
    }
    "unknown".to_string()
}

mod which {
    use std::path::PathBuf;
    use std::process::Command;

    pub fn which(name: &str) -> Result<PathBuf, ()> {
        let out = Command::new("which").arg(name).output();
        match out {
            Ok(o) if o.status.success() => {
                let s = String::from_utf8_lossy(&o.stdout).trim().to_string();
                Ok(PathBuf::from(s))
            }
            _ => Err(()),
        }
    }
}
