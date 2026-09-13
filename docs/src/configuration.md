# Configuration

## Environment Overrides

Set these variables in your shell environment or prefix them to `qvm create`:

| Variable | Default | Description |
| :--- | :--- | :--- |
| `QVM_DIR` | `~/vms` | Root directory for all VM storage |
| `QVM_RAM` | `8192` | Default RAM in MB for new VMs |
| `QVM_CPUS` | `4` | Default vCPU count for new VMs |
| `QVM_DISK` | `40G` | Default disk size for new VMs |
| `QVM_RDP_USER` | `$USER` | Default username for `qvm app` FreeRDP sessions |
| `QVM_RDP_PASS` | *(unset)* | Default password for `qvm app` FreeRDP sessions |
| `NO_COLOR` | *(unset)* | Set to disable ANSI color codes |

### Example
```bash
# Create a VM in a custom path with 16GB RAM and 8 vCPUs
QVM_DIR=/mnt/storage/vms QVM_RAM=16384 QVM_CPUS=8 qvm create bigvm ~/iso/ubuntu.iso
```

---

## Per-VM Config (`vm.conf`)

Each VM stores its configuration in `~/vms/<name>/vm.conf`:

```ini
NAME=win11
ISO=/path/to/installer.iso
DRIVER_ISO=/path/to/virtio.iso
DISK=/home/user/vms/win11/disk.qcow2
OVMF_CODE=/usr/share/edk2/x64/OVMF_CODE_4M.secboot.fd
OVMF_VARS=/home/user/vms/win11/ovmf-vars.fd
RAM_MB=8192
VCPUS=4
CREATED=2026-09-13T18:00:00+00:00
# Optional credentials for 'qvm app'
RDP_USER=myusername
RDP_PASS=mypassword
```

To modify CPU or RAM for an existing VM, stop the VM and edit `RAM_MB` or `VCPUS` directly in this file. You can also persist default credentials for RemoteApp by adding `RDP_USER` (and optionally `RDP_PASS`).
