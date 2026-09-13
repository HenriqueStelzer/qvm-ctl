use std::net::TcpStream;
use std::path::Path;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

use crate::config::VmConfig;
use crate::display::{info, warn};
use crate::error::QvmError;
use crate::vm;

pub fn run(base: &Path, name: String, app_cmd: String, args: Vec<String>) -> Result<(), QvmError> {
    let vmdir = vm::vm_dir(base, &name);
    if !vmdir.is_dir() {
        return Err(QvmError::VmNotFound(name));
    }

    let config = VmConfig::load(&vmdir)?;

    let mut just_launched = false;
    if !vm::vm_is_running(base, &name) {
        info(format!("VM '{}' is stopped. Booting headless...", name));
        super::launch::run(base, name.clone(), true, true)?;
        just_launched = true;
    }

    let rdp_port_str = std::fs::read_to_string(vm::vm_rdp_port_file(base, &name))
        .map_err(|_| QvmError::RdpPortNotFound(name.clone()))?;
    let rdp_port = rdp_port_str
        .trim()
        .parse::<u16>()
        .map_err(|_| QvmError::RdpPortNotFound(name.clone()))?;

    let is_wayland = std::env::var_os("WAYLAND_DISPLAY").is_some();
    let rdp_client = if is_wayland && which("wlfreerdp") {
        "wlfreerdp"
    } else if which("xfreerdp") {
        "xfreerdp"
    } else if which("wlfreerdp") {
        "wlfreerdp"
    } else {
        return Err(QvmError::FreeRdpNotFound);
    };

    if just_launched {
        info("Waiting for guest RDP service to become ready...");
        let mut ready = false;
        for _ in 0..30 {
            if !vm::vm_is_running(base, &name) {
                return Err(QvmError::VmStoppedUnexpectedly(name));
            }
            if TcpStream::connect_timeout(
                &format!("127.0.0.1:{}", rdp_port).parse().unwrap(),
                Duration::from_secs(1),
            )
            .is_ok()
            {
                ready = true;
                break;
            }
            sleep(Duration::from_secs(1));
        }

        if !ready {
            warn("Guest RDP port did not respond in 30s. Attempting to connect anyway...");
        }
    }

    let env_user = std::env::var("QVM_RDP_USER").ok();
    let current_user = std::env::var("USER").unwrap_or_else(|_| "user".to_string());
    let user = env_user.or(config.rdp_user).unwrap_or(current_user);

    let env_pass = std::env::var("QVM_RDP_PASS").ok();
    let pass = env_pass.or(config.rdp_pass);

    let mut cmd = Command::new(rdp_client);
    cmd.arg(format!("/v:127.0.0.1:{}", rdp_port))
        .arg(format!("/u:{}", user))
        .arg(format!("/app:{}", app_cmd))
        .arg("/cert:ignore")
        .arg("+clipboard")
        .arg("+dynamic-resolution");

    if let Some(ref p) = pass {
        cmd.arg(format!("/p:{}", p));
    }

    if !args.is_empty() {
        cmd.arg(format!("/app-cmd:{}", args.join(" ")));
    }

    info(format!(
        "Launching '{}' on '{}' via {} (127.0.0.1:{})...",
        app_cmd, name, rdp_client, rdp_port
    ));

    let status = cmd.status()?;
    if !status.success() {
        // Return Ok or non-zero depending on FreeRDP exit
    }

    Ok(())
}

fn which(name: &str) -> bool {
    Command::new("which")
        .arg(name)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}
