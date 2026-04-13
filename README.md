# qvm-ctl

QEMU/KVM wrapper. Repeatable VM lifecycles without libvirt.

## Requirements

- qemu (`qemu-system-x86_64`, `qemu-img`)
- OVMF/edk2 (UEFI firmware)
- bash ≥ 4
- SPICE viewer (`remote-viewer` or `virt-viewer`) — optional, for display

Arch: `sudo pacman -S qemu-full edk2-ovmf virt-viewer`  
Debian/Ubuntu: `sudo apt install qemu-system-x86 qemu-utils ovmf virt-viewer`

## Install

    git clone https://github.com/youruser/qvm-ctl.git
    cd qvm-ctl
    sudo install -m755 qvm-ctl.sh /usr/local/bin/qvm

The installed command is `qvm`. The repo name is `qvm-ctl`.

## Usage

    qvm create <name> <iso>     Create a VM
    qvm launch <name>           Boot with ISO attached
    qvm launch <name> --no-iso  Boot from disk only
    qvm stop   <name>           Graceful shutdown
    qvm list                    Show all VMs
    qvm disable <name>          Stop + delete a VM
    qvm version                 Print version

VMs are stored in `~/vms/<name>/` by default. Override with `QVM_DIR`:

    QVM_DIR=/data/vms qvm create archlinux /path/to/archlinux.iso
    QVM_DIR=/data/vms qvm launch archlinux

The name passed to `create` becomes a subdirectory of `QVM_DIR`. Do not
create that directory beforehand — `qvm create` will fail if it already exists.

## Environment

    QVM_DIR    VM storage (default: ~/vms)
    QVM_RAM    RAM in MB  (default: 8192)
    QVM_CPUS   vCPUs      (default: 4)
    QVM_DISK   Disk size  (default: 40G)