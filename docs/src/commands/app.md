# qvm app

Launches an individual application inside the guest VM and displays it as a seamless, native window on the host desktop using FreeRDP and Windows RemoteApp (RAIL).

---

## Usage

```bash
qvm app <name> <app_path> [args...]
```

### Arguments

- `<name>`: The VM name.
- `<app_path>`: The full path to the executable inside the guest (e.g. `C:\Program Files\Notepad++\notepad++.exe` or `notepad.exe`).
- `[args...]`: Optional arguments to pass to the guest application.

---

## What It Does

1. Checks if the VM is running. If stopped, automatically starts it in **headless mode** (`--no-iso --headless`).
2. Discovers the VM's assigned RDP port (from `rdp.port`).
3. Waits for the guest RDP service to become responsive.
4. Detects the best available FreeRDP client on the host (`wlfreerdp` on Wayland, falling back to `xfreerdp`).
5. Resolves credentials from environment variables (`QVM_RDP_USER`, `QVM_RDP_PASS`) or `vm.conf` (`RDP_USER`, `RDP_PASS`), defaulting to `$USER` and prompting securely if password is not preset.
6. Launches the application in seamless RemoteApp mode with clipboard integration and dynamic resolution matching.

---

## Guest Setup (One-time)

To allow RemoteApp execution, configure the Windows guest:

### 1. Enable Remote Desktop
Open **Settings → System → Remote Desktop** and toggle **On**.

### 2. Allow RemoteApps
By default, Windows only permits pre-registered RemoteApps. Run the following command in an Administrator Command Prompt or PowerShell inside the guest to allow launching any unlisted executable:

```cmd
reg add "HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Terminal Server\TSAppAllowList" /v fDisabledAllowList /t REG_DWORD /d 1 /f
```

*(Alternatively, open-source tools such as [RemoteApp Tool](http://www.kimconnect.com/remoteapp-tool/) can be used to generate specific allowed application lists).*

---

## Examples

```bash
# Launch Notepad
qvm app win11 notepad.exe

# Launch Notepad with an argument
qvm app win11 notepad.exe "C:\Users\Public\notes.txt"

# Launch an installed third-party application
qvm app win11 "C:\Program Files\Notepad++\notepad++.exe"

# Launch with credentials specified in environment
QVM_RDP_USER="Admin" QVM_RDP_PASS="secret" qvm app win11 cmd.exe
```
