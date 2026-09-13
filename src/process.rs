use nix::sys::signal::{kill, Signal};
use nix::unistd::Pid;
use std::fs;
use std::path::Path;
use std::process::Command;

pub fn read_pid_file(path: &Path) -> Option<i32> {
    if !path.is_file() {
        return None;
    }
    let content = fs::read_to_string(path).ok()?;
    content.trim().parse::<i32>().ok()
}

pub fn write_pid_file(path: &Path, pid: i32) -> std::io::Result<()> {
    let tmp_path = path.with_extension("pid.tmp");
    fs::write(&tmp_path, format!("{}\n", pid))?;
    fs::rename(tmp_path, path)
}

pub fn is_process_alive(pid: i32) -> bool {
    kill(Pid::from_raw(pid), None).is_ok()
}

pub fn check_process_comm(pid: i32, expected: &str) -> bool {
    // Preserve exact bash behavior: ps -p "$pid" -o comm= | grep -q '^qemu-system-x86_64$'
    let output = Command::new("ps")
        .args(["-p", &pid.to_string(), "-o", "comm="])
        .output();

    match output {
        Ok(out) if out.status.success() => {
            let comm = String::from_utf8_lossy(&out.stdout).trim().to_string();
            comm == expected
        }
        _ => false,
    }
}

pub fn send_signal(pid: i32, sig: Signal) -> bool {
    kill(Pid::from_raw(pid), sig).is_ok()
}

pub fn which(name: &str) -> Option<std::path::PathBuf> {
    let name_path = Path::new(name);
    if name_path.components().count() > 1 {
        if is_executable(name_path) {
            return Some(name_path.to_path_buf());
        }
        return None;
    }

    let paths = std::env::var_os("PATH")?;
    for dir in std::env::split_paths(&paths) {
        let candidate = dir.join(name);
        if is_executable(&candidate) {
            return Some(candidate);
        }
    }
    None
}

fn is_executable(path: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;
    if let Ok(meta) = path.metadata() {
        meta.is_file() && (meta.permissions().mode() & 0o111 != 0)
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_which_lookup() {
        assert!(which("cargo").is_some());
        assert!(which("definitely_nonexistent_binary_xyz_12345").is_none());
    }
}
