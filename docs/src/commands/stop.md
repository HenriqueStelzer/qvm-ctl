# qvm stop

Gracefully powers down a running VM.

---

## Usage

```bash
qvm stop <name>
```

---

## How It Works

1. Sends `SIGTERM` to the QEMU process, triggering an ACPI power button event in the guest OS for clean shutdown.
2. Waits up to 10 seconds for the guest to cleanly shut down and flush disks.
3. If the guest does not exit within 10 seconds, automatically escalates to `SIGKILL`.
4. Saves TPM state via `swtpm_ioctl --save` and terminates the `swtpm` daemon.
5. Cleans up PID and socket files.

---

## Example

```bash
qvm stop win11
# Output:
# → Stopping win11...
# ✓ Stopped win11
```
