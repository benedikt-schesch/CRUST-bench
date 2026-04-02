# Integration Test Dataset Design (CBench_int / RBench_int)

## Goal

Create two paired datasets — **CBench_int** and **RBench_int** — that enable end-to-end behavioral equivalence testing of LLM-generated Rust transpilations against original C implementations using I/O comparison of compiled binaries.

These are **integration tests**, not unit tests. They compile full programs, run them as standalone binaries, and verify the entire program's observable output (stdout, stderr, exit code) matches between C and Rust.

## Context

- **CBench/** contains ~100 C projects with unit tests in `tests/` — standalone `.c` files with `main()` that compile to executables, print output, and use `assert()`
- **RBench/** contains ~100 corresponding Rust projects with tests in `src/bin/` using `#[test]` annotations and empty `main(){}`
- Name mapping: C project hyphens become underscores in Rust (e.g., `btree-map` -> `btree_map`), digit-prefixed names get `proj_` prefix
- Build systems: C uses CMake, Rust uses Cargo
- The existing `Benchmark` class in `src/benchmark.py` handles the C-to-Rust name mapping

## Dataset Structure

```
datasets/
├── CBench/              (existing - source of truth)
├── RBench/              (existing - current Rust counterparts)
├── CBench_int/          (new - C projects with expected output)
│   └── <project>/
│       ├── src/              (copied from CBench)
│       ├── tests/            (copied from CBench)
│       ├── CMakeLists.txt    (copied from CBench)
│       └── expected/
│           ├── test1.stdout
│           ├── test1.stderr
│           ├── test1.exitcode
│           ├── test2.stdout
│           └── ...
├── RBench_int/          (new - Rust scaffolds for LLM)
│   └── <project>/
│       ├── Cargo.toml
│       ├── src/
│       │   ├── lib.rs
│       │   ├── interfaces/   (copied from RBench)
│       │   └── bin/
│       │       ├── test1.rs  (auto-generated main())
│       │       ├── test2.rs  (auto-generated main())
│       │       └── ...
│       └── expected/         (same files as CBench_int)
│           ├── test1.stdout
│           ├── test1.stderr
│           ├── test1.exitcode
│           └── ...
└── create_integration_tests/
    ├── create_integration_tests.py  (main orchestrator)
    ├── generate_rust_mains.py       (LLM-based main() generation)
    ├── capture_expected_output.py   (compile & run C, capture output)
    ├── validate.py                  (integration test harness)
    └── skipped.json                 (log of skipped tests + reasons)
```

## Pipeline

The pipeline has three sequential steps.

### Step 1: Build CBench_int — Copy, Compile, Capture Expected Output

For each project in CBench:

1. **Copy** the project into `CBench_int/<project>/` (src, tests, CMakeLists.txt, any data files)
2. **Compile** using CMake: `cmake -B build && cmake --build build`
3. **For each test binary** (identified from CMakeLists.txt or by globbing compiled executables):
   a. **Skip detection** — static analysis of the `.c` source for:
      - `scanf`, `fgets`, `getline`, `read(STDIN` — stdin dependency
      - `srand(time(`, `rand()` without fixed seed — non-deterministic output
      - `fopen` where the opened path is not within the project directory — external file dependency
   b. If problematic: log to `skipped.json` with project name, test name, and reason; skip this test
   c. Otherwise: run the binary with a 30-second timeout, capture:
      - stdout -> `expected/<testname>.stdout`
      - stderr -> `expected/<testname>.stderr`
      - exit code -> `expected/<testname>.exitcode` (as a plain text integer)
4. If a project fails to compile or has zero viable tests after filtering: skip the entire project, log it in `skipped.json`

### Step 2: Build RBench_int — Generate Rust Scaffolds with Auto-Generated `main()` Functions

For each project that survived Step 1:

1. **Create** `RBench_int/<project>/` with:
   - `Cargo.toml` with package name, `[lib]` section, and `[[bin]]` entries for each test
   - `src/lib.rs` (copied from RBench, or a stub with module declarations)
   - `src/interfaces/` (copied from RBench — these define the API contract)
2. **For each viable test** (those not skipped in Step 1):
   a. Read the C test file: `CBench/<project>/tests/<testname>.c`
   b. Read the Rust interface files: `RBench/<project>/src/interfaces/*.rs`
   c. **Prompt Claude Code** to generate a Rust `main()` function (see prompt template below)
   d. Write the result to `RBench_int/<project>/src/bin/<testname>.rs`
   e. Add a `[[bin]]` entry to `Cargo.toml`
3. **Copy** the `expected/` directory from `CBench_int/<project>/` into `RBench_int/<project>/`

### Step 3: Validation

1. For each project in RBench_int, verify:
   - `Cargo.toml` is well-formed (parseable as TOML)
   - All `src/bin/<testname>.rs` files exist and are non-empty
   - Expected output files exist for every binary target
   - Each `src/bin/<testname>.rs` contains a `fn main()`
2. Produce a summary report:
   - Total projects processed
   - Total projects in CBench_int / RBench_int
   - Total tests per project
   - Total tests skipped (with breakdown by reason)
   - Any projects that failed at any stage

## Prompt Template for Rust `main()` Generation

```
Given the following C test file and the Rust interface it should use,
generate ONLY a Rust file with a main() function that:
1. Calls the equivalent Rust API functions defined in the interface
2. Produces identical stdout output to the C version
3. Preserves any assert! checks for correctness
4. Uses `println!`/`print!` to match printf output exactly
5. Does NOT implement the library — only the main() function and any
   helper functions local to the test

C test file:
{c_test_content}

Rust interface:
{rust_interface_content}

Existing Rust lib.rs:
{rust_lib_content}

Generate only the Rust file with necessary `use` statements and a `fn main()`.
Do not generate the library implementation.
Do not wrap the output in markdown code fences.
```

## Skip Detection Heuristics

A test is skipped if its `.c` source matches any of the following:

| Pattern | Reason |
|---------|--------|
| `scanf`, `fgets`, `getline`, `read(0,`, `read(STDIN` | stdin dependency |
| `srand(time(`, `srand(clock(` | non-deterministic (time-seeded RNG) |
| `rand()` without a preceding fixed `srand(<integer>)` | non-deterministic RNG |
| Binary segfaults (exit code 139) | crash — can't capture meaningful output |
| Binary times out (>30s) | hang or infinite loop |
| Binary produces no stdout and exit code 0 | no observable output to compare |

Skipped tests are logged in `skipped.json`:
```json
[
  {
    "project": "carrays",
    "test": "test.c",
    "reason": "non-deterministic: srand(time(NULL))"
  }
]
```

## Integration Test Harness (`validate.py`)

Used **after** an LLM fills in the Rust library implementation in an RBench_int project.

### Usage

```bash
# Validate a single project
python validate.py --project datasets/RBench_int/<project>

# Validate all projects
python validate.py --all --rbench-int-dir datasets/RBench_int
```

### Per-test validation flow

1. `cargo build --bin <testname>` — compile the binary
2. Run the binary with a 30-second timeout
3. Capture stdout, stderr, exit code
4. Compare against `expected/<testname>.{stdout,stderr,exitcode}`:
   - **stdout**: exact string match (after stripping trailing whitespace per line)
   - **stderr**: not compared by default (warnings, debug output may differ); optionally enable with `--strict-stderr`
   - **exit code**: exact integer match
5. Report per-test: PASS or FAIL (with diff on failure)

### Output

```
Project: bigint
  test1: PASS
  test2: PASS
  test3: FAIL
    stdout diff:
      - expected: "Result: 42"
      + actual:   "Result: 43"

Summary: 2/3 passed
```

## Name Mapping

Reuse the existing logic from `src/benchmark.py`:

```python
project_name = c_project_name.replace("-", "_")
if project_name[0].isdigit():
    project_name = "proj_" + project_name
```

This maps CBench project names to RBench_int project names consistently.

## Execution Plan

The `create_integration_tests.py` orchestrator runs Steps 1-3 sequentially:

```bash
python create_integration_tests.py \
  --cbench-dir datasets/CBench \
  --rbench-dir datasets/RBench \
  --cbench-int-dir datasets/CBench_int \
  --rbench-int-dir datasets/RBench_int
```

It should:
- Support `--projects <name1,name2,...>` to process a subset for testing
- Log progress with `tqdm` (consistent with existing codebase style)
- Be idempotent: re-running skips projects that already have complete output
- Write `skipped.json` incrementally so partial runs are recoverable

## Autonomy with Claude Code

The entire pipeline is designed to run autonomously:
1. Claude Code executes `create_integration_tests.py`
2. Step 1 (C compilation + output capture) is fully automated
3. Step 2 (Rust main generation) uses Claude Code itself as the LLM — reading C tests and interfaces, generating Rust main functions, writing them to disk
4. Step 3 (validation) is fully automated
5. The final `skipped.json` and summary report provide full traceability

No human intervention is needed unless a project has unusual build requirements.
