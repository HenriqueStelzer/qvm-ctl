# qvm launch

Boots a virtual machine in the background and connects to its display.

---

## Usage

```bash
qvm launch <name> [--no-iso] [--headless]
```

### Flags

- `--no-iso`: Boots from the virtual disk only, without mounting installation or driver ISOs. Use this for normal day-to-day boots after OS installation.
- `--headless`: Runs the VM in the background without automatically opening a SPICE display viewer (`remote-viewer`/`virt-viewer`). SPICE and RDP services remain accessible over their respective ports.

---

## What It Does

1. Locks the VM to prevent concurrent duplicate launches.
2. Allocates the next available SPICE port (starting at 5900) and saves it to `spice.port`.
3. Allocates the next available RDP port (starting at 3389) and saves it to `rdp.port`.
4. Starts an isolated TPM 2.0 emulator daemon (`swtpm`).
5. Launches QEMU with KVM acceleration, UEFI firmware, VirtIO hardware, and user networking with RDP port forwarding.
6. Spawns `remote-viewer` or `virt-viewer` if installed (unless `--headless` is passed).

---

## Examples

```bash
# Boot with installer ISO attached (for installation)
qvm launch win11

# Boot directly from disk (post-installation)
qvm launch win11 --no-iso

# Boot directly from disk in headless mode (no GUI viewer)
qvm launch win11 --no-iso --headless
```

### Remote / Manual Connection

If running headless or if no viewer is installed on the host:
- **SPICE viewer:**
  ```bash
  remote-viewer spice://localhost:5900
  ```
- **RDP / Remote Desktop:**
  ```bash
  wlfreerdp /v:127.0.0.1:3389 /u:User
  ```
