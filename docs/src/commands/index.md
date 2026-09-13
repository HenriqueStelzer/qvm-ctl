# Command Reference

Quick summary of all `qvm` subcommands.

---

## Commands

| Command | Usage | Description |
| :--- | :--- | :--- |
| [`create`](create.md) | `qvm create <name> <iso> [driver_iso]` | Provision a new VM directory, disk, and configuration |
| [`launch`](launch.md) | `qvm launch <name> [--no-iso]` | Start VM with or without installation media |
| [`stop`](stop.md) | `qvm stop <name>` | Gracefully shut down a running VM (ACPI SIGTERM) |
| [`list`](list.md) | `qvm list` | Display all VMs and their running/stopped status |
| [`disable`](disable.md) | `qvm disable <name>` | Stop and permanently delete a VM |
| `version` | `qvm version` | Print `qvm` version number |
| `help` | `qvm help` | Display built-in CLI help |

---

## Global Tips

- **Color Output:** Automatically disabled when stdout is piped or when `NO_COLOR=1` is set.
- **Exit Status:** Exits `0` on success and `1` on error with error details printed to `stderr`.
