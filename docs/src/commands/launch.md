# qvm launch

Boots a virtual machine in the background and connects to its display.

---

## Usage

```bash
qvm launch <name> [--no-iso]
```

### Flags

- `--no-iso`: Boots from the virtual disk only, without mounting installation or driver ISOs. Use this for normal day-to-day boots after OS installation.

---

## What It Does

1. Locks the VM to prevent concurrent duplicate launches.
2. Allocates the next available SPICE port (starting at 5900).
3. Starts an isolated TPM 2.0 emulator daemon (`swtpm`).
4. Launches QEMU with KVM acceleration, UEFI firmware, and VirtIO hardware.
5. Spawns `remote-viewer` or `virt-viewer` if installed.

---

## Examples

```bash
# Boot with installer ISO attached (for installation)
qvm launch win11

# Boot directly from disk (post-installation)
qvm launch win11 --no-iso
```

### Headless / Remote Connection

If no display viewer is installed on the host, connect manually via SPICE:
```bash
remote-viewer spice://localhost:5900
```
