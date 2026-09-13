# qvm list

Shows all existing VMs, execution state, and connection details.

---

## Usage

```bash
qvm list
```

---

## Example Output

```text
────────────────────────────────────────
  my-arch   stopped
  win11     running  spice://localhost:5900
────────────────────────────────────────
```

- **Green `running`:** Process is active with its SPICE URL.
- **Yellow `stopped`:** VM is configured but not running.
