#! /bin/bash

for res in 512x512 256x256 128x128 80x80 72x72 64x64 48x48 44x44 36x36 32x32 24x24 22x22 20x20 16x16 8x8; do
		install -dm755 "${pkgdir}/usr/share/icons/hicolor/${res}/apps"
		cp -v "/home/clyds/Projets/Rust/tri_photo/icons/${res}/tp.png" "${pkgdir}/usr/share/icons/hicolor/${res}/apps/tp.png"
done