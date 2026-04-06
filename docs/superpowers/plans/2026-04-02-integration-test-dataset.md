# Integration Test Dataset (CBench_int / RBench_int) Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Create paired CBench_int and RBench_int datasets that enable end-to-end behavioral equivalence testing between C originals and LLM-generated Rust transpilations via I/O comparison of compiled binaries.

**Architecture:** Python scripts handle C compilation, output capture, Rust project scaffolding, and validation. Claude Code handles Rust `main()` generation by reading C tests + Rust interfaces and writing equivalent Rust binaries. An orchestrator script ties the pipeline together.

**Tech Stack:** Python 3.10+, CMake, Make, GCC, Cargo/Rust, tqdm, subprocess, shutil, json, toml

---

## File Structure

```
datasets/create_integration_tests/
├── create_integration_tests.py   # Main orchestrator - runs the full pipeline
├── capture_expected_output.py    # Step 1: copy C projects, compile, capture output
├── build_rbench_scaffold.py      # Step 2a: create RBench_int directory structure
├── generate_rust_mains.py        # Step 2b: manifest generator for Claude Code
├── validate.py                   # Step 3: integration test harness
├── utils.py                      # Shared utilities (name mapping, skip detection)
├── skipped.json                  # Generated: log of skipped tests
└── generation_manifest.json      # Generated: tasks for Claude Code main() generation
```

```
datasets/
├── CBench_int/<project>/         # Generated: C projects with expected output
│   ├── src/
│   ├── tests/
│   ├── CMakeLists.txt or Makefile
│   └── expected/
│       ├── <testname>.stdout
│       ├── <testname>.stderr
│       └── <testname>.exitcode
└── RBench_int/<project>/         # Generated: Rust scaffolds
    ├── Cargo.toml
    ├── src/
    │   ├── lib.rs
    │   ├── interfaces/
    │   └── bin/
    │       └── <testname>.rs     # Auto-generated main()
    └── expected/
```

---

### Task 1: Shared Utilities (`utils.py`)

**Files:**
- Create: `datasets/create_integration_tests/utils.py`

- [ ] **Step 1: Write `utils.py` with name mapping, skip detection, and build helpers**

```python
import os
import re
import subprocess
import shutil
from pathlib import Path
from typing import Optional


def c_name_to_rust_name(c_project_name: str) -> str:
    """Convert C project name to Rust-compatible name.
    Mirrors logic from src/benchmark.py."""
    name = c_project_name.replace("-", "_")
    if name[0].isdigit():
        name = "proj_" + name
    if "." in name:
        name = name.split(".")[0]
    return name


def find_c_test_files(project_dir: Path) -> list[Path]:
    """Find all .c test files in a C project.
    Searches tests/, test/, and project root for files with main()."""
    test_dirs = [
        project_dir / "tests",
        project_dir / "test",
    ]
    test_files = []
    seen = set()
    for test_dir in test_dirs:
        if test_dir.exists():
            for f in sorted(test_dir.rglob("*.c")):
                content = f.read_text(encoding="utf-8", errors="replace")
                if "int main(" in content or "void main(" in content:
                    test_files.append(f)
                    seen.add(f.name)
    # Also check project root for test files (e.g., test.c, *_test.c)
    for f in sorted(project_dir.glob("*.c")):
        if f.name in seen:
            continue
        if "test" in f.name.lower():
            content = f.read_text(encoding="utf-8", errors="replace")
            if "int main(" in content or "void main(" in content:
                test_files.append(f)
    return test_files


def test_name_from_file(test_file: Path) -> str:
    """Extract test name from a test .c file path."""
    return test_file.stem


STDIN_PATTERNS = [
    r'\bscanf\s*\(',
    r'\bfgets\s*\(.+,\s*stdin\s*\)',
    r'\bgetline\s*\(',
    r'\bread\s*\(\s*0\s*,',
    r'\bread\s*\(\s*STDIN_FILENO\s*,',
]

NONDETERMINISTIC_PATTERNS = [
    r'srand\s*\(\s*time\s*\(',
    r'srand\s*\(\s*clock\s*\(',
]

RAND_PATTERN = r'\brand\s*\(\s*\)'
FIXED_SRAND_PATTERN = r'srand\s*\(\s*\d+\s*\)'


def detect_skip_reason(test_file: Path) -> Optional[str]:
    """Check if a test file should be skipped. Returns reason or None."""
    try:
        content = test_file.read_text(encoding="utf-8", errors="replace")
    except Exception as e:
        return f"unreadable: {e}"

    for pattern in STDIN_PATTERNS:
        if re.search(pattern, content):
            return f"stdin dependency: matches {pattern}"

    for pattern in NONDETERMINISTIC_PATTERNS:
        if re.search(pattern, content):
            return f"non-deterministic: matches {pattern}"

    # rand() without fixed srand
    if re.search(RAND_PATTERN, content) and not re.search(FIXED_SRAND_PATTERN, content):
        return "non-deterministic: rand() without fixed srand()"

    return None


def build_c_project(project_dir: Path, timeout: int = 120) -> bool:
    """Build a C project using its native build system. Returns True on success."""
    cmake_file = project_dir / "CMakeLists.txt"
    makefile = project_dir / "Makefile"
    makefile_lower = project_dir / "makefile"

    if cmake_file.exists():
        build_dir = project_dir / "build"
        build_dir.mkdir(exist_ok=True)
        try:
            subprocess.run(
                ["cmake", ".."],
                cwd=build_dir, capture_output=True, text=True, timeout=timeout
            )
            result = subprocess.run(
                ["cmake", "--build", "."],
                cwd=build_dir, capture_output=True, text=True, timeout=timeout
            )
            return result.returncode == 0
        except (subprocess.TimeoutExpired, Exception):
            return False
    elif makefile.exists() or makefile_lower.exists():
        try:
            result = subprocess.run(
                ["make", "-j4"],
                cwd=project_dir, capture_output=True, text=True, timeout=timeout
            )
            return result.returncode == 0
        except (subprocess.TimeoutExpired, Exception):
            return False
    return False


def find_test_binary(project_dir: Path, test_name: str) -> Optional[Path]:
    """Find the compiled binary for a given test name."""
    # Search common locations
    candidates = [
        project_dir / "build" / test_name,
        project_dir / "tests" / test_name,
        project_dir / "test" / test_name,
        project_dir / test_name,
    ]
    for candidate in candidates:
        if candidate.exists() and os.access(candidate, os.X_OK):
            return candidate

    # Fallback: search for any executable matching the test name
    for root, dirs, files in os.walk(project_dir):
        for f in files:
            fpath = Path(root) / f
            if f == test_name and os.access(fpath, os.X_OK):
                return fpath

    return None


def run_binary_and_capture(
    binary_path: Path, cwd: Path, timeout: int = 30
) -> Optional[dict]:
    """Run a binary and capture stdout, stderr, exit code.
    Returns dict with 'stdout', 'stderr', 'exitcode' or None on timeout."""
    try:
        result = subprocess.run(
            [str(binary_path)],
            cwd=cwd,
            capture_output=True,
            text=True,
            timeout=timeout,
        )
        return {
            "stdout": result.stdout,
            "stderr": result.stderr,
            "exitcode": result.returncode,
        }
    except subprocess.TimeoutExpired:
        return None
    except Exception:
        return None
```

- [ ] **Step 2: Verify the file was written correctly**

Run: `python -c "from utils import c_name_to_rust_name, detect_skip_reason; print(c_name_to_rust_name('btree-map')); print(c_name_to_rust_name('2DPartInt'))"`

Expected output:
```
btree_map
proj_2DPartInt
```

- [ ] **Step 3: Commit**

```bash
git add datasets/create_integration_tests/utils.py
git commit -m "feat: add shared utilities for integration test dataset"
```

---

### Task 2: C Build and Output Capture (`capture_expected_output.py`)

**Files:**
- Create: `datasets/create_integration_tests/capture_expected_output.py`

- [ ] **Step 1: Write `capture_expected_output.py`**

```python
import json
import shutil
import argparse
from pathlib import Path
from tqdm import tqdm
from utils import (
    c_name_to_rust_name,
    find_c_test_files,
    test_name_from_file,
    detect_skip_reason,
    build_c_project,
    find_test_binary,
    run_binary_and_capture,
)


def copy_c_project(src: Path, dst: Path):
    """Copy a C project to CBench_int, excluding build artifacts."""
    if dst.exists():
        shutil.rmtree(dst)
    shutil.copytree(
        src,
        dst,
        ignore=shutil.ignore_patterns(
            "build", "*.o", "*.gcda", "*.gcno", "*.so", "*.a", "__pycache__"
        ),
    )


def process_project(
    c_project_dir: Path, cbench_int_dir: Path, skipped: list
) -> dict:
    """Process a single C project: copy, build, capture output.
    Returns a report dict."""
    project_name = c_project_dir.name
    report = {
        "project": project_name,
        "tests_captured": 0,
        "tests_skipped": 0,
        "build_success": False,
    }

    # Copy to CBench_int
    dst = cbench_int_dir / project_name
    copy_c_project(c_project_dir, dst)

    # Find test files
    test_files = find_c_test_files(dst)
    if not test_files:
        skipped.append({
            "project": project_name,
            "test": "*",
            "reason": "no test files with main() found",
        })
        shutil.rmtree(dst)
        return report

    # Build
    if not build_c_project(dst):
        skipped.append({
            "project": project_name,
            "test": "*",
            "reason": "build failed",
        })
        shutil.rmtree(dst)
        return report
    report["build_success"] = True

    # Process each test
    expected_dir = dst / "expected"
    expected_dir.mkdir(exist_ok=True)
    captured_any = False

    for test_file in test_files:
        test_name = test_name_from_file(test_file)

        # Skip detection
        reason = detect_skip_reason(test_file)
        if reason:
            skipped.append({
                "project": project_name,
                "test": test_file.name,
                "reason": reason,
            })
            report["tests_skipped"] += 1
            continue

        # Find the compiled binary
        binary = find_test_binary(dst, test_name)
        if binary is None:
            skipped.append({
                "project": project_name,
                "test": test_file.name,
                "reason": f"compiled binary not found for {test_name}",
            })
            report["tests_skipped"] += 1
            continue

        # Run and capture
        result = run_binary_and_capture(binary, cwd=dst)
        if result is None:
            skipped.append({
                "project": project_name,
                "test": test_file.name,
                "reason": "timeout (>30s)",
            })
            report["tests_skipped"] += 1
            continue

        # Check for segfault
        if result["exitcode"] == 139 or result["exitcode"] == -11:
            skipped.append({
                "project": project_name,
                "test": test_file.name,
                "reason": f"segfault (exit code {result['exitcode']})",
            })
            report["tests_skipped"] += 1
            continue

        # Check for empty output with success exit
        if not result["stdout"].strip() and result["exitcode"] == 0:
            skipped.append({
                "project": project_name,
                "test": test_file.name,
                "reason": "no stdout output produced",
            })
            report["tests_skipped"] += 1
            continue

        # Write expected output files
        (expected_dir / f"{test_name}.stdout").write_text(result["stdout"])
        (expected_dir / f"{test_name}.stderr").write_text(result["stderr"])
        (expected_dir / f"{test_name}.exitcode").write_text(str(result["exitcode"]))
        report["tests_captured"] += 1
        captured_any = True

    # If no tests were captured, remove the project
    if not captured_any:
        skipped.append({
            "project": project_name,
            "test": "*",
            "reason": "no viable tests after filtering",
        })
        shutil.rmtree(dst)

    return report


def main():
    parser = argparse.ArgumentParser(
        description="Build CBench_int: copy C projects, compile, capture expected output"
    )
    parser.add_argument("--cbench-dir", type=Path, required=True)
    parser.add_argument("--cbench-int-dir", type=Path, required=True)
    parser.add_argument(
        "--projects", type=str, default=None,
        help="Comma-separated list of project names to process (default: all)"
    )
    args = parser.parse_args()

    args.cbench_int_dir.mkdir(parents=True, exist_ok=True)

    # Gather projects
    all_projects = sorted([
        p for p in args.cbench_dir.iterdir()
        if p.is_dir() and not p.name.endswith(".zip")
    ])
    if args.projects:
        names = set(args.projects.split(","))
        all_projects = [p for p in all_projects if p.name in names]

    skipped = []
    reports = []

    for project_dir in tqdm(all_projects, desc="CBench_int"):
        report = process_project(project_dir, args.cbench_int_dir, skipped)
        reports.append(report)

    # Write skipped log
    skipped_path = Path(__file__).parent / "skipped.json"
    with open(skipped_path, "w") as f:
        json.dump(skipped, f, indent=2)

    # Print summary
    built = sum(1 for r in reports if r["build_success"])
    captured = sum(r["tests_captured"] for r in reports)
    skipped_count = sum(r["tests_skipped"] for r in reports)
    projects_with_tests = sum(1 for r in reports if r["tests_captured"] > 0)

    print(f"\n--- CBench_int Summary ---")
    print(f"Projects processed: {len(reports)}")
    print(f"Projects built successfully: {built}")
    print(f"Projects with captured tests: {projects_with_tests}")
    print(f"Total tests captured: {captured}")
    print(f"Total tests skipped: {skipped_count}")
    print(f"Skipped log: {skipped_path}")


if __name__ == "__main__":
    main()
```

- [ ] **Step 2: Test on a single project**

Run:
```bash
cd /nas/CRUST-bench-COLM/CRUST-bench/datasets/create_integration_tests
python capture_expected_output.py \
  --cbench-dir ../CBench \
  --cbench-int-dir ../CBench_int \
  --projects bigint
```

Expected: `CBench_int/bigint/expected/` contains `test1.stdout`, `test1.stderr`, `test1.exitcode` (and same for test2, test3).

- [ ] **Step 3: Test on 3 more projects to validate build system diversity**

Run:
```bash
python capture_expected_output.py \
  --cbench-dir ../CBench \
  --cbench-int-dir ../CBench_int \
  --projects cJSON,bostree,cissy
```

Expected: Projects compile and tests are captured. Check `skipped.json` for any issues. Debug and fix `build_c_project` or `find_test_binary` if binaries aren't found.

- [ ] **Step 4: Commit**

```bash
git add datasets/create_integration_tests/capture_expected_output.py
git commit -m "feat: add C build and expected output capture script"
```

---

### Task 3: RBench_int Scaffolding (`build_rbench_scaffold.py`)

**Files:**
- Create: `datasets/create_integration_tests/build_rbench_scaffold.py`

- [ ] **Step 1: Write `build_rbench_scaffold.py`**

```python
import json
import shutil
import argparse
from pathlib import Path
from tqdm import tqdm
from utils import c_name_to_rust_name


def get_test_names_from_expected(expected_dir: Path) -> list[str]:
    """Get test names from expected output files."""
    names = set()
    for f in expected_dir.iterdir():
        if f.suffix in (".stdout", ".stderr", ".exitcode"):
            names.add(f.stem)
    return sorted(names)


def find_matching_rbench_project(rbench_dir: Path, c_project_name: str) -> Path | None:
    """Find the corresponding RBench project for a C project."""
    rust_name = c_name_to_rust_name(c_project_name)
    candidate = rbench_dir / rust_name
    if candidate.exists():
        return candidate
    # Fallback: try exact name
    candidate = rbench_dir / c_project_name
    if candidate.exists():
        return candidate
    return None


def generate_cargo_toml(project_name: str, test_names: list[str], dependencies: dict) -> str:
    """Generate a Cargo.toml with [[bin]] entries for each test."""
    lines = [
        '[package]',
        f'name = "{project_name}"',
        'version = "0.1.0"',
        'edition = "2021"',
        '',
        '[dependencies]',
    ]
    for dep, version in dependencies.items():
        lines.append(f'{dep} = "{version}"')

    lines.extend([
        '',
        '[lib]',
        'path = "src/lib.rs"',
    ])

    for test_name in test_names:
        lines.extend([
            '',
            '[[bin]]',
            f'name = "{test_name}"',
            f'path = "src/bin/{test_name}.rs"',
        ])

    return "\n".join(lines) + "\n"


def parse_dependencies_from_cargo_toml(cargo_toml_path: Path) -> dict:
    """Extract [dependencies] from an existing Cargo.toml (simple parser)."""
    deps = {}
    in_deps = False
    content = cargo_toml_path.read_text()
    for line in content.splitlines():
        stripped = line.strip()
        if stripped == "[dependencies]":
            in_deps = True
            continue
        if stripped.startswith("[") and in_deps:
            break
        if in_deps and "=" in stripped:
            key, value = stripped.split("=", 1)
            deps[key.strip()] = value.strip().strip('"')
    return deps


def scaffold_project(
    c_project_name: str,
    cbench_int_dir: Path,
    rbench_dir: Path,
    rbench_int_dir: Path,
    manifest: list,
) -> bool:
    """Create RBench_int scaffold for a single project. Returns True on success."""
    rust_name = c_name_to_rust_name(c_project_name)
    cbench_int_project = cbench_int_dir / c_project_name
    expected_dir = cbench_int_project / "expected"

    if not expected_dir.exists():
        return False

    test_names = get_test_names_from_expected(expected_dir)
    if not test_names:
        return False

    # Find matching RBench project
    rbench_project = find_matching_rbench_project(rbench_dir, c_project_name)
    if rbench_project is None:
        print(f"  WARNING: No RBench match for {c_project_name}")
        return False

    # Create RBench_int project directory
    dst = rbench_int_dir / rust_name
    if dst.exists():
        shutil.rmtree(dst)
    dst.mkdir(parents=True)

    # Copy interfaces
    src_dir = dst / "src"
    src_dir.mkdir()
    interfaces_src = rbench_project / "src" / "interfaces"
    if interfaces_src.exists():
        shutil.copytree(interfaces_src, src_dir / "interfaces")

    # Copy lib.rs
    lib_rs_src = rbench_project / "src" / "lib.rs"
    if lib_rs_src.exists():
        shutil.copy2(lib_rs_src, src_dir / "lib.rs")
    else:
        (src_dir / "lib.rs").write_text("// placeholder\n")

    # Create bin directory
    bin_dir = src_dir / "bin"
    bin_dir.mkdir()

    # Parse dependencies from original Cargo.toml
    deps = {}
    orig_cargo = rbench_project / "Cargo.toml"
    if orig_cargo.exists():
        deps = parse_dependencies_from_cargo_toml(orig_cargo)

    # Generate Cargo.toml
    cargo_content = generate_cargo_toml(rust_name, test_names, deps)
    (dst / "Cargo.toml").write_text(cargo_content)

    # Copy expected output
    shutil.copytree(expected_dir, dst / "expected")

    # Add entries to generation manifest
    for test_name in test_names:
        # Find the matching C test source file
        c_test_file = None
        for test_dir in ["tests", "test"]:
            candidate = cbench_int_project / test_dir / f"{test_name}.c"
            if candidate.exists():
                c_test_file = candidate
                break
        if c_test_file is None:
            # Try searching recursively
            for f in cbench_int_project.rglob(f"{test_name}.c"):
                c_test_file = f
                break

        if c_test_file is None:
            print(f"  WARNING: C test source not found for {c_project_name}/{test_name}")
            # Write a placeholder so the structure is complete
            (bin_dir / f"{test_name}.rs").write_text(
                f"// TODO: generate main() for {test_name}\nfn main() {{}}\n"
            )
            continue

        # Collect interface file paths
        interface_files = []
        interfaces_dir = src_dir / "interfaces"
        if interfaces_dir.exists():
            interface_files = sorted(str(f) for f in interfaces_dir.glob("*.rs"))

        manifest.append({
            "project": c_project_name,
            "rust_project": rust_name,
            "test_name": test_name,
            "c_test_path": str(c_test_file),
            "interface_paths": interface_files,
            "lib_rs_path": str(src_dir / "lib.rs"),
            "target_path": str(bin_dir / f"{test_name}.rs"),
        })

    return True


def main():
    parser = argparse.ArgumentParser(
        description="Build RBench_int scaffolds from CBench_int and RBench"
    )
    parser.add_argument("--cbench-int-dir", type=Path, required=True)
    parser.add_argument("--rbench-dir", type=Path, required=True)
    parser.add_argument("--rbench-int-dir", type=Path, required=True)
    parser.add_argument(
        "--projects", type=str, default=None,
        help="Comma-separated C project names to process (default: all)"
    )
    args = parser.parse_args()

    args.rbench_int_dir.mkdir(parents=True, exist_ok=True)

    # Gather projects from CBench_int
    all_projects = sorted([
        p.name for p in args.cbench_int_dir.iterdir()
        if p.is_dir() and (p / "expected").exists()
    ])
    if args.projects:
        names = set(args.projects.split(","))
        all_projects = [p for p in all_projects if p in names]

    manifest = []
    success_count = 0

    for project_name in tqdm(all_projects, desc="RBench_int scaffold"):
        ok = scaffold_project(
            project_name, args.cbench_int_dir, args.rbench_dir,
            args.rbench_int_dir, manifest
        )
        if ok:
            success_count += 1

    # Write generation manifest
    manifest_path = Path(__file__).parent / "generation_manifest.json"
    with open(manifest_path, "w") as f:
        json.dump(manifest, f, indent=2)

    print(f"\n--- RBench_int Scaffold Summary ---")
    print(f"Projects scaffolded: {success_count}/{len(all_projects)}")
    print(f"Generation tasks: {len(manifest)}")
    print(f"Manifest: {manifest_path}")


if __name__ == "__main__":
    main()
```

- [ ] **Step 2: Test on bigint**

Run:
```bash
cd /nas/CRUST-bench-COLM/CRUST-bench/datasets/create_integration_tests
python build_rbench_scaffold.py \
  --cbench-int-dir ../CBench_int \
  --rbench-dir ../RBench \
  --rbench-int-dir ../RBench_int \
  --projects bigint
```

Expected: `RBench_int/bigint/` has `Cargo.toml`, `src/lib.rs`, `src/interfaces/bigint.rs`, `src/bin/` (empty .rs placeholders or no files yet), `expected/`. `generation_manifest.json` has 3 entries (one per test).

- [ ] **Step 3: Verify Cargo.toml structure**

Run:
```bash
cat /nas/CRUST-bench-COLM/CRUST-bench/datasets/RBench_int/bigint/Cargo.toml
```

Expected:
```toml
[package]
name = "bigint"
version = "0.1.0"
edition = "2021"

[dependencies]

[lib]
path = "src/lib.rs"

[[bin]]
name = "test1"
path = "src/bin/test1.rs"

[[bin]]
name = "test2"
path = "src/bin/test2.rs"

[[bin]]
name = "test3"
path = "src/bin/test3.rs"
```

- [ ] **Step 4: Commit**

```bash
git add datasets/create_integration_tests/build_rbench_scaffold.py
git commit -m "feat: add RBench_int scaffold builder with manifest generation"
```

---

### Task 4: Rust `main()` Generation via Claude Code (`generate_rust_mains.py`)

**Files:**
- Create: `datasets/create_integration_tests/generate_rust_mains.py`

This script reads the generation manifest and, for each entry, prints the prompt context. Claude Code processes the manifest by dispatching subagents that read C tests + interfaces and write Rust `main()` files.

- [ ] **Step 1: Write `generate_rust_mains.py`**

```python
import json
import argparse
from pathlib import Path


PROMPT_TEMPLATE = """Given the following C test file and the Rust interface it should use, \
generate ONLY a Rust file with a main() function that:
1. Calls the equivalent Rust API functions defined in the interface
2. Produces identical stdout output to the C version using println!/print!
3. Does NOT include assert! or assert_eq! — this is a coarse-grained integration \
test, not a unit test. Correctness is verified by comparing stdout/exit-code.
4. Does NOT implement the library — only the main() function and any helper functions local to the test

C test file ({c_test_name}):
```c
{c_test_content}
```

Rust interface files:
{interface_content}

Rust lib.rs:
```rust
{lib_rs_content}
```

The crate name is `{crate_name}`.

Generate only the Rust file with necessary `use` statements and a `fn main()`.
Do not generate the library implementation.
Do not include #[test] annotations or assert macros.
Do not wrap the output in markdown code fences."""


def build_prompt(entry: dict) -> str:
    """Build the LLM prompt for a single generation task."""
    c_test_path = Path(entry["c_test_path"])
    c_test_content = c_test_path.read_text(encoding="utf-8", errors="replace")

    interface_parts = []
    for iface_path in entry["interface_paths"]:
        p = Path(iface_path)
        content = p.read_text(encoding="utf-8", errors="replace")
        interface_parts.append(f"// {p.name}\n```rust\n{content}\n```")
    interface_content = "\n\n".join(interface_parts) if interface_parts else "(no interface files)"

    lib_rs_content = Path(entry["lib_rs_path"]).read_text(encoding="utf-8", errors="replace")

    return PROMPT_TEMPLATE.format(
        c_test_name=c_test_path.name,
        c_test_content=c_test_content,
        interface_content=interface_content,
        lib_rs_content=lib_rs_content,
        crate_name=entry["rust_project"],
    )


def main():
    parser = argparse.ArgumentParser(
        description="Generate prompts for Rust main() generation from manifest"
    )
    parser.add_argument(
        "--manifest", type=Path,
        default=Path(__file__).parent / "generation_manifest.json",
    )
    parser.add_argument(
        "--print-prompt", type=str, default=None,
        help="Print the prompt for a specific project/test (format: project/test_name)"
    )
    parser.add_argument(
        "--summary", action="store_true",
        help="Print summary of pending generation tasks"
    )
    args = parser.parse_args()

    with open(args.manifest) as f:
        manifest = json.load(f)

    if args.summary:
        # Group by project
        by_project = {}
        for entry in manifest:
            proj = entry["project"]
            by_project.setdefault(proj, []).append(entry["test_name"])

        pending = 0
        done = 0
        for proj, tests in sorted(by_project.items()):
            for test_name in tests:
                target = Path(manifest[0]["target_path"]).parent.parent.parent.parent / entry["rust_project"] / "src" / "bin" / f"{test_name}.rs"
                # Check from entry directly
                entry_match = [e for e in manifest if e["project"] == proj and e["test_name"] == test_name][0]
                target = Path(entry_match["target_path"])
                if target.exists() and "TODO" not in target.read_text():
                    done += 1
                else:
                    pending += 1
            status_tests = ", ".join(tests)
            print(f"  {proj}: {status_tests}")

        print(f"\nTotal tasks: {len(manifest)} ({done} done, {pending} pending)")
        return

    if args.print_prompt:
        project, test_name = args.print_prompt.split("/")
        for entry in manifest:
            if entry["project"] == project and entry["test_name"] == test_name:
                print(build_prompt(entry))
                return
        print(f"Not found: {args.print_prompt}")
        return

    # Default: print all tasks as JSON with prompts
    for entry in manifest:
        entry["prompt"] = build_prompt(entry)
    print(json.dumps(manifest, indent=2))


if __name__ == "__main__":
    main()
```

- [ ] **Step 2: Test prompt generation**

Run:
```bash
cd /nas/CRUST-bench-COLM/CRUST-bench/datasets/create_integration_tests
python generate_rust_mains.py --manifest generation_manifest.json --print-prompt bigint/test1
```

Expected: Prints a well-formed prompt containing the C test1.c content, the Rust bigint interface, and lib.rs.

- [ ] **Step 3: Commit**

```bash
git add datasets/create_integration_tests/generate_rust_mains.py
git commit -m "feat: add Rust main() prompt generator with manifest support"
```

- [ ] **Step 4: Generate Rust `main()` files using Claude Code**

For each entry in the manifest, Claude Code should:
1. Read the C test file and Rust interface files
2. Generate a Rust `main()` function that produces identical stdout
3. Write it to the target path

This is done by dispatching parallel Claude Code subagents — one per project. Each subagent processes all tests for that project:

```
For each project in generation_manifest.json:
  Read: <c_test_path>
  Read: each file in <interface_paths>
  Read: <lib_rs_path>
  Generate Rust main() file
  Write: <target_path>
```

Use `--projects` flag on the orchestrator to process in batches if needed.

- [ ] **Step 5: Verify generated files**

Run:
```bash
# Check that generated files have fn main() and no TODO placeholders
for f in /nas/CRUST-bench-COLM/CRUST-bench/datasets/RBench_int/bigint/src/bin/*.rs; do
  echo "=== $(basename $f) ==="
  grep -c "fn main" "$f"
  grep -c "TODO" "$f"
done
```

Expected: Each file has 1 `fn main`, 0 `TODO`.

- [ ] **Step 6: Commit generated files**

```bash
git add datasets/RBench_int/
git commit -m "feat: add generated Rust main() functions for integration tests"
```

---

### Task 5: Validation Harness (`validate.py`)

**Files:**
- Create: `datasets/create_integration_tests/validate.py`

- [ ] **Step 1: Write `validate.py`**

```python
import json
import subprocess
import argparse
import difflib
from pathlib import Path
from tqdm import tqdm


def get_expected(expected_dir: Path, test_name: str) -> dict | None:
    """Load expected output for a test."""
    stdout_file = expected_dir / f"{test_name}.stdout"
    stderr_file = expected_dir / f"{test_name}.stderr"
    exitcode_file = expected_dir / f"{test_name}.exitcode"

    if not stdout_file.exists() or not exitcode_file.exists():
        return None

    return {
        "stdout": stdout_file.read_text(),
        "stderr": stderr_file.read_text() if stderr_file.exists() else "",
        "exitcode": int(exitcode_file.read_text().strip()),
    }


def get_bin_names(project_dir: Path) -> list[str]:
    """Get binary target names from Cargo.toml."""
    cargo_toml = project_dir / "Cargo.toml"
    if not cargo_toml.exists():
        return []
    names = []
    content = cargo_toml.read_text()
    in_bin = False
    for line in content.splitlines():
        stripped = line.strip()
        if stripped == "[[bin]]":
            in_bin = True
            continue
        if in_bin and stripped.startswith("name"):
            name = stripped.split("=", 1)[1].strip().strip('"')
            names.append(name)
            in_bin = False
    return names


def build_project(project_dir: Path, timeout: int = 120) -> tuple[bool, str]:
    """Build the Rust project. Returns (success, error_message)."""
    try:
        result = subprocess.run(
            ["cargo", "build"],
            cwd=project_dir,
            capture_output=True,
            text=True,
            timeout=timeout,
        )
        if result.returncode != 0:
            return False, result.stderr
        return True, ""
    except subprocess.TimeoutExpired:
        return False, "build timed out"


def run_bin(project_dir: Path, bin_name: str, timeout: int = 30) -> dict | None:
    """Run a binary and capture output."""
    try:
        result = subprocess.run(
            ["cargo", "run", "--bin", bin_name],
            cwd=project_dir,
            capture_output=True,
            text=True,
            timeout=timeout,
        )
        return {
            "stdout": result.stdout,
            "stderr": result.stderr,
            "exitcode": result.returncode,
        }
    except subprocess.TimeoutExpired:
        return None


def normalize_stdout(text: str) -> str:
    """Normalize stdout for comparison: strip trailing whitespace per line."""
    lines = text.splitlines()
    return "\n".join(line.rstrip() for line in lines)


def compare(expected: dict, actual: dict, strict_stderr: bool = False) -> dict:
    """Compare expected and actual output. Returns result dict."""
    result = {"pass": True, "diffs": []}

    exp_stdout = normalize_stdout(expected["stdout"])
    act_stdout = normalize_stdout(actual["stdout"])

    if exp_stdout != act_stdout:
        result["pass"] = False
        diff = list(difflib.unified_diff(
            exp_stdout.splitlines(keepends=True),
            act_stdout.splitlines(keepends=True),
            fromfile="expected",
            tofile="actual",
        ))
        result["diffs"].append(("stdout", "".join(diff)))

    if expected["exitcode"] != actual["exitcode"]:
        result["pass"] = False
        result["diffs"].append((
            "exitcode",
            f"expected {expected['exitcode']}, got {actual['exitcode']}"
        ))

    if strict_stderr:
        exp_stderr = normalize_stdout(expected["stderr"])
        act_stderr = normalize_stdout(actual["stderr"])
        if exp_stderr != act_stderr:
            result["pass"] = False
            diff = list(difflib.unified_diff(
                exp_stderr.splitlines(keepends=True),
                act_stderr.splitlines(keepends=True),
                fromfile="expected",
                tofile="actual",
            ))
            result["diffs"].append(("stderr", "".join(diff)))

    return result


def validate_project(project_dir: Path, strict_stderr: bool = False) -> dict:
    """Validate a single RBench_int project. Returns report."""
    project_name = project_dir.name
    expected_dir = project_dir / "expected"
    report = {
        "project": project_name,
        "build_success": False,
        "results": [],
    }

    # Build
    ok, err = build_project(project_dir)
    if not ok:
        report["build_error"] = err
        return report
    report["build_success"] = True

    # Get binary names
    bin_names = get_bin_names(project_dir)

    for bin_name in bin_names:
        test_result = {"test": bin_name, "pass": False}

        expected = get_expected(expected_dir, bin_name)
        if expected is None:
            test_result["error"] = "no expected output files"
            report["results"].append(test_result)
            continue

        actual = run_bin(project_dir, bin_name)
        if actual is None:
            test_result["error"] = "timeout"
            report["results"].append(test_result)
            continue

        comparison = compare(expected, actual, strict_stderr)
        test_result["pass"] = comparison["pass"]
        if not comparison["pass"]:
            test_result["diffs"] = comparison["diffs"]
        report["results"].append(test_result)

    return report


def print_report(report: dict):
    """Print a human-readable report for a project."""
    print(f"\nProject: {report['project']}")
    if not report["build_success"]:
        print(f"  BUILD FAILED: {report.get('build_error', 'unknown')[:200]}")
        return

    passed = sum(1 for r in report["results"] if r["pass"])
    total = len(report["results"])

    for r in report["results"]:
        if r["pass"]:
            print(f"  {r['test']}: PASS")
        elif "error" in r:
            print(f"  {r['test']}: ERROR - {r['error']}")
        else:
            print(f"  {r['test']}: FAIL")
            for diff_type, diff_text in r.get("diffs", []):
                print(f"    {diff_type} diff:")
                for line in diff_text.splitlines()[:10]:
                    print(f"      {line}")

    print(f"  Summary: {passed}/{total} passed")


def main():
    parser = argparse.ArgumentParser(
        description="Integration test harness: validate RBench_int projects"
    )
    parser.add_argument("--project", type=Path, default=None, help="Single project dir")
    parser.add_argument("--rbench-int-dir", type=Path, default=None, help="All projects")
    parser.add_argument("--all", action="store_true", help="Validate all projects")
    parser.add_argument("--strict-stderr", action="store_true")
    parser.add_argument("--json-output", type=Path, default=None, help="Write JSON report")
    args = parser.parse_args()

    reports = []

    if args.project:
        report = validate_project(args.project, args.strict_stderr)
        print_report(report)
        reports.append(report)
    elif args.all and args.rbench_int_dir:
        projects = sorted([
            p for p in args.rbench_int_dir.iterdir()
            if p.is_dir() and (p / "Cargo.toml").exists()
        ])
        for project_dir in tqdm(projects, desc="Validating"):
            report = validate_project(project_dir, args.strict_stderr)
            print_report(report)
            reports.append(report)

        # Global summary
        total_pass = sum(
            sum(1 for r in rep["results"] if r["pass"]) for rep in reports
        )
        total_tests = sum(len(rep["results"]) for rep in reports)
        total_build_ok = sum(1 for rep in reports if rep["build_success"])

        print(f"\n=== GLOBAL SUMMARY ===")
        print(f"Projects: {len(reports)} ({total_build_ok} built)")
        print(f"Tests: {total_pass}/{total_tests} passed")
    else:
        parser.print_help()

    if args.json_output:
        # Convert diffs tuples to serializable format
        serializable = []
        for rep in reports:
            r = dict(rep)
            results = []
            for tr in r.get("results", []):
                tr2 = dict(tr)
                if "diffs" in tr2:
                    tr2["diffs"] = [
                        {"type": d[0], "content": d[1]} for d in tr2["diffs"]
                    ]
                results.append(tr2)
            r["results"] = results
            serializable.append(r)
        with open(args.json_output, "w") as f:
            json.dump(serializable, f, indent=2)


if __name__ == "__main__":
    main()
```

- [ ] **Step 2: Verify syntax**

Run:
```bash
cd /nas/CRUST-bench-COLM/CRUST-bench/datasets/create_integration_tests
python -c "import validate; print('OK')"
```

Expected: `OK`

- [ ] **Step 3: Commit**

```bash
git add datasets/create_integration_tests/validate.py
git commit -m "feat: add integration test validation harness"
```

---

### Task 6: Main Orchestrator (`create_integration_tests.py`)

**Files:**
- Create: `datasets/create_integration_tests/create_integration_tests.py`

- [ ] **Step 1: Write `create_integration_tests.py`**

```python
import argparse
import subprocess
import sys
from pathlib import Path


def run_step(description: str, cmd: list[str], cwd: Path):
    """Run a pipeline step and abort on failure."""
    print(f"\n{'='*60}")
    print(f"STEP: {description}")
    print(f"{'='*60}")
    result = subprocess.run(cmd, cwd=cwd)
    if result.returncode != 0:
        print(f"FAILED: {description}")
        sys.exit(1)
    print(f"DONE: {description}\n")


def main():
    parser = argparse.ArgumentParser(
        description="Create integration test datasets (CBench_int + RBench_int)"
    )
    parser.add_argument(
        "--cbench-dir", type=Path,
        default=Path(__file__).parent.parent / "CBench",
    )
    parser.add_argument(
        "--rbench-dir", type=Path,
        default=Path(__file__).parent.parent / "RBench",
    )
    parser.add_argument(
        "--cbench-int-dir", type=Path,
        default=Path(__file__).parent.parent / "CBench_int",
    )
    parser.add_argument(
        "--rbench-int-dir", type=Path,
        default=Path(__file__).parent.parent / "RBench_int",
    )
    parser.add_argument(
        "--projects", type=str, default=None,
        help="Comma-separated project names (default: all)",
    )
    parser.add_argument(
        "--skip-capture", action="store_true",
        help="Skip Step 1 (C build + capture) if CBench_int already exists",
    )
    parser.add_argument(
        "--skip-scaffold", action="store_true",
        help="Skip Step 2a (RBench scaffold) if RBench_int already exists",
    )
    args = parser.parse_args()

    script_dir = Path(__file__).parent
    project_filter = ["--projects", args.projects] if args.projects else []

    # Step 1: Build CBench_int
    if not args.skip_capture:
        run_step(
            "Build CBench_int: compile C projects and capture expected output",
            [
                sys.executable, str(script_dir / "capture_expected_output.py"),
                "--cbench-dir", str(args.cbench_dir),
                "--cbench-int-dir", str(args.cbench_int_dir),
            ] + project_filter,
            cwd=script_dir,
        )

    # Step 2a: Build RBench_int scaffolds
    if not args.skip_scaffold:
        run_step(
            "Build RBench_int scaffolds and generation manifest",
            [
                sys.executable, str(script_dir / "build_rbench_scaffold.py"),
                "--cbench-int-dir", str(args.cbench_int_dir),
                "--rbench-dir", str(args.rbench_dir),
                "--rbench-int-dir", str(args.rbench_int_dir),
            ] + project_filter,
            cwd=script_dir,
        )

    # Step 2b: Generation manifest is ready
    manifest_path = script_dir / "generation_manifest.json"
    print(f"\n{'='*60}")
    print(f"STEP: Rust main() generation")
    print(f"{'='*60}")
    print(f"Generation manifest ready at: {manifest_path}")
    print(f"Use Claude Code to process the manifest:")
    print(f"  python generate_rust_mains.py --summary")
    print(f"  python generate_rust_mains.py --print-prompt <project>/<test>")
    print(f"{'='*60}\n")


if __name__ == "__main__":
    main()
```

- [ ] **Step 2: Commit**

```bash
git add datasets/create_integration_tests/create_integration_tests.py
git commit -m "feat: add main orchestrator for integration test pipeline"
```

---

### Task 7: End-to-End Test on Pilot Projects

Run the full pipeline on 2-3 small projects to validate everything works before processing all 100.

**Files:**
- Modify: any scripts that have bugs discovered during testing

- [ ] **Step 1: Run Step 1 (C capture) on pilot projects**

```bash
cd /nas/CRUST-bench-COLM/CRUST-bench/datasets/create_integration_tests
python capture_expected_output.py \
  --cbench-dir ../CBench \
  --cbench-int-dir ../CBench_int \
  --projects bigint,avalanche,cissy
```

Verify:
- `CBench_int/bigint/expected/` has test1, test2, test3 output files
- `CBench_int/avalanche/expected/` has output files
- `CBench_int/cissy/expected/` has output files
- `skipped.json` is populated with any skip reasons

- [ ] **Step 2: Run Step 2a (scaffold) on pilot projects**

```bash
python build_rbench_scaffold.py \
  --cbench-int-dir ../CBench_int \
  --rbench-dir ../RBench \
  --rbench-int-dir ../RBench_int \
  --projects bigint,avalanche,cissy
```

Verify:
- `RBench_int/bigint/` has `Cargo.toml` with `[[bin]]` entries, `src/interfaces/`, `expected/`
- `generation_manifest.json` has entries for all tests

- [ ] **Step 3: Generate Rust `main()` files for pilot projects using Claude Code**

For each entry in `generation_manifest.json`, Claude Code reads the C test and Rust interfaces, generates the Rust `main()`, and writes it to the target path. Use parallel subagents — one per project.

- [ ] **Step 4: Validate pilot projects**

```bash
python validate.py --project ../RBench_int/bigint
python validate.py --project ../RBench_int/avalanche
python validate.py --project ../RBench_int/cissy
```

Expected: BUILD SUCCESS for all. Test results show PASS (if the existing RBench implementations are correct) or meaningful FAIL diffs.

Note: Since RBench_int only has `main()` + interfaces but no library implementation, `cargo build` will fail until an LLM fills in the implementation. To test the harness itself, temporarily copy the implementation from RBench:

```bash
# Temporary: copy implementation to test the harness
cp ../RBench/bigint/src/bin/bigint.rs ../RBench_int/bigint/src/interfaces/
# Then run validate.py
```

- [ ] **Step 5: Fix any bugs discovered and commit**

```bash
git add datasets/create_integration_tests/
git commit -m "fix: address issues found during pilot testing"
```

---

### Task 8: Full Pipeline Run

- [ ] **Step 1: Run Step 1 on all projects**

```bash
cd /nas/CRUST-bench-COLM/CRUST-bench/datasets/create_integration_tests
python capture_expected_output.py \
  --cbench-dir ../CBench \
  --cbench-int-dir ../CBench_int
```

This processes all ~100 projects. May take 10-20 minutes. Check `skipped.json` afterward.

- [ ] **Step 2: Run Step 2a on all projects**

```bash
python build_rbench_scaffold.py \
  --cbench-int-dir ../CBench_int \
  --rbench-dir ../RBench \
  --rbench-int-dir ../RBench_int
```

- [ ] **Step 3: Review manifest and generate all Rust `main()` files**

```bash
python generate_rust_mains.py --summary
```

Claude Code dispatches subagents to generate all Rust `main()` files from the manifest. Process in batches of ~10 projects per round of parallel agents.

- [ ] **Step 4: Structural validation**

Verify every RBench_int project has:
```bash
# Check all bin files exist and have fn main()
for proj in ../RBench_int/*/; do
  for rs in "$proj"/src/bin/*.rs; do
    if ! grep -q "fn main" "$rs" 2>/dev/null; then
      echo "MISSING main(): $rs"
    fi
  done
done
```

- [ ] **Step 5: Commit final datasets**

```bash
git add datasets/CBench_int/ datasets/RBench_int/
git add datasets/create_integration_tests/skipped.json
git add datasets/create_integration_tests/generation_manifest.json
git commit -m "feat: complete CBench_int and RBench_int integration test datasets"
```

- [ ] **Step 6: Print final summary**

```bash
echo "CBench_int projects:" && ls ../CBench_int/ | wc -l
echo "RBench_int projects:" && ls ../RBench_int/ | wc -l
echo "Skipped entries:" && python -c "import json; d=json.load(open('skipped.json')); print(len(d))"
echo "Generation tasks:" && python -c "import json; d=json.load(open('generation_manifest.json')); print(len(d))"
```
