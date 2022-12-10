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
md5sums=() #autofill using updpkgsums

prepare() {
  cargo fetch --locked --target "$CARCH-unknown-linux-gnu"
}

build() {
  export RUSTUP_TOOLCHAIN=stable
  export CARGO_TARGET_DIR=target
  cargo build --frozen --release
}

package() {
  install -Dm0755 -t "$pkgdir/usr/bin" "target/release/$pkgname"
  install -Dm0644 -t "$pkgdir/etc" "shitpost.toml"
  install -Dm0644 -t "$pkgdir/usr/lib/systemd/system" "shitpost.service"
}