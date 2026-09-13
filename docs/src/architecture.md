# Architecture & Internals

`qvm-ctl` is implemented in Rust as a compiled native binary. It manages virtual machines as standalone background processes without hypervisor daemons or runtime interpreter dependencies.

---

## Rust Module Design

The codebase is organized into clean, single-responsibility modules:

- **`cli`:** Command-line argument parsing and validation powered by [`clap`](https://crates.io/crates/clap).
- **`qemu`:** Pure, testable QEMU command builder mapping VM configuration to Q35/KVM parameters.
- **`port`:** Subprocess-free dynamic TCP port allocation using `std::net::TcpListener` (starting at port 5900 for SPICE and 3389 for RDP).
- **`process`:** Subprocess management, PID file tracking, and graceful signal dispatch via [`nix`](https://crates.io/crates/nix).
- **`ovmf`:** Multi-distribution discovery for UEFI `OVMF_CODE` and `OVMF_VARS` firmware templates across Arch, Debian/Ubuntu, and Fedora.
- **`tpm`:** Lifecycle supervisor for `swtpm` software TPM 2.0 daemon and Unix control sockets.
- **`config`:** Strongly-typed parser and serializer for `vm.conf` key-value pairs and environment overrides.
- **`error`:** Comprehensive error domain modeled with [`thiserror`](https://crates.io/crates/thiserror) and [`anyhow`](https://crates.io/crates/anyhow).

---

## Directory Layout

Each VM is self-contained in `~/vms/<name>/`:

```text
~/vms/<name>/
├── vm.conf          # VM configuration and hardware parameters
├── disk.qcow2       # Primary QCOW2 sparse virtual disk
├── ovmf-vars.fd     # Independent writable UEFI NVRAM
├── qemu.log         # Process stdout & stderr log
├── spice.port       # Currently allocated SPICE port
├── rdp.port         # Currently allocated RDP port
├── <name>.pid       # QEMU process ID
├── swtpm.pid        # swtpm process ID
├── swtpm.sock       # Unix control socket for swtpm
└── tpm/             # Persistent TPM 2.0 state directory
```

---

## Hardware Configuration

When `qvm launch` runs, it executes `qemu-system-x86_64` with:

- **Chipset:** `q35` with `accel=kvm` and host CPU passthrough (`-cpu host`).
- **Firmware:** UEFI via read-only `OVMF_CODE` and per-VM writable `ovmf-vars.fd`.
- **Security:** Hardware TPM 2.0 emulation via `swtpm` on a dedicated Unix socket (`-device tpm-crb`).
- **Graphics:** `virtio-gpu-pci` with SPICE server and `vdagent` for clipboard synchronization and auto-resizing.
- **Input:** USB tablet pointer device (`-device usb-tablet`) for lag-free cursor tracking.
- **Storage:** Paravirtualized VirtIO block device (`-drive file=$DISK,if=virtio,format=qcow2`).
- **Networking:** User-mode networking with VirtIO (`-nic user,model=virtio-net-pci`) with host port-forwarding for RDP.

---

## Port & Concurrency Management

- **Port Allocation:** Dynamic scanning with native `std::net::TcpListener::bind` eliminates external subprocess overhead (such as `ss`) while guaranteeing no port collisions.
- **Launch Locking:** Uses cross-process advisory locking (`fd-lock`) on `.launch.lock` to prevent accidental concurrent execution of the same VM.
