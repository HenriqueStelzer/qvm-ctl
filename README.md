<div align="center">

# qvm-ctl

**Simple, lightweight QEMU/KVM virtual machines from your terminal.**

[![CI](https://github.com/HenriqueStelzer/qvm-ctl/actions/workflows/ci.yml/badge.svg)](https://github.com/HenriqueStelzer/qvm-ctl/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Release](https://img.shields.io/github/v/release/HenriqueStelzer/qvm-ctl?color=brightgreen)](https://github.com/HenriqueStelzer/qvm-ctl/releases)
[![Docs](https://img.shields.io/badge/docs-mdBook-orange.svg)](docs/src/introduction.md)

</div>

---

`qvm-ctl` (installed as `qvm`) is a command-line tool that lets you create, run, and manage virtual machines with single commands. You provide an ISO, and it handles everything required to get a modern OS up and running: virtual disk creation, UEFI firmware, TPM 2.0 emulation, and graphical display.

There are no background services to configure, no complex XML files, and no hypervisor daemons running when your VMs are turned off. Every virtual machine lives in its own directory on your computer, making your environments simple to inspect, move, or delete.

---

## Quickstart

### 1. Install Dependencies

```bash
# Arch Linux
sudo pacman -S qemu-full edk2-ovmf swtpm virt-viewer

# Debian / Ubuntu
sudo apt install qemu-system-x86 qemu-utils ovmf swtpm swtpm-tools virt-viewer

# Fedora
sudo dnf install qemu-kvm qemu-img edk2-ovmf swtpm swtpm-tools virt-viewer
```

### 2. Install qvm

```bash
git clone https://github.com/HenriqueStelzer/qvm-ctl.git
cd qvm-ctl
sudo install -m755 src/qvm-ctl.sh /usr/local/bin/qvm
```

*(Arch Linux users can also run `makepkg -si` from the cloned repo).*

### 3. Create & Launch a VM

```bash
# Create a VM from an ISO
qvm create win11 ~/Downloads/Win11_x64.iso ~/Downloads/virtio-win.iso

# Initial boot with installer media
qvm launch win11

# Future boots from disk (post-installation)
qvm launch win11 --no-iso

# Check status
qvm list

# Graceful shutdown
qvm stop win11
```

---

## Command Reference

| Command | Usage | Description |
| :--- | :--- | :--- |
| `create` | `qvm create <name> <iso> [virtio.iso]` | Provision directory, disk image, and UEFI variables |
| `launch` | `qvm launch <name> [--no-iso]` | Start VM process, TPM daemon, and open SPICE viewer |
| `stop` | `qvm stop <name>` | Graceful ACPI shutdown (SIGTERM with SIGKILL fallback) |
| `list` | `qvm list` | List all VMs, runtime status, and SPICE ports |
| `disable` | `qvm disable <name>` | Stop VM and delete its directory and disk image |
| `version` | `qvm version` | Display current installed version |

---

## Configuration

Control default resource allocations with environment variables:

| Variable | Default | Description |
| :--- | :--- | :--- |
| `QVM_DIR` | `~/vms` | Directory where VMs are stored |
| `QVM_RAM` | `8192` | Default RAM in MB for new VMs |
| `QVM_CPUS` | `4` | Default vCPU count for new VMs |
| `QVM_DISK` | `40G` | Default virtual disk size |
| `NO_COLOR` | *(unset)* | Set to disable ANSI terminal colors |

```bash
# Example: create a development workstation with custom resources
QVM_RAM=16384 QVM_CPUS=8 QVM_DISK=100G qvm create dev-box ~/Downloads/arch.iso
```

Existing VMs can also be customized at any time by editing `~/vms/<name>/vm.conf`.

---

## Documentation

Full in-depth documentation is included in the `docs/` folder as an [mdBook](https://rust-lang.github.io/mdBook/):

```bash
cd docs
mdbook serve --open
```

Or browse the documentation files directly in [`docs/src/`](docs/src/introduction.md):
- [Installation Guide](docs/src/installation.md)
- [Getting Started Guide](docs/src/getting-started.md)
- [Command Details](docs/src/commands/index.md)
- [Architecture & Internals](docs/src/architecture.md)
- [Troubleshooting & FAQ](docs/src/troubleshooting.md)

---

## Specifications

For technical reference, each VM is launched with the following hardware profile:

| Component | Implementation | Details |
| :--- | :--- | :--- |
| Machine Type | Q35 (`q35,accel=kvm`) | Native PCIe bus layout with KVM hardware acceleration |
| CPU | Host Passthrough (`-cpu host`) | Exposes host instruction sets and CPU extensions |
| Firmware | UEFI via OVMF | Dedicated writable NVRAM per VM; Secure Boot supported |
| TPM | TPM 2.0 (`swtpm`) | Emulated via Unix domain socket with CRB interface |
| Graphics | VirtIO GPU (`virtio-gpu-pci`) | SPICE server with dynamic port allocation and clipboard sharing |
| Storage | VirtIO Block (`qcow2`) | Paravirtualized disk image with direct I/O |
| Networking | VirtIO Net (`virtio-net-pci`) | User-mode network stack (SLIRP) |
| Input | USB Tablet (`usb-tablet`) | Absolute pointer device for smooth cursor synchronization |

---

## Contributing

Pull requests and issues are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on project philosophy and the process for suggesting features or submitting fixes.

---

## License

Released under the [MIT License](LICENSE).