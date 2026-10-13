#!/usr/bin/env bash
# Regenerates every app icon from the Inkscape sources in design/.
#   design/icon.svg          full-bleed square (iOS/Android/maskable: the OS rounds it)
#   design/icon-rounded.svg  with corner radius (favicon, desktop, in-app)
# Requires: inkscape, imagemagick (convert)
set -euo pipefail
cd "$(dirname "$0")/.."
ex() { inkscape "$1" --export-type=png --export-filename="$2" -w "$3" -h "$3" >/dev/null 2>&1; }
for s in 512 192 180; do ex design/icon.svg public/icons/icon-$s.png $s; done
ex design/icon.svg public/icons/maskable-512.png 512
ex design/icon.svg icons/icon.png 1024
for s in 32 128 256; do ex design/icon-rounded.svg icons/${s}x${s}.png $s; done
convert icons/256x256.png icons/128x128.png icons/32x32.png icons/icon.ico
ex design/icon-rounded.svg /tmp/f16.png 16; ex design/icon-rounded.svg /tmp/f48.png 48
ex design/icon-rounded.svg public/icons/favicon-32.png 32
convert /tmp/f16.png public/icons/favicon-32.png /tmp/f48.png public/favicon.ico
cp design/icon-rounded.svg public/icons/icon.svg
cp design/icon-rounded.svg assets/app-icon.svg
echo "Icons regenerated."
