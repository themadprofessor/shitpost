# shellcheck disable=SC2034
# shellcheck disable=SC2154
# shellcheck disable=SC2164
# Maintainer: Stuart Reilly <stu@reilly-family.co.uk>
_pkgname=shitpost
pkgname=$_pkgname-git
pkgver=r15.2a1bd5d
pkgrel=1
pkgdesc="Discord shitposting bot"
arch=('x86_64')
url="https://github.com/themadprofessor/shitpost"
license=('MIT')
depends=('libssl.so' 'libcrypto.so')
makedepends=(cargo)
backup=('etc/shitpost.toml')
source=('git+https://github.com/themadprofessor/shitpost.git')
sha512sums=('SKIP')

prepare() {
  cd $_pkgname
  cargo fetch --locked --target "$CARCH-unknown-linux-gnu"
}

build() {
  cd $_pkgname
  cargo build --frozen --release
}

package() {
  install -Dm0755 -t "$pkgdir/usr/bin" "$_pkgname/target/release/$_pkgname"
  install -Dm0644 -t "$pkgdir/etc" "$_pkgname/shitpost.toml"
  install -Dm0644 -t "$pkgdir/usr/lib/systemd/system" "$_pkgname/shitpost.service"
}

pkgver() {
  cd $_pkgname
  printf "r%s.%s" "$(git rev-list --count HEAD)" "$(git rev-parse --short HEAD)"
}
