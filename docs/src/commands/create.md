# qvm create

Creates and initializes a new virtual machine.

---

## Usage

```bash
qvm create <name> <iso> [driver_iso]
```

### Arguments

- `<name>`: Alphanumeric VM identifier (can include `-` and `_`).
- `<iso>`: Path to the installer ISO image.
- `[driver_iso]` *(optional)*: Path to auxiliary driver ISO (e.g. VirtIO drivers for Windows).

---

## What It Creates

Stores everything under `~/vms/<name>/` (override via `QVM_DIR`):

- `vm.conf`: Saved VM hardware configuration.
- `disk.qcow2`: QCOW2 sparse virtual disk (default: 40G, override via `QVM_DISK`).
- `ovmf-vars.fd`: Dedicated writable UEFI NVRAM.

---

## Examples

```bash
# Basic VM
qvm create debian-vm ~/Downloads/debian.iso

# Windows VM with VirtIO drivers
qvm create win11 ~/Downloads/Win11.iso ~/Downloads/virtio-win.iso

# Custom RAM, CPU, and Disk size
QVM_RAM=16384 QVM_CPUS=8 QVM_DISK=80G qvm create dev-box ~/Downloads/arch.iso
```
