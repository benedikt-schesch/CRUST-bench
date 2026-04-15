#!/usr/bin/env python3
"""Compute LOC and unsafe-code metrics for CRUST-bench translation outputs.

Mirrors the metrics from harvest-agentic (battery.rs count_loc / count_unsafe):
  - LOC: non-blank, non-comment lines in *.rs files (excluding bin/ and tests/)
  - Unsafe blocks, fns, impls, and lines inside them

Usage:
    python3 scripts/compute_metrics.py                    # all models
    python3 scripts/compute_metrics.py --model gpt54      # single model
    python3 scripts/compute_metrics.py --output metrics.json  # save JSON
"""
import argparse
import json
import os
import re
import sys
from pathlib import Path


def count_loc(src_dir: Path) -> int:
    """Count non-blank, non-comment lines in *.rs files, excluding bin/ and tests/."""
    total = 0
    if not src_dir.is_dir():
        return 0
    for entry in sorted(src_dir.iterdir()):
        if entry.is_dir():
            if entry.name in ("bin", "tests"):
                continue
            total += count_loc(entry)
        elif entry.suffix == ".rs":
            try:
                text = entry.read_text(errors="replace")
            except OSError:
                continue
            for line in text.splitlines():
                stripped = line.strip()
                if stripped and not stripped.startswith("//"):
                    total += 1
    return total


def count_unsafe(src_dir: Path) -> dict:
    """Count unsafe blocks, fns, impls, and lines inside them.

    Uses brace-depth tracking after detecting unsafe keywords.
    This is a simplified approximation of the syn-based Rust AST analysis.
    """
    counts = {"blocks": 0, "fns": 0, "impls": 0, "lines": 0}
    if not src_dir.is_dir():
        return counts
    for entry in sorted(src_dir.iterdir()):
        if entry.is_dir():
            if entry.name in ("bin", "tests"):
                continue
            sub = count_unsafe(entry)
            for k in counts:
                counts[k] += sub[k]
        elif entry.suffix == ".rs":
            try:
                text = entry.read_text(errors="replace")
            except OSError:
                continue
            fc = _count_unsafe_in_source(text)
            for k in counts:
                counts[k] += fc[k]
    return counts


def _count_unsafe_in_source(src: str) -> dict:
    """Parse a single Rust source file for unsafe constructs."""
    counts = {"blocks": 0, "fns": 0, "impls": 0, "lines": 0}
    lines = src.splitlines()
    i = 0
    while i < len(lines):
        line = lines[i].strip()
        # Remove string literals and comments for keyword detection
        clean = _strip_strings_and_comments(line)

        if re.search(r'\bunsafe\s+impl\b', clean):
            counts["impls"] += 1
            span = _brace_span(lines, i)
            counts["lines"] += span
            i += span
            continue
        elif re.search(r'\bunsafe\s+(extern\s+("C"\s+)?)?fn\b', clean):
            counts["fns"] += 1
            span = _brace_span(lines, i)
            counts["lines"] += span
            i += span
            continue
        elif re.search(r'\bunsafe\s*\{', clean):
            counts["blocks"] += 1
            span = _brace_span(lines, i)
            counts["lines"] += span
            i += span
            continue

        i += 1
    return counts


def _strip_strings_and_comments(line: str) -> str:
    """Remove string literals and line comments for keyword detection."""
    # Remove line comments
    result = re.sub(r'//.*$', '', line)
    # Remove string literals (simple approximation)
    result = re.sub(r'"(?:[^"\\]|\\.)*"', '""', result)
    return result


def _brace_span(lines: list, start: int) -> int:
    """Count lines from start until braces balance (inclusive)."""
    depth = 0
    found_open = False
    for i in range(start, len(lines)):
        clean = _strip_strings_and_comments(lines[i])
        for ch in clean:
            if ch == '{':
                depth += 1
                found_open = True
            elif ch == '}':
                depth -= 1
        if found_open and depth <= 0:
            return i - start + 1
    # If braces never balanced, count to end
    return len(lines) - start


def compute_project_metrics(project_dir: Path) -> dict | None:
    """Compute metrics for a single project."""
    src_dir = project_dir / "src"
    if not src_dir.is_dir():
        return None
    loc = count_loc(src_dir)
    unsafe = count_unsafe(src_dir)
    return {"loc": {"code": loc}, "unsafe": unsafe}


def main():
    parser = argparse.ArgumentParser(description="Compute LOC and unsafe metrics for CRUST-bench outputs")
    parser.add_argument("--model", help="Single model to process (default: all)")
    parser.add_argument("--output", help="Save aggregate JSON to file")
    args = parser.parse_args()

    repo_root = Path(__file__).resolve().parent.parent
    outputs_dir = repo_root / "src" / "outputs"

    if not outputs_dir.is_dir():
        print(f"Error: {outputs_dir} not found", file=sys.stderr)
        sys.exit(1)

    models = [args.model] if args.model else sorted(
        d.name for d in outputs_dir.iterdir() if d.is_dir()
    )

    all_results = {}
    for model in models:
        model_dir = outputs_dir / model
        if not model_dir.is_dir():
            print(f"Warning: {model_dir} not found, skipping", file=sys.stderr)
            continue

        projects = sorted(d.name for d in model_dir.iterdir() if d.is_dir())
        model_metrics = {}
        total_loc = 0
        total_unsafe_lines = 0

        for proj in projects:
            metrics = compute_project_metrics(model_dir / proj)
            if metrics is None:
                continue
            model_metrics[proj] = metrics
            total_loc += metrics["loc"]["code"]
            total_unsafe_lines += metrics["unsafe"]["lines"]

        unsafe_pct = (
            f"{total_unsafe_lines / total_loc * 100:.1f}%"
            if total_loc > 0 else "N/A"
        )
        summary = {
            "projects": len(model_metrics),
            "total_loc": total_loc,
            "total_unsafe_lines": total_unsafe_lines,
            "unsafe_pct": unsafe_pct,
        }
        all_results[model] = {"summary": summary, "projects": model_metrics}

        print(f"{model}: {len(model_metrics)} projects, LOC={total_loc}, "
              f"unsafe_lines={total_unsafe_lines} ({unsafe_pct})")

        # Write per-project metrics into each project's metadata/
        for proj, metrics in model_metrics.items():
            meta_dir = model_dir / proj / "metadata"
            meta_dir.mkdir(exist_ok=True)
            metrics_path = meta_dir / "code_metrics.json"
            with open(metrics_path, "w") as f:
                json.dump(metrics, f, indent=2, sort_keys=True)
                f.write("\n")

    if args.output:
        with open(args.output, "w") as f:
            json.dump(all_results, f, indent=2, sort_keys=True)
            f.write("\n")
        print(f"\nWrote {args.output}")


if __name__ == "__main__":
    main()
