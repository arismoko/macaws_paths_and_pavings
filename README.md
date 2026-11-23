# Macaw's Paths and Pavings for Sunrise

315 paths, pavings, slabs and stairs in stone, brick, wood and soil styles.

A port of [Macaw's Paths and Pavings](https://github.com/sketchmacaw/Paths-Pavings) by Sketch Macaw and Peachy Macaw to [Sunrise](https://playsunrise.net) (Minecraft Java 26.3), as a content mod: the original's blocks, models, textures, recipes and strings, with no code.

## What it does

- 130 thin path and paving blocks, 24 facing paths, 52 engraved pavings and 5 flattened soil paths, with the original's shapes.
- 52 slabs and 52 stairs to match.
- Stonecutter and crafting recipes, drops, advancements and a creative tab.

## Differences from the original

- Engraving a paving and flattening soil with a tool are Java code and don't work; the engraved and flattened blocks come from the stonecutter or creative.
- No tooltips on the engraved and flattened blocks.

## Building

With the Sunrise mod tool (`cargo sunrise-mod`, from the Sunrise SDK):

```sh
cargo sunrise-mod check
cargo sunrise-mod build    # target/sunrise-mod/mcwpaths.smod
```

This version is made for the Sunrise Dev build `dev-2026-11-22-7cfdf81`.

## Licence and credits

Macaw's Paths and Pavings is by Sketch Macaw and Peachy Macaw (https://github.com/sketchmacaw/Paths-Pavings). Its textures, models, recipes and strings are Sketch Macaw and Peachy Macaw's and are used here under the **MIT licence**, the original's; `LICENSE` holds the text and its copyright notice, and `LICENSE-NOTICE.md` says what this port carries.

Carried to Minecraft 26.3 by AdLf0208 ([AdLf0208/macaws_paths_and_pavings](https://github.com/AdLf0208/macaws_paths_and_pavings)), and ported to Sunrise by arismoko. This port is distributed under the same licence.
