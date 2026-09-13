# Architecture & Internals

`qvm-ctl` manages virtual machines as standalone background processes without system daemons.

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
- **Networking:** User-mode networking with VirtIO (`-nic user,model=virtio-net-pci`).

---

## Port & Concurrency Management

- **Port Allocation:** Dynamic scanning with `ss` starting at port 5900 ensures multiple VMs run concurrently without collisions.
- **Launch Locking:** Uses `flock` on `.launch.lock` to prevent accidental dual-execution of the same VM.
