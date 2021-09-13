#!/usr/bin/env python3

import os

template = """
#Maintainer: Josh Münte

_pkgname='knockson'
pkgname="${_pkgname}-bin"
pkgver=0.5.0
pkgrel=1
pkgdesc='simple multi-threaded port scanner written in rust'
arch=('x86_64')
url='https://github.com/joshmuente/knockson'
_url_source='https://github.com/joshmuente/knockson'
license=('MIT')
depends=()
provides=("${_pkgname}")
conflicts=("${_pkgname}")
source=("${_url_source}/releases/download/v${pkgver}/${_pkgname}_v${pkgver}_${arch}-unknown-linux-musl.tar.xz")
sha256sums=('165f65f6943fa5280fcecde2390f1d8f8bea6233458ea2548831b49548ddb0d7')

package () {
  install -Dm 775 "${_pkgname}" "${pkgdir}/usr/bin/${_pkgname}"
}
"""

print(template)
print(os.environ)
