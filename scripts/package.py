"""Builds the plugin and packages it as dist/<id>-<version>.rgplugin (works on Python 3.8+).

    rustup target add wasm32-unknown-unknown
    python scripts/package.py
"""
import json
import subprocess
import zipfile
from pathlib import Path

root = Path(__file__).resolve().parent.parent
manifest = json.loads((root / "plugin.json").read_text(encoding="utf-8"))
subprocess.run(["cargo", "build", "--release", "--target", "wasm32-unknown-unknown"], cwd=root, check=True)
wasm = root / "target/wasm32-unknown-unknown/release/istat_boundaries.wasm"
out = root / "dist" / f"{manifest['id']}-{manifest['version']}.rgplugin"
out.parent.mkdir(exist_ok=True)
with zipfile.ZipFile(out, "w", zipfile.ZIP_DEFLATED) as z:
    z.write(root / "plugin.json", "plugin.json")
    z.write(wasm, manifest["entry"])
    z.write(root / "README.md", "README.md")
print(out)
