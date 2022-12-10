# shellcheck disable=SC2034
# shellcheck disable=SC2154
# shellcheck disable=SC2164
# Maintainer: Stuart Reilly <stu@reilly-family.co.uk>
pkgname=shitpost
pkgver=0.1.0
pkgrel=1
pkgdesc="Discord shitposting bot"
arch=('x86_64')
url="https://github.com/themadprofessor/shitpost"
license=('GPL')
groups=()
depends=()
makedepends=(cargo)
optdepends=()
provides=()
conflicts=()
replaces=()
backup=()
options=()
install=
changelog=
source=('git+https://github.com/themadprofessor/shitpost.git')
noextract=()
md5sums=('SKIP')

prepare() {
  cd $pkgname
  cargo fetch --locked --target "$CARCH-unknown-linux-gnu"
}

build() {
  cd $pkgname
  cargo build --frozen --release
}

package() {
  install -Dm0755 -t "$pkgdir/usr/bin" "$pkgname/target/release/$pkgname"
  install -Dm0644 -t "$pkgdir/etc" "$pkgname/shitpost.toml"
  install -Dm0644 -t "$pkgdir/usr/lib/systemd/system" "$pkgname/shitpost.service"
}