use std::ffi::OsString;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct LaunchParams {
    pub vcpus: u32,
    pub ram_mb: u32,
    pub ovmf_code: PathBuf,
    pub ovmf_vars: PathBuf,
    pub tpm_socket: PathBuf,
    pub spice_port: u16,
    pub rdp_port: u16,
    pub disk: PathBuf,
    pub attach_iso: bool,
    pub iso: PathBuf,
    pub driver_iso: Option<PathBuf>,
}

/// Pure function: builds program name and arguments vector.
/// Does not spawn any process, perform any I/O, or modify state.
pub fn build_qemu_command(params: &LaunchParams) -> (OsString, Vec<OsString>) {
    let program = OsString::from("qemu-system-x86_64");
    let mut args: Vec<OsString> = Vec::new();

    // Machine / acceleration
    args.push("-machine".into());
    args.push("q35,accel=kvm".into());
    args.push("-cpu".into());
    args.push("host".into());
    args.push("-smp".into());
    args.push(params.vcpus.to_string().into());
    args.push("-m".into());
    args.push(params.ram_mb.to_string().into());

    // UEFI
    args.push("-drive".into());
    args.push(
        format!(
            "if=pflash,format=raw,readonly=on,file={}",
            params.ovmf_code.display()
        )
        .into(),
    );
    args.push("-drive".into());
    args.push(format!("if=pflash,format=raw,file={}", params.ovmf_vars.display()).into());

    // TPM 2.0
    args.push("-chardev".into());
    args.push(format!("socket,id=chrtpm,path={}", params.tpm_socket.display()).into());
    args.push("-tpmdev".into());
    args.push("emulator,id=tpm0,chardev=chrtpm".into());
    args.push("-device".into());
    args.push("tpm-crb,tpmdev=tpm0".into());

    // Graphics
    args.push("-device".into());
    args.push("virtio-gpu-pci".into());
    args.push("-spice".into());
    args.push(
        format!(
            "addr=127.0.0.1,port={},disable-ticketing=on",
            params.spice_port
        )
        .into(),
    );

    // SPICE agent
    args.push("-device".into());
    args.push("virtio-serial-pci".into());
    args.push("-chardev".into());
    args.push("spicevmc,id=vdagent,name=vdagent".into());
    args.push("-device".into());
    args.push("virtserialport,chardev=vdagent,name=com.redhat.spice.0".into());

    // Network with RDP forwarding
    args.push("-nic".into());
    args.push(
        format!(
            "user,model=virtio-net-pci,hostfwd=tcp:127.0.0.1:{}-:3389",
            params.rdp_port
        )
        .into(),
    );

    // System disk
    args.push("-drive".into());
    args.push(
        format!(
            "file={},if=virtio,format=qcow2,cache=none",
            params.disk.display()
        )
        .into(),
    );

    // USB tablet
    args.push("-device".into());
    args.push("usb-ehci".into());
    args.push("-device".into());
    args.push("usb-tablet".into());

    // No local QEMU window
    args.push("-display".into());
    args.push("none".into());

    // Boot order
    args.push("-boot".into());
    args.push("order=dc".into());

    // Windows installation ISO
    if params.attach_iso && params.iso.is_file() {
        args.push("-drive".into());
        args.push(format!("file={},media=cdrom,readonly=on", params.iso.display()).into());
    }

    // VirtIO driver ISO
    if let Some(ref driver_iso) = params.driver_iso {
        if driver_iso.is_file() {
            args.push("-drive".into());
            args.push(format!("file={},media=cdrom,readonly=on", driver_iso.display()).into());
        }
    }

    (program, args)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pure_qemu_command_builder() {
        let params = LaunchParams {
            vcpus: 4,
            ram_mb: 8192,
            ovmf_code: PathBuf::from("/usr/share/OVMF/OVMF_CODE.fd"),
            ovmf_vars: PathBuf::from("/tmp/ovmf-vars.fd"),
            tpm_socket: PathBuf::from("/tmp/swtpm.sock"),
            spice_port: 5900,
            rdp_port: 3389,
            disk: PathBuf::from("/tmp/disk.qcow2"),
            attach_iso: false,
            iso: PathBuf::from("/tmp/fake.iso"),
            driver_iso: None,
        };

        let (prog, args) = build_qemu_command(&params);
        assert_eq!(prog, "qemu-system-x86_64");

        let args_str: Vec<String> = args
            .into_iter()
            .map(|s| s.to_string_lossy().to_string())
            .collect();
        assert!(args_str.contains(&"-machine".to_string()));
        assert!(args_str.contains(&"q35,accel=kvm".to_string()));
        assert!(args_str.contains(&"-cpu".to_string()));
        assert!(args_str.contains(&"host".to_string()));
        assert!(args_str.contains(&"-smp".to_string()));
        assert!(args_str.contains(&"4".to_string()));
        assert!(args_str.contains(&"-m".to_string()));
        assert!(args_str.contains(&"8192".to_string()));
        assert!(args_str.contains(&"addr=127.0.0.1,port=5900,disable-ticketing=on".to_string()));
        assert!(args_str
            .contains(&"user,model=virtio-net-pci,hostfwd=tcp:127.0.0.1:3389-:3389".to_string()));
    }
}
