#!/usr/bin/env bash
# MANUCRAFT: regenerate the launcher's app and tray icons from the MANUCRAFT mark
# (assets/manucraft/manucraft-mark.svg, an original drawing; see ATTRIBUTION.md). Replaces the
# upstream recipe, which drew them from the ArtCraft trademark (not licensed for forks).
#
# - App icon (Dock, taskbar, Linux menus, installers): the mark. macOS gets Apple's 824/1024 grid
#   (transparent margin); everything else uses the mark full size.
# - Tray: the black "M" as the macOS menu-bar template (44 px; macOS tints it); the mark elsewhere.
#
# Needs resvg (cargo install resvg --locked --version 0.48.1). The .ico and .icns are packed by
# `cargo xtask ico` / `cargo xtask icns`. Output file names stay upstream's (artcraft-launcher*,
# ai.storyteller.artcraft-launcher.png) so packaging and include_bytes! paths don't change.
#
#   packaging/manucraft-icons.sh      (packaging/icons.sh runs this)
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DIR="$ROOT/assets/app-icon"
OURS="$ROOT/assets/manucraft"
SVG="$OURS/manucraft-mark.svg"
ID="ai.storyteller.artcraft-launcher"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

command -v resvg >/dev/null || { echo "error: resvg not found (cargo install resvg --locked --version 0.48.1)" >&2; exit 1; }

BOX='viewBox="0 0 824 824"'
grep -q "$BOX" "$SVG" || { echo "error: expected $BOX in $SVG" >&2; exit 1; }
MAC="$TMP/mac.svg"
sed "s/$BOX/viewBox=\"-100 -100 1024 1024\"/" "$SVG" >"$MAC"
FLAT="$SVG"

render() { resvg -w "$2" -h "$2" "$1" "$3" </dev/null; }

render "$MAC" 1024 "$DIR/artcraft-launcher-1024.png"

for s in 16 24 32 48 64 128 256 512; do
  mkdir -p "$DIR/hicolor/${s}x${s}/apps"
  render "$FLAT" "$s" "$DIR/hicolor/${s}x${s}/apps/$ID.png"
done
mkdir -p "$DIR/hicolor/scalable/apps"
cp "$SVG" "$DIR/hicolor/scalable/apps/$ID.svg"
cp "$SVG" "$DIR/artcraft-launcher.svg"

ICO_PNGS=()
for s in 16 20 24 32 40 48 64 128 256; do
  render "$FLAT" "$s" "$TMP/ico-$s.png"
  ICO_PNGS+=("$TMP/ico-$s.png")
done
(cd "$ROOT" && cargo run -q -p xtask -- ico "$DIR/artcraft-launcher.ico" "${ICO_PNGS[@]}")

ICNS_PNGS=()
for s in 16 32 64 128 256 512 1024; do
  render "$MAC" "$s" "$TMP/icns-$s.png"
  ICNS_PNGS+=("$TMP/icns-$s.png")
done
(cd "$ROOT" && cargo run -q -p xtask -- icns "$DIR/artcraft-launcher.icns" "${ICNS_PNGS[@]}")

render "$OURS/manucraft-mark-black.svg" 44 "$OURS/manucraft-mark-black-44.png"
render "$SVG" 64 "$OURS/manucraft-mark-64.png"
# The wordmark as a PNG for the Windows installer art (packaging/windows/package.ps1).
resvg -w 1600 "$OURS/manucraft-wordmark-white.svg" "$OURS/manucraft-wordmark-white.png" </dev/null
echo "icons written to $DIR and $OURS"
