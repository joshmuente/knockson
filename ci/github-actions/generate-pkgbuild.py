#!/usr/bin/env python3

import os
import requests
import sys

try:
  tag = os.environ.get("tag_name")
  raw_version = tag.replace('v', '')
  sha265sum_url = f"https://github.com/joshmuente/knockson/releases/download/{tag}/knockson_{tag}_x86_64-unknown-linux-musl.tar.xz.sha256sum"
  sha265sum = requests.get(sha265sum_url).content.decode("utf-8")

  template = """#Maintainer: Josh Münte
#Auto generated. Do not edit.
_pkgname='knockson'
pkgname="${_pkgname}-bin"
pkgver="""+ raw_version +"""
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
sha256sums=('"""+ sha265sum +"""')

package () {
  install -Dm 775 "${_pkgname}" "${pkgdir}/usr/bin/${_pkgname}"
}"""

  with open("PKGBUILD", "w") as text_file:
      text_file.write(template)

  print("PKGBUILD generated")
  sys.exit(os.EX_OK)
except Exception as e:
  print("PKGBUILD Error:")
  print(e)
  sys.exit(os.EX_USAGE)