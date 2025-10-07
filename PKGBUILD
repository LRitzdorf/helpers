# Maintainer: LRitzdorf <42657792+LRitzdorf@users.noreply.github.com>
pkgname=helpers-git
pkgver=r32.e088717
pkgrel=1
pkgdesc="Random helper scripts, mostly for personal use"
arch=('x86_64') # Only becuase we compile a Rust binary. If you test this on another architecure, let me know!
url="https://github.com/LRitzdorf/helpers"
license=('MIT')
depends=('sh' 'coreutils')
makedepends=('git' 'rust')
optdepends=('tmux: for use with the "stmux" helper')
source=("git+https://github.com/LRitzdorf/helpers.git")
sha256sums=('SKIP')

pkgver() {
    cd "${pkgname%-git}"
    printf "r%s.%s" "$(git rev-list --count HEAD)" "$(git rev-parse --short=7 HEAD)"
}

build() {
    cd "${pkgname%-git}"
    make
}

package() {
    cd "${pkgname%-git}"
    install -D -t "${pkgdir}/usr/bin" notify nvim-hosted stmux
    # `conserve` must be setuid root to access battery controls
    install -D -t "${pkgdir}/usr/bin" conserve -o root -m 4755
    # And the license
    install -D -t "${pkgdir}/usr/share/licenses/${pkgname}" LICENSE
}
