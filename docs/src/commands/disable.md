# qvm disable

Stops and permanently deletes a virtual machine.

---

## Usage

```bash
qvm disable <name>
```

> **Warning:** This will permanently delete the entire VM directory `~/vms/<name>/` and its virtual disk.

---

## Behavior

1. Prompts you to type the VM name to confirm deletion:
   ```text
   Type 'win11' to confirm deletion: win11
   ```
2. If the VM is currently running, stops it automatically.
3. Recursively removes the VM directory (`rm -rf`).

---

## Scripted Deletion

To bypass the interactive confirmation in automated scripts, pipe the name:

```bash
echo "win11" | qvm disable win11
```
