# Introduction

**`qvm-ctl`** (installed as `qvm`) is a lightweight CLI tool to manage repeatable QEMU/KVM virtual machine lifecycles without libvirt.

---

## What It Does

`qvm-ctl` is a command-line tool that lets you create, run, and manage QEMU/KVM virtual machines with simple commands instead of dealing with complex hypervisors. It automatically configures UEFI boot, TPM 2.0 emulation, virtual disks, and graphical displays so modern Linux and Windows installations work immediately out of the box. Each virtual machine is kept self-contained in its own folder on your computer, making your VMs easy to locate, inspect, and delete.

---

## Next Steps

To get started, follow the [Installation](installation.md) guide to install the required system packages and set up the `qvm` command on your system. Next, check out the [Getting Started](getting-started.md) walkthrough to create and boot your first VM, or jump into the [Command Reference](commands/index.md) for full command syntax. You can also explore [Configuration](configuration.md) to tweak hardware defaults, or visit the [GitHub repository](https://github.com/HenriqueStelzer/qvm-ctl) to view the source code and report issues.
