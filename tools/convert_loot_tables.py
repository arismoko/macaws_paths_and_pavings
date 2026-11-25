#!/usr/bin/env python3
"""Convert pre-26.3 loot tables ("condition"/"function" type keys) to the 26.3
"type" format. A typed object is a dict whose "condition"/"function" value is a
string (a type id); dict-valued "condition" fields (pool/entry slots) are kept."""
import json
import pathlib
import sys


def convert(node):
    if isinstance(node, list):
        return [convert(v) for v in node]
    if not isinstance(node, dict):
        return node
    out = {}
    for key, value in node.items():
        if key in ("condition", "function") and isinstance(value, str) and "type" not in node:
            out["type"] = value
        else:
            out[key] = convert(value)
    return out


def main(root):
    changed = 0
    for path in sorted(pathlib.Path(root).glob("data/*/loot_table/**/*.json")):
        old = json.loads(path.read_text())
        new = convert(old)
        if new != old:
            path.write_text(json.dumps(new, indent=2) + "\n")
            changed += 1
            print("converted", path)
    print(f"{changed} file(s) converted")


if __name__ == "__main__":
    main(sys.argv[1] if len(sys.argv) > 1 else ".")
