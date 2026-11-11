#!/usr/bin/env bash
# Build the Dew website: the landing page (site/) at the root, the real web app under /app/
# (also embedded live on the landing page), and version.json shared by both.
#
#   tools/build-site.sh            # local preview, served from /        -> target/site
#   tools/build-site.sh dew        # GitHub Pages project site at /dew/  (CI passes the repo name)
set -euo pipefail
cd "$(dirname "$0")/.."
BASE="${1:-}"
APP_BASE="${BASE:+$BASE/}app"
TARGET="${CARGO_TARGET_DIR:-target}"
OUT="$TARGET/site"

# 1. The app, built for /<base>/app/
cp Dioxus.toml /tmp/Dioxus.toml.bak
trap 'cp /tmp/Dioxus.toml.bak Dioxus.toml' EXIT
# Portable in-place edit: macOS sed has no -i without a suffix.
awk -v base="$APP_BASE" '/^\[web.app\]/{print; print "base_path = \"" base "\""; next} {print}' \
  Dioxus.toml > /tmp/Dioxus.toml.site && mv /tmp/Dioxus.toml.site Dioxus.toml
dx bundle --platform web --release
APP="$TARGET/dx/dew/release/web/public"

# 2. Assemble
rm -rf "$OUT" && mkdir -p "$OUT/screens"
cp -r site/. "$OUT/"
cp -r "$APP" "$OUT/app"
# site/ is self-contained (icons, favicon, screens, version.json) so it also reads
# correctly when index.html is opened straight off disk; the release script only
# needs to refresh version.json from the build above.
cp public/version.json "$OUT/version.json"
# GitHub Pages serves 404.html for unknown paths: hand deep links (/app/bible/JHN/3) to the app.
cp "$OUT/app/index.html" "$OUT/404.html"
touch "$OUT/.nojekyll"
echo "Site ready: $OUT"
