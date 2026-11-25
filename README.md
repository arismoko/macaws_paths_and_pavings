# Macaw's Paths and Pavings for Sunrise

316 paths, pavings, slabs and stairs in stone, brick, wood and soil styles.

A port of [Macaw's Paths and Pavings](https://github.com/sketchmacaw/Paths-Pavings) by Sketch Macaw and Peachy Macaw to [Sunrise](https://playsunrise.net) (Minecraft Java 26.3), as a content mod with a small native part: the original's blocks, models, textures, recipes and strings, plus its tools and placement rules.

## What it does

- 130 thin path and paving blocks, 26 facing paths (pale oak and poplar included), 52 engraved pavings and 5 flattened soil paths, with the original's shapes.
- Right-click an engraved paving's block with a pickaxe to engrave it and with a shovel on a soil path block to flatten it, as in the original: the block drops 1/16 in height, a sound plays and the tool loses one durability (not in creative).
- Wooden paths and dumble pavings are placed turned a quarter clockwise from where you look.
- The engraving and flattening blocks say how to use them in their tooltip.
- 52 slabs and 52 stairs to match.
- Stonecutter and crafting recipes, drops, advancements and a creative tab.

## Differences from the original

- The wooden paths do not carry the original's `forceSolidOn` flag: the block declaration cannot set it.

## Building

With the Sunrise mod tool (`cargo sunrise-mod`, from the Sunrise SDK):

```sh
cargo sunrise-mod check
cargo sunrise-mod build    # target/sunrise-mod/mcwpaths.smod
```

This version is made for the Sunrise Dev build `dev-2026-11-25-bdad86d`.

## Licence and credits

Macaw's Paths and Pavings is by Sketch Macaw and Peachy Macaw (https://github.com/sketchmacaw/Paths-Pavings). Its textures, models, recipes and strings are Sketch Macaw and Peachy Macaw's and are used here under the **MIT licence**, the original's; `LICENSE` holds the text and its copyright notice, and `LICENSE-NOTICE.md` says what this port carries.

Carried to Minecraft 26.3 by AdLf0208 ([AdLf0208/macaws_paths_and_pavings](https://github.com/AdLf0208/macaws_paths_and_pavings)), and ported to Sunrise by arismoko. This port is distributed under the same licence.
