# Maintainer: Henrique Stelzer <your-email at domain dot tld>
pkgname=qvm-ctl
pkgver=1.1.0
pkgrel=1
pkgdesc='QEMU/KVM VM lifecycle manager without libvirt'
arch=('any')
url='https://github.com/HenriqueStelzer/qvm-ctl'
license=('MIT')
depends=(
    'qemu-system-x86'
    'edk2-ovmf'
    'swtpm'
    'iproute2'
    'procps-ng'
)
optdepends=('virt-viewer: SPICE display connection')
provides=('qvm')
source=("$pkgname-$pkgver.tar.gz::$url/archive/refs/tags/v$pkgver.tar.gz")
b2sums=('SKIP')

check() {
    cd "$pkgname-$pkgver"
    bash test/smoke.sh
}

package() {
    cd "$pkgname-$pkgver"
    install -Dm755 src/qvm-ctl.sh "$pkgdir/usr/bin/qvm"
    install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
    install -Dm644 README.md "$pkgdir/usr/share/doc/$pkgname/README.md"
}
