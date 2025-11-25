#!/usr/bin/env python3
"""
Condense cached_bindings by removing duplicates and generating cfg-gated wrappers.

The cached_bindings/ directory contains one binding file per R version and
platform. Many of these files are byte-for-byte identical apart from the
generated header metadata. This script:
1. Canonicalizes files (dropping the metadata header and normalizing line endings)
   to find identical variants.
2. Deletes duplicate on-disk files, keeping a single canonical copy per unique
   variant.
3. Writes cached_bindings/reduced/<stem>.rs wrappers that select the correct
   canonical file using cfg conditions on target_os and an r_ver_<major_minor>
   cfg set via Makevars.
4. Emits cached_bindings/manifest.json summarizing which bindings map to which
   canonical file.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import re
import subprocess
from dataclasses import dataclass
from pathlib import Path
from typing import Dict, Iterable, List
import os

ROOT = Path(__file__).resolve().parent.parent
CACHE_DIR = ROOT / "cached_bindings"
REDUCED_DIR = CACHE_DIR / "reduced"
MANIFEST_PATH = CACHE_DIR / "manifest.json"

PATH_PATTERN = re.compile(
    r"^(?P<stem>.+)-(?P<platform>Darwin|Linux|windows)-(?P<version>\d\.\d)\.rs$"
)


@dataclass
class BindingFile:
    base: str
    platform: str
    version: str
    path: Path
    content: str
    canonical_hash: str

    @property
    def rel(self) -> str:
        return self.path.relative_to(CACHE_DIR).as_posix()


def canonical_body_from_text(text: str) -> str:
    """
    Strip the bindgen header (platform/r-version metadata) and normalize CRLF
    endings so identical bodies hash identically across OS/version builds.
    """
    lines = text.splitlines()
    idx = 0
    while idx < len(lines) and (
        lines[idx].strip() == "" or lines[idx].lstrip().startswith("/*")
    ):
        idx += 1
    normalized = [ln.rstrip("\r") for ln in lines[idx:]]
    return "\n".join(normalized)


def binding_from_text(rel: str, content: str) -> BindingFile | None:
    match = PATH_PATTERN.match(rel)
    if not match:
        return None

    base = match.group("stem")
    platform = match.group("platform")
    version = match.group("version")
    canonical_hash = hashlib.sha256(
        canonical_body_from_text(content).encode()
    ).hexdigest()

    return BindingFile(
        base=base,
        platform=platform,
        version=version,
        path=CACHE_DIR / rel,
        content=content,
        canonical_hash=canonical_hash,
    )


def read_head_bindings() -> List[BindingFile]:
    """Load binding files from git HEAD (covers deleted duplicates)."""
    result: List[BindingFile] = []
    ls = subprocess.check_output(
        ["git", "ls-tree", "-r", "--name-only", "HEAD", "cached_bindings"],
        cwd=ROOT,
    ).decode()
    for rel in ls.splitlines():
        if not rel.endswith(".rs"):
            continue
        content = subprocess.check_output(
            ["git", "show", f"HEAD:{rel}"], cwd=ROOT
        ).decode()
        binding = binding_from_text(rel[len("cached_bindings/") :], content)
        if binding:
            result.append(binding)
    return result


def group_bindings(bindings: Iterable[BindingFile]) -> Dict[str, Dict[str, List[BindingFile]]]:
    grouped: Dict[str, Dict[str, List[BindingFile]]] = {}
    for binding in bindings:
        grouped.setdefault(binding.base, {}).setdefault(
            binding.canonical_hash, []
        ).append(binding)
    return grouped


def pick_canonical(files: List[BindingFile]) -> BindingFile:
    """Prefer an existing file in the working tree, otherwise lexicographic order."""
    existing = [f for f in files if f.path.exists()]
    if existing:
        return sorted(existing, key=lambda f: f.rel)[0]
    return sorted(files, key=lambda f: f.rel)[0]


def ensure_file(binding: BindingFile) -> None:
    """Write the binding content if it is missing locally."""
    if binding.path.exists():
        return
    binding.path.parent.mkdir(parents=True, exist_ok=True)
    binding.path.write_text(binding.content)


def delete_duplicates(grouped: Dict[str, Dict[str, List[BindingFile]]]) -> int:
    """Remove duplicate on-disk files, keep the preferred canonical copy per variant."""
    removed = 0
    for variants in grouped.values():
        for files in variants.values():
            files.sort(key=lambda f: f.rel)
            canonical = pick_canonical(files)
            ensure_file(canonical)
            for dup in files:
                if dup is canonical:
                    continue
                if dup.path.exists():
                    dup.path.unlink()
                    removed += 1
    return removed


def cfg_expr(platform: str, version: str) -> str:
    target = {"Darwin": "macos", "Linux": "linux", "windows": "windows"}[platform]
    version_cfg = f"r_ver_{version.replace('.', '_')}"
    return f'all(target_os = "{target}", {version_cfg})'


def rel_include_path(from_path: Path, to_path: Path) -> str:
    """Return a POSIX relative path from the dir containing from_path to to_path."""
    return Path(
        os.path.relpath(to_path, start=from_path.parent)
    ).as_posix()


def write_wrappers(grouped: Dict[str, Dict[str, List[BindingFile]]], out_dir: Path) -> None:
    out_dir.mkdir(parents=True, exist_ok=True)
    for base, variants in grouped.items():
        wrapper_path = out_dir / f"{base}.rs"
        wrapper_path.parent.mkdir(parents=True, exist_ok=True)

        lines: List[str] = []
        lines.append("// Auto-generated by tools/reduce_cached_bindings.py. Do not edit.")
        lines.append("#![allow(non_upper_case_globals)]")
        lines.append("#![allow(non_camel_case_types)]")
        lines.append("#![allow(non_snake_case)]")
        lines.append("#![allow(unused)]")
        lines.append("")

        all_conditions: List[str] = []
        for variant_files in variants.values():
            variant_files.sort(key=lambda f: f.rel)
            canonical = pick_canonical(variant_files)
            ensure_file(canonical)
            include_target = rel_include_path(wrapper_path, canonical.path)
            combos = [cfg_expr(f.platform, f.version) for f in variant_files]
            condition = "any(\n" + "\n".join(f"    {expr}," for expr in combos) + "\n)"
            all_conditions.extend([f"    {expr}," for expr in combos])
            lines.append(f"#[cfg({condition})]")
            lines.append(f'include!("{include_target}");')
            lines.append("")

        if all_conditions:
            lines.append("#[cfg(not(any(")
            lines.extend(all_conditions)
            lines.append(")))]")
            lines.append(
                f'compile_error!("No cached binding match for {base} on this platform / R version");'
            )

        wrapper_path.write_text("\n".join(lines) + "\n")


def write_manifest(grouped: Dict[str, Dict[str, List[BindingFile]]]) -> None:
    manifest = {}
    for base, variants in grouped.items():
        entries = []
        for files in variants.values():
            files.sort(key=lambda f: f.rel)
            canonical = pick_canonical(files)
            ensure_file(canonical)
            combos = [
                {
                    "platform": f.platform,
                    "r_version": f.version,
                    "path": canonical.rel,
                }
                for f in files
            ]
            entries.append({"canonical": canonical.rel, "combos": combos})
        manifest[base] = entries
    MANIFEST_PATH.write_text(json.dumps(manifest, indent=2))


def main() -> None:
    parser = argparse.ArgumentParser(
        description="Condense cached bindings and emit cfg-gated wrappers."
    )
    parser.add_argument(
        "--prune-duplicates",
        action="store_true",
        help="Delete duplicate cached binding files. By default, duplicates are kept.",
    )
    args = parser.parse_args()

    grouped = group_bindings(read_head_bindings())
    removed = delete_duplicates(grouped) if args.prune_duplicates else 0
    write_wrappers(grouped, REDUCED_DIR)
    write_wrappers(grouped, ROOT / "src" / "bindings")
    write_wrappers(grouped, ROOT / "src" / "custom_bindings")
    write_manifest(grouped)

    msg = [
        f"Reduced cached_bindings: removed {removed} duplicate files"
        if args.prune_duplicates
        else "Reduced cached_bindings: duplicates kept",
        f"wrote wrappers to {REDUCED_DIR.relative_to(ROOT)}, src/bindings, and src/custom_bindings.",
    ]
    print(", ".join(msg))


if __name__ == "__main__":
    main()
