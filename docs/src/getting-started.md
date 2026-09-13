# Getting Started

Follow these 4 steps to create and run your first virtual machine.

---

## 1. Create a VM

Run `qvm create <name> <iso> [virtio_driver_iso]`:

```bash
# Linux VM
qvm create my-arch ~/Downloads/archlinux-x86_64.iso

# Windows VM (with optional VirtIO driver ISO)
qvm create win11 ~/Downloads/Win11_x64.iso ~/Downloads/virtio-win.iso
```

This creates a dedicated directory at `~/vms/<name>/` with a 40GB `qcow2` virtual disk and UEFI NVRAM.

---

## 2. Boot with Installer ISO

Start the VM for installation:

```bash
qvm launch win11
```

- QEMU and TPM 2.0 (`swtpm`) start in the background.
- If `remote-viewer` or `virt-viewer` is installed, the graphical display opens automatically.
- Otherwise, connect with any SPICE client: `remote-viewer spice://localhost:5900`.

---

## 3. Boot from Disk (Post-Install)

Once OS installation is complete, shut down the VM. For future boots, start directly from disk without attaching the installer ISO:

```bash
qvm launch win11 --no-iso
```

---

## 4. Manage Lifecycle

```bash
# View all VMs and their status
qvm list

# Gracefully shut down a running VM
qvm stop win11

# Delete a VM and remove its files (requires typing VM name to confirm)
qvm disable win11
```
