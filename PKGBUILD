# Maintainer: Henrique Stelzer <your-email at domain dot tld>
pkgname=qvm-ctl
pkgver=2.0.0
pkgrel=1
pkgdesc='QEMU/KVM VM lifecycle manager without libvirt'
arch=('x86_64')
url='https://github.com/HenriqueStelzer/qvm-ctl'
license=('MIT')
depends=(
    'qemu-system-x86'
    'edk2-ovmf'
    'swtpm'
    'procps-ng'
)
makedepends=('cargo')
optdepends=(
    'virt-viewer: SPICE display connection'
    'freerdp: RemoteApp guest application execution'
)
provides=('qvm')
source=("$pkgname-$pkgver.tar.gz::$url/archive/refs/tags/v$pkgver.tar.gz")
b2sums=('SKIP')

build() {
    cd "$pkgname-$pkgver"
    cargo build --release --locked
}

check() {
    cd "$pkgname-$pkgver"
    cargo test --locked
}

package() {
    cd "$pkgname-$pkgver"
    install -Dm755 "target/release/qvm" "$pkgdir/usr/bin/qvm"
    install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
    install -Dm644 README.md "$pkgdir/usr/share/doc/$pkgname/README.md"
}
