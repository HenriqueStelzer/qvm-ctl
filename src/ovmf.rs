use crate::error::QvmError;
use std::path::{Path, PathBuf};

pub struct OvmfInfo {
    pub code: PathBuf,
    pub vars: PathBuf,
    pub secure_boot: bool,
}

pub fn find_ovmf() -> Result<OvmfInfo, QvmError> {
    let code_paths = [
        "/usr/share/edk2/x64/OVMF_CODE_4M.secboot.fd",
        "/usr/share/edk2/x64/OVMF_CODE.secboot.fd",
        "/usr/share/edk2/x64/OVMF_CODE_4M.fd",
        "/usr/share/edk2/x64/OVMF_CODE.fd",
        "/usr/share/OVMF/OVMF_CODE_4M.secboot.fd",
        "/usr/share/OVMF/OVMF_CODE_4M.fd",
        "/usr/share/OVMF/OVMF_CODE_4M.ms.fd",
        "/usr/share/OVMF/x64/OVMF_CODE.secboot.4m.fd",
        "/usr/share/OVMF/x64/OVMF_CODE.4m.fd",
        "/usr/share/OVMF/OVMF_CODE.secboot.fd",
        "/usr/share/OVMF/OVMF_CODE.fd",
        "/usr/share/qemu/OVMF_CODE.secboot.fd",
        "/usr/share/qemu/OVMF_CODE.fd",
    ];

    let vars_paths = [
        "/usr/share/edk2/x64/OVMF_VARS_4M.ms.fd",
        "/usr/share/edk2/x64/OVMF_VARS.ms.fd",
        "/usr/share/edk2/x64/OVMF_VARS_4M.fd",
        "/usr/share/edk2/x64/OVMF_VARS.fd",
        "/usr/share/OVMF/OVMF_VARS_4M.ms.fd",
        "/usr/share/OVMF/OVMF_VARS_4M.fd",
        "/usr/share/OVMF/x64/OVMF_VARS.ms.4m.fd",
        "/usr/share/OVMF/x64/OVMF_VARS.4m.fd",
        "/usr/share/OVMF/OVMF_VARS.ms.fd",
        "/usr/share/OVMF/OVMF_VARS.fd",
        "/usr/share/qemu/OVMF_VARS.ms.fd",
        "/usr/share/qemu/OVMF_VARS.fd",
    ];

    let mut found_code = None;
    let mut secure_boot = false;

    for path_str in code_paths {
        let p = Path::new(path_str);
        if p.is_file() {
            found_code = Some(p.to_path_buf());
            if path_str.contains("secboot") {
                secure_boot = true;
            }
            break;
        }
    }

    let mut found_vars = None;
    for path_str in vars_paths {
        let p = Path::new(path_str);
        if p.is_file() {
            found_vars = Some(p.to_path_buf());
            break;
        }
    }

    let code = found_code.ok_or(QvmError::OvmfCodeNotFound)?;
    let vars = found_vars.ok_or(QvmError::OvmfVarsNotFound)?;

    Ok(OvmfInfo {
        code,
        vars,
        secure_boot,
    })
}
