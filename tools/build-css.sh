#!/usr/bin/env bash
# Rebuild assets/tailwind.css from tailwind.css.
#
# Tailwind's output is committed so `dx serve` / `dx bundle` need no Node toolchain. Run this
# after adding, removing or renaming utility classes in src/**/*.rs — otherwise the new classes
# silently do nothing, because assets/tailwind.css is a pre-built stylesheet.
set -euo pipefail
cd "$(dirname "$0")/.."

VERSION="${TAILWIND_VERSION:-4.1.5}"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

npm i --silent --prefix "$TMP" "tailwindcss@$VERSION" "@tailwindcss/cli@$VERSION"
# NODE_PATH lets the CLI resolve `tailwindcss` even though it is installed outside this project.
NODE_PATH="$TMP/node_modules" "$TMP/node_modules/.bin/tailwindcss" -i tailwind.css -o assets/tailwind.css --minify
echo "assets/tailwind.css: $(wc -c < assets/tailwind.css) bytes"
