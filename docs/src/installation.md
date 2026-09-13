# Installation

## 1. Install Dependencies

Ensure virtualization is supported (`ls -l /dev/kvm`). Then install dependencies for your distribution (including the Rust toolchain to compile from source):

### Arch Linux
```bash
sudo pacman -S qemu-full edk2-ovmf swtpm virt-viewer freerdp rust
```

### Debian / Ubuntu
```bash
sudo apt update
sudo apt install qemu-system-x86 qemu-utils ovmf swtpm swtpm-tools virt-viewer freerdp2-x11 cargo rustc
```

### Fedora
```bash
sudo dnf install qemu-kvm qemu-img edk2-ovmf swtpm swtpm-tools virt-viewer freerdp cargo rust
```

> **Note:** If you get permission errors accessing `/dev/kvm`, add your user to the `kvm` group:
> ```bash
> sudo usermod -aG kvm $USER
> ```

---

## 2. Build & Install qvm

Clone the repository and build the release binary with `cargo`:

```bash
git clone https://github.com/HenriqueStelzer/qvm-ctl.git
cd qvm-ctl
cargo build --release
sudo install -m755 target/release/qvm /usr/local/bin/qvm
```

Alternatively, install directly using `cargo install`:

```bash
cargo install --path .
```

Or on Arch Linux using the included PKGBUILD:
```bash
makepkg -si
```

---

## 3. Verify

```bash
qvm version
# Output: qvm 1.2.0
```
