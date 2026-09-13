# Troubleshooting

Quick solutions for common issues.

---

### `OVMF_CODE not found. Install: edk2-ovmf`
Install UEFI firmware:
- **Arch:** `sudo pacman -S edk2-ovmf`
- **Debian / Ubuntu:** `sudo apt install ovmf`
- **Fedora:** `sudo dnf install edk2-ovmf`

---

### `QEMU exited immediately. Check: qemu.log`
Check the log:
```bash
cat ~/vms/<name>/qemu.log
```
Common fixes:
- **Permission denied on `/dev/kvm`:** Add user to group: `sudo usermod -aG kvm $USER` and log back in.
- **VT-x / AMD-V disabled:** Enable virtualization in your host motherboard BIOS/UEFI.

---

### Windows installer cannot find the virtual disk
The installer lacks VirtIO storage drivers. Download [virtio-win.iso](https://fedorapeople.org/groups/virt/virtio-win/direct-downloads/latest-virtio/virtio-win.iso) and pass it as the third argument:
```bash
qvm create win11 /path/to/win11.iso /path/to/virtio-win.iso
```
In the installer, select **Load driver** -> browse to the VirtIO CD -> `viostor/w11/amd64`.

---

### `Failed to start swtpm`
Install `swtpm` or clear orphaned sockets:
```bash
# Verify installation
command -v swtpm || sudo pacman -S swtpm

# Clear stale socket if previous run crashed
rm -f ~/vms/<name>/swtpm.sock ~/vms/<name>/swtpm.pid
```

---

### `VM '<name>' is already launching`
If an earlier launch was interrupted, remove the stale lockfile:
```bash
rm -f ~/vms/<name>/.launch.lock
```

---

### Clipboard or auto-resize not working
Install the SPICE guest agent inside the guest OS:
- **Linux guest:** `sudo pacman -S spice-vdagent` (or `sudo apt install spice-vdagent`)
- **Windows guest:** Run `virtio-win-guest-tools.exe` from the VirtIO driver ISO.
