# Installation

## 1. Install Dependencies

Ensure virtualization is supported (`ls -l /dev/kvm`). Then install dependencies for your distribution:

### Arch Linux
```bash
sudo pacman -S qemu-full edk2-ovmf swtpm virt-viewer
```

### Debian / Ubuntu
```bash
sudo apt update
sudo apt install qemu-system-x86 qemu-utils ovmf swtpm swtpm-tools virt-viewer
```

### Fedora
```bash
sudo dnf install qemu-kvm qemu-img edk2-ovmf swtpm swtpm-tools virt-viewer
```

> **Note:** If you get permission errors accessing `/dev/kvm`, add your user to the `kvm` group:
> ```bash
> sudo usermod -aG kvm $USER
> ```

---

## 2. Install qvm

```bash
git clone https://github.com/HenriqueStelzer/qvm-ctl.git
cd qvm-ctl
sudo install -m755 src/qvm-ctl.sh /usr/local/bin/qvm
```

Or on Arch Linux using the included PKGBUILD:
```bash
makepkg -si
```

---

## 3. Verify

```bash
qvm version
# Output: qvm 1.1.0
```
