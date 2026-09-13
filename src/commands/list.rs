use std::fs;
use std::path::Path;

use crate::config::VmConfig;
use crate::display::{is_color_enabled, sep, warn};
use crate::error::QvmError;
use crate::vm;

pub fn run(base: &Path) -> Result<(), QvmError> {
    sep();

    if !base.is_dir() {
        warn("No VMs found.");
        sep();
        return Ok(());
    }

    let mut found = false;
    let entries = fs::read_dir(base)?;

    let mut dirs: Vec<_> = entries
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
        .collect();
    dirs.sort_by_key(|e| e.file_name());

    for dir_entry in dirs {
        let path = dir_entry.path();
        if !path.join("vm.conf").is_file() {
            continue;
        }

        found = true;
        let config = match VmConfig::load(&path) {
            Ok(c) => c,
            Err(_) => continue,
        };

        let name = &config.name;
        if vm::vm_is_running(base, name) {
            let port = fs::read_to_string(path.join("spice.port"))
                .unwrap_or_else(|_| "?".to_string())
                .trim()
                .to_string();
            let rdp_p = fs::read_to_string(path.join("rdp.port"))
                .unwrap_or_else(|_| "?".to_string())
                .trim()
                .to_string();

            if is_color_enabled() {
                println!(
                    "  \x1b[1m{}\x1b[0m  \x1b[0;32mrunning\x1b[0m  spice://localhost:{}  rdp://localhost:{}",
                    name, port, rdp_p
                );
            } else {
                println!(
                    "  {}  running  spice://localhost:{}  rdp://localhost:{}",
                    name, port, rdp_p
                );
            }
        } else if is_color_enabled() {
            println!("  \x1b[1m{}\x1b[0m  \x1b[1;33mstopped\x1b[0m", name);
        } else {
            println!("  {}  stopped", name);
        }
    }

    if !found {
        warn("No VMs found.");
    }

    sep();
    Ok(())
}
