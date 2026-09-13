# Introduction

**`qvm-ctl`** (installed as `qvm`) is a fast, lightweight CLI tool written in Rust to manage repeatable QEMU/KVM virtual machine lifecycles without libvirt.

---

## What It Does

`qvm-ctl` is a command-line tool that lets you create, run, and manage QEMU/KVM virtual machines with simple commands instead of dealing with complex hypervisors. It automatically configures UEFI boot, TPM 2.0 emulation, virtual disks, and graphical displays so modern Linux and Windows installations work immediately out of the box. Each virtual machine is kept self-contained in its own folder on your computer, making your VMs easy to locate, inspect, and delete.

---

## Built with Rust

Written from the ground up in idiomatic Rust, `qvm-ctl` emphasizes speed, safety, and operational reliability:
- **Zero Runtime Overhead:** Compiled into a single binary with no interpreter dependencies or background hypervisor daemons.
- **Reliable Process Lifecycle:** Direct OS signal dispatch and process supervision via native system libraries.
- **Deterministic Port & Concurrency Management:** Built-in TCP socket probing and cross-process file locking guarantee conflict-free multi-VM execution.

---

## Next Steps

To get started, follow the [Installation](installation.md) guide to install the required system packages and set up the `qvm` command on your system. Next, check out the [Getting Started](getting-started.md) walkthrough to create and boot your first VM, or jump into the [Command Reference](commands/index.md) for full command syntax. You can also explore [Configuration](configuration.md) to tweak hardware defaults, or visit the [GitHub repository](https://github.com/HenriqueStelzer/qvm-ctl) to view the source code and report issues.
