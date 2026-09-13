# Maintainer: Henrique Stelzer <your-email@example.com>
pkgname=qvm-ctl
pkgver=1.1.0
pkgrel=1
pkgdesc='QEMU/KVM VM lifecycle manager — create, launch, stop VMs without libvirt'
arch=('any')
url='https://github.com/HenriqueStelzer/qvm-ctl'
license=('MIT')
depends=('qemu-system-x86' 'edk2-ovmf' 'bash' 'swtpm' 'iproute2')
optdepends=('virt-viewer: SPICE display connection')
source=("$pkgname-$pkgver.tar.gz::https://github.com/HenriqueStelzer/qvm-ctl/archive/refs/tags/v$pkgver.tar.gz")
sha256sums=('SKIP')

package() {
    install -Dm755 "$srcdir/$pkgname-$pkgver/qvm-ctl.sh" "$pkgdir/usr/bin/qvm"
    install -Dm644 "$srcdir/$pkgname-$pkgver/LICENSE" "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
    install -Dm644 "$srcdir/$pkgname-$pkgver/README.md" "$pkgdir/usr/share/doc/$pkgname/README.md"
}
