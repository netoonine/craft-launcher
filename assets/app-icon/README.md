# MANUCRAFT app icon

<!-- MANUCRAFT: this file is the fork's own (upstream's described the ArtCraft "A" mark). -->

The app icon is the **MANUCRAFT "M" mark**, an original drawing in
[`assets/manucraft/manucraft-mark.svg`](../manucraft/manucraft-mark.svg): a warm-white geometric
"M" and an amber rule on a near-black rounded square. Every file in this directory is rendered
from it by `packaging/manucraft-icons.sh` (which `packaging/icons.sh` runs). The file names are
upstream's, so packaging and the code that includes them didn't change.

License: MIT OR Apache-2.0, like the code (see `ATTRIBUTION.md`).

## Files

- `artcraft-launcher.svg`: a copy of the mark; also the hicolor scalable icon.
- `artcraft-launcher-1024.png`: 1024 px render, with transparent margin for Apple's icon grid.
- `artcraft-launcher.icns`: macOS bundle icon (`CFBundleIconFile`, copied as `MANUCRAFT.icns`).
- `artcraft-launcher.ico`: Windows icon, 16–256 px.
- `hicolor/<size>/apps/ai.storyteller.artcraft-launcher.png` (16–512) and
  `hicolor/scalable/apps/ai.storyteller.artcraft-launcher.svg`: Linux icon theme; the 256 px one
  is also the window icon.
- The tray / menu-bar icon: `assets/manucraft/manucraft-mark-black-44.png` (macOS template) and
  `assets/manucraft/manucraft-mark-64.png` (elsewhere).

## Regenerate

`packaging/icons.sh` (needs `resvg`: `cargo install resvg --locked --version 0.48.1`). Never edit
the renders by hand; change `assets/manucraft/manucraft-mark.svg` and run the script.
