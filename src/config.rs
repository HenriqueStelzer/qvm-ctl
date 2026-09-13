use crate::error::QvmError;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Default)]
pub struct VmConfig {
    pub name: String,
    pub iso: PathBuf,
    pub driver_iso: Option<PathBuf>,
    pub disk: PathBuf,
    pub ovmf_code: PathBuf,
    pub ovmf_vars: PathBuf,
    pub ram_mb: u32,
    pub vcpus: u32,
    pub created: String,
    pub rdp_user: Option<String>,
    pub rdp_pass: Option<String>,
}

impl VmConfig {
    pub fn load(vmdir: &Path) -> Result<Self, QvmError> {
        let conf_path = vmdir.join("vm.conf");
        if !conf_path.is_file() {
            return Err(QvmError::MissingVmConf(vmdir.display().to_string()));
        }

        let content = fs::read_to_string(&conf_path)?;
        let mut config = VmConfig::default();

        for line in content.lines() {
            let line = line.trim_end_matches('\r').trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if let Some((key, val)) = line.split_once('=') {
                let key = key.trim();
                let val = val.trim();
                match key {
                    "NAME" => config.name = val.to_string(),
                    "ISO" => config.iso = PathBuf::from(val),
                    "DRIVER_ISO" => {
                        if !val.is_empty() {
                            config.driver_iso = Some(PathBuf::from(val));
                        }
                    }
                    "DISK" => config.disk = PathBuf::from(val),
                    "OVMF_CODE" => config.ovmf_code = PathBuf::from(val),
                    "OVMF_VARS" => config.ovmf_vars = PathBuf::from(val),
                    "RAM_MB" => {
                        if let Ok(num) = val.parse::<u32>() {
                            config.ram_mb = num;
                        }
                    }
                    "VCPUS" => {
                        if let Ok(num) = val.parse::<u32>() {
                            config.vcpus = num;
                        }
                    }
                    "CREATED" => config.created = val.to_string(),
                    "RDP_USER" => {
                        config.rdp_user = if val.is_empty() {
                            None
                        } else {
                            Some(val.to_string())
                        };
                    }
                    "RDP_PASS" => {
                        config.rdp_pass = if val.is_empty() {
                            None
                        } else {
                            Some(val.to_string())
                        };
                    }
                    _ => {} // Ignore unknown keys for forward compatibility
                }
            }
        }

        Ok(config)
    }

    pub fn save(&self, vmdir: &Path) -> Result<(), QvmError> {
        let conf_path = vmdir.join("vm.conf");
        let driver_iso_str = self
            .driver_iso
            .as_ref()
            .map(|p| p.display().to_string())
            .unwrap_or_default();

        let mut content = format!(
            "NAME={}\nISO={}\nDRIVER_ISO={}\nDISK={}\nOVMF_CODE={}\nOVMF_VARS={}\nRAM_MB={}\nVCPUS={}\nCREATED={}\n",
            self.name,
            self.iso.display(),
            driver_iso_str,
            self.disk.display(),
            self.ovmf_code.display(),
            self.ovmf_vars.display(),
            self.ram_mb,
            self.vcpus,
            self.created
        );

        if let Some(user) = &self.rdp_user {
            content.push_str(&format!("RDP_USER={}\n", user));
        }
        if let Some(pass) = &self.rdp_pass {
            content.push_str(&format!("RDP_PASS={}\n", pass));
        }

        fs::write(conf_path, content)?;
        Ok(())
    }
}

pub struct EnvDefaults {
    pub vm_dir: PathBuf,
    pub ram_mb: u32,
    pub vcpus: u32,
    pub disk_size: String,
}

impl EnvDefaults {
    pub fn load() -> Self {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let vm_dir = match std::env::var_os("QVM_DIR") {
            Some(val) => PathBuf::from(val),
            None => PathBuf::from(home).join("vms"),
        };

        let ram_mb = std::env::var("QVM_RAM")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(8192);

        let vcpus = std::env::var("QVM_CPUS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(4);

        let disk_size = std::env::var("QVM_DISK").unwrap_or_else(|_| "40G".to_string());

        Self {
            vm_dir,
            ram_mb,
            vcpus,
            disk_size,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_load_and_save_config() {
        let dir = tempdir().unwrap();
        let config = VmConfig {
            name: "testvm".to_string(),
            iso: PathBuf::from("/path/to/test.iso"),
            driver_iso: Some(PathBuf::from("/path/to/virtio.iso")),
            disk: dir.path().join("disk.qcow2"),
            ovmf_code: PathBuf::from("/usr/share/OVMF/OVMF_CODE.fd"),
            ovmf_vars: dir.path().join("ovmf-vars.fd"),
            ram_mb: 4096,
            vcpus: 2,
            created: "2026-09-13T18:00:00Z".to_string(),
            rdp_user: Some("testuser".to_string()),
            rdp_pass: Some("testpass".to_string()),
        };

        config.save(dir.path()).unwrap();
        let loaded = VmConfig::load(dir.path()).unwrap();

        assert_eq!(loaded.name, "testvm");
        assert_eq!(loaded.iso, PathBuf::from("/path/to/test.iso"));
        assert_eq!(
            loaded.driver_iso,
            Some(PathBuf::from("/path/to/virtio.iso"))
        );
        assert_eq!(loaded.ram_mb, 4096);
        assert_eq!(loaded.vcpus, 2);
        assert_eq!(loaded.rdp_user.as_deref(), Some("testuser"));
        assert_eq!(loaded.rdp_pass.as_deref(), Some("testpass"));
    }
}
