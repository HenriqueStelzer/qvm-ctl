use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum QvmError {
    #[error("Missing: {0}. Install it with your package manager.")]
    MissingDependency(String),

    #[error("Invalid VM name '{0}'.")]
    InvalidVmName(String),

    #[error("Windows ISO not found: {0}")]
    IsoNotFound(PathBuf),

    #[error("VirtIO ISO not found: {0}")]
    VirtioIsoNotFound(PathBuf),

    #[error("VM '{0}' already exists")]
    VmAlreadyExists(String),

    #[error("VM '{0}' not found. Run: qvm list")]
    VmNotFound(String),

    #[error("VM '{0}' not found")]
    VmNotFoundPlain(String),

    #[error("VM '{0}' is already launching.")]
    VmAlreadyLaunching(String),

    #[error("VM '{0}' is not running")]
    VmNotRunning(String),

    #[error("Failed to stop VM '{0}'")]
    FailedToStopVm(String),

    #[error("VM '{0}' stopped unexpectedly.")]
    VmStoppedUnexpectedly(String),

    #[error("OVMF_CODE not found. Install: edk2-ovmf")]
    OvmfCodeNotFound,

    #[error("OVMF_VARS not found. Install: edk2-ovmf")]
    OvmfVarsNotFound,

    #[error("OVMF_CODE not found at {0} (stored in vm.conf)")]
    OvmfCodeStoredNotFound(String),

    #[error("OVMF_VARS not found at {0} (stored in vm.conf)")]
    OvmfVarsStoredNotFound(String),

    #[error("Missing vm.conf in {0}")]
    MissingVmConf(String),

    #[error("Failed to start swtpm")]
    FailedToStartTpm,

    #[error("QEMU exited immediately. Check: {0}")]
    QemuExitedImmediately(PathBuf),

    #[error("Could not determine RDP port for VM '{0}'")]
    RdpPortNotFound(String),

    #[error("FreeRDP not found. Install 'freerdp' (wlfreerdp or xfreerdp).")]
    FreeRdpNotFound,

    #[error("Aborted")]
    Aborted,

    #[error("{0}")]
    Custom(String),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}
