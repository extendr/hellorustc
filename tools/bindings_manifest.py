#!/usr/bin/env python3
"""
Generate manifest summaries of cached bindings across OS and R versions.

Outputs:
- cached_bindings/manifest_variants.json: per header, unique canonical variants
  (by body hash) and the (platform, R version) combos that map to each.
- cached_bindings/manifest_axes.json: per header, per-platform uniqueness across
  R versions, and per-R-version uniqueness across platforms. Useful to see where
  cfg(target_os) vs cfg(r_ver_*) gates are needed.

This reads files from git HEAD, so it accounts for tracked cached bindings
regardless of the working tree state.
"""

from __future__ import annotations

import hashlib
import json
import re
import subprocess
from dataclasses import dataclass
from pathlib import Path
from typing import Dict, List

ROOT = Path(__file__).resolve().parent.parent
CACHE_PREFIX = "cached_bindings/"
VARIANTS_OUT = ROOT / "cached_bindings" / "manifest_variants.json"
AXES_OUT = ROOT / "cached_bindings" / "manifest_axes.json"

PATH_PATTERN = re.compile(
    r"^(?P<stem>.+)-(?P<platform>Darwin|Linux|windows)-(?P<version>\d\.\d)\.rs$"
)


@dataclass
class Binding:
    base: str
    platform: str
    r_version: str
    rel_path: str
    canonical_hash: str


def canonical_body(text: str) -> str:
    """Strip bindgen header lines and normalize line endings."""
    lines = text.splitlines()
    idx = 0
    while idx < len(lines) and (
        lines[idx].strip() == "" or lines[idx].lstrip().startswith("/*")
    ):
        idx += 1
    return "\n".join(ln.rstrip("\r") for ln in lines[idx:])


def read_bindings_from_head() -> List[Binding]:
    out = subprocess.check_output(
        ["git", "ls-tree", "-r", "--name-only", "HEAD", CACHE_PREFIX], cwd=ROOT
    ).decode()
    bindings: List[Binding] = []
    for rel in out.splitlines():
        if not rel.endswith(".rs"):
            continue
        rel_short = rel[len(CACHE_PREFIX) :]
        m = PATH_PATTERN.match(rel_short)
        if not m:
            continue
        content = subprocess.check_output(
            ["git", "show", f"HEAD:{rel}"], cwd=ROOT
        ).decode()
        canon = canonical_body(content)
        h = hashlib.sha256(canon.encode()).hexdigest()
        bindings.append(
            Binding(
                base=m.group("stem"),
                platform=m.group("platform"),
                r_version=m.group("version"),
                rel_path=rel_short,
                canonical_hash=h,
            )
        )
    return bindings


def build_variants_manifest(bindings: List[Binding]) -> Dict[str, List[dict]]:
    manifest: Dict[str, Dict[str, dict]] = {}
    for b in bindings:
        base_entry = manifest.setdefault(b.base, {})
        entry = base_entry.setdefault(
            b.canonical_hash, {"canonical": b.rel_path, "combos": []}
        )
        if b.rel_path < entry["canonical"]:
            entry["canonical"] = b.rel_path
        entry["combos"].append({"platform": b.platform, "r_version": b.r_version})

    return {
        base: [
            {
                "canonical": data["canonical"],
                "hash": h,
                "combos": sorted(data["combos"], key=lambda c: (c["platform"], c["r_version"])),
            }
            for h, data in sorted(variants.items())
        ]
        for base, variants in sorted(manifest.items())
    }


def build_axes_manifest(bindings: List[Binding]) -> Dict[str, dict]:
    by_platform: Dict[str, Dict[str, Dict[str, dict]]] = {}
    by_r_version: Dict[str, Dict[str, Dict[str, dict]]] = {}

    for b in bindings:
        plat_entry = by_platform.setdefault(b.base, {}).setdefault(b.platform, {})
        plat_data = plat_entry.setdefault(
            b.canonical_hash, {"canonical": b.rel_path, "r_versions": []}
        )
        if b.rel_path < plat_data["canonical"]:
            plat_data["canonical"] = b.rel_path
        if b.r_version not in plat_data["r_versions"]:
            plat_data["r_versions"].append(b.r_version)

        ver_entry = by_r_version.setdefault(b.base, {}).setdefault(b.r_version, {})
        ver_data = ver_entry.setdefault(
            b.canonical_hash, {"canonical": b.rel_path, "platforms": []}
        )
        if b.rel_path < ver_data["canonical"]:
            ver_data["canonical"] = b.rel_path
        if b.platform not in ver_data["platforms"]:
            ver_data["platforms"].append(b.platform)

    def normalize(entry: Dict[str, Dict[str, dict]], sort_inner: str, sort_list: str):
        return {
            base: {
                key: [
                    {
                        "canonical": data["canonical"],
                        sort_inner: h,
                        sort_list: sorted(data[sort_list]),
                    }
                    for h, data in sorted(group.items())
                ]
                for key, group in sorted(per_base.items())
            }
            for base, per_base in sorted(entry.items())
        }

    return {
        "by_platform": normalize(by_platform, "hash", "r_versions"),
        "by_r_version": normalize(by_r_version, "hash", "platforms"),
    }


def main() -> None:
    bindings = read_bindings_from_head()
    VARIANTS_OUT.write_text(json.dumps(build_variants_manifest(bindings), indent=2))
    AXES_OUT.write_text(json.dumps(build_axes_manifest(bindings), indent=2))
    print(
        f"Wrote manifests: {VARIANTS_OUT.relative_to(ROOT)}, "
        f"{AXES_OUT.relative_to(ROOT)}"
    )


if __name__ == "__main__":
    main()
