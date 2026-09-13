use std::fs;
use std::io::{self, BufRead, Write};
use std::path::Path;

use crate::display::{ok, warn};
use crate::error::QvmError;
use crate::vm;

pub fn run(base: &Path, name: String) -> Result<(), QvmError> {
    let vmdir = vm::vm_dir(base, &name);
    if !vmdir.is_dir() {
        return Err(QvmError::VmNotFoundPlain(name));
    }

    print!("Type '{}' to confirm deletion: ", name);
    io::stdout().flush()?;

    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line)?;

    let confirm = line.trim();
    if confirm != name {
        return Err(QvmError::Aborted);
    }

    if vm::vm_is_running(base, &name) {
        warn("Stopping running VM...");
        let _ = super::stop::run(base, name.clone());
    }

    fs::remove_dir_all(&vmdir)?;
    ok(format!("Deleted {}", name));

    Ok(())
}
