@amendment marker=2026-11-04-linux-and-macos-parity
title: the OS-gated tests census
section: Cross-cutting Patterns (the body's cross-platform discipline line; cross-cutting-patterns.md: a new group) · Existing Scopes (the body's subjects list)
change:
  - The body's cross-platform discipline line reads «Every OS-specific branch compiles and is tested on its CI runner, and every OS-gated test names its reason in the census binary `tests/contract_os_gated_tests.rs`» (was the first clause alone).
  - Cross-cutting Patterns gains the group «OS-gated tests named with their reason»: the E2E contract binary `tests/contract_os_gated_tests.rs` (a `contract_` binary, auto-discovered with no `[[test]]` table, run under `run --e2e`, carrying `andromeda:walks-tree`) holds a `GATES` table, one `Gate { file, test, gate, reason }` row per OS-gated test function or gated file, each with a non-empty reason. Its census walks every `.rs` directly under `tests/` and under each `crates/*/tests/`, taking a file with a leading `#![cfg(…)]` and a test function whose contiguous attribute run holds `#[test]`, `#[rstest]` or `#[tokio::test]` and a `#[cfg(…)]` or `#[cfg_attr(…)]` whose predicate names `windows`, `unix` or `target_os` outside its string values (so `feature = "unix-like"` is no hit). `os_gated_tests_are_each_named_with_their_reason` fails both ways, on a gate no row names and on a row whose gate no longer stands; `gates_table_rows_are_unique` holds one row per gate; the self-check `census_finds_an_unnamed_gate_and_passes_a_named_one` reads eight planted cases. A new OS gate on a test lands with its `GATES` row and reason, or the census goes red.
  - The body's subjects list gains `crosscut-os-gated-tests`, after `crosscut-sanitised-error-surfaces`.
why: the chunk landed the census binary in the E2E layer, passing on the three CI runners on the final pushed sha; no standing architecture text named it. Standing rule: every new OS gate on a test carries its reason in `GATES`.
ref: .andromeda/runs/2026-11-04T21-59-44-wrap/
@amendment marker=2026-11-04-linux-and-macos-parity
title: the root waits on the WITHIN bound
section: Occupied Resources (occupied-resources.md: the root test support `Watch` entry)
change:
  - `Instant::now() + WITHIN` reads 32 sites in 24 files under `tests/`, its rule `grep -rn 'Instant::now() + WITHIN' tests/ | wc -l` and `grep -rln 'Instant::now() + WITHIN' tests/ | wc -l`, as measured at chunk 2026-11-04-linux-and-macos-parity's wrap on the Linux dev host (was 28 sites in 20 files, as measured at chunk 2026-10-10-budget-governor's wrap, with its note of the two sites added since chunk 2026-10-10-viola-revive, now dropped).
  - `tests/support/frozen.rs` `Frozen` is named a user of the bound: on Unix its `attach` polls `ps -o stat=` under a `Watch` bounded by `super::watch::WITHIN` until the frozen wrapper reads `T`.
why: the 28 was stale: the chunk's gate and its Outcome read 32 by the count's rule, and the wrap's own reading on the Linux dev host gives 24 files; the Unix `Frozen`, moved into the shared support this chunk, polls under the bound.
ref: .andromeda/runs/2026-11-04T21-59-44-wrap/
@amendment marker=2026-11-04-linux-and-macos-parity
title: the shared Frozen guard
section: Occupied Resources (occupied-resources.md: a new test-only entry) · Project directory structure (contracts/project-directory-structure: the `tests/support/` row)
change:
  - Occupied Resources gains a test-only entry: `tests/support/frozen.rs` (`pub mod frozen;` in `tests/support/mod.rs`) holds the `Frozen` guard, `attach(pid)` and its `Drop`, one twin per OS under its own gate: on Unix `attach` sends `kill -STOP` and polls `ps -o stat=` until it reads `T`, and the drop sends `kill -CONT`; on Windows `attach` attaches a debugger with kill-on-exit off and waits for the first debug event, and the drop detaches. It is the guard's one home, the way a root test freezes a wrapper so that it reads stale; its callers are in `tests/cli_instance_state.rs` and `tests/cross_list_rows.rs`. `viola` never names it.
  - The project tree's `tests/support/` row names `frozen.rs` after `sgr.rs`: the shared freeze guard `Frozen` (`attach(pid)` + `Drop`; Unix `kill -STOP` polled to `ps` state `T` within `WITHIN`, `kill -CONT` on drop; Windows a debugger attach, detached on drop), its one home, used by `tests/cli_instance_state.rs` and `tests/cross_list_rows.rs`.
why: the chunk moved the two local `Frozen` twins whole out of `tests/cli_instance_state.rs` into `tests/support/frozen.rs`, its sole home, its callers in two files (the new stale-row witness in `tests/cross_list_rows.rs` among them); no architecture passage named the guard.
ref: .andromeda/runs/2026-11-04T21-59-44-wrap/
@amendment marker=2026-11-04-linux-and-macos-parity
title: the per-OS JUnit script
section: Occupied Resources (occupied-resources.md: a new entry beside the repository's scripts)
change:
  - Occupied Resources gains `scripts/junit-os-sets.py` (the python 3 standard library and the host's `gh`), the operator pass's per-OS JUnit reading: `--run <id>` downloads the `junit-windows-2025`, `junit-macos-latest` and `junit-ubuntu-latest` artifacts with `gh run download` into a `tempfile.mkdtemp()` dir removed on exit, and prints `{os}: {n} cases · {f} failed · {s} skipped` per OS, then `## {label}: {count}` and the sorted `classname::name` lines of `ubuntu-not-macos`, `macos-not-ubuntu`, `windows-not-ubuntu` and `ubuntu-not-windows`. It exits 0, 1 when a report is missing or unparsable (the report named), and 2 on a usage error; it reads no environment variable and writes nothing in the tree. It is an operator-pass instrument, never a CI step.
why: the chunk landed the script, run by the plan's operator leg and never by CI; the plan's expected amendment places it in architecture beside the repository's scripts.
kept: the project tree's `scripts/` listing has no row for it: the row's place is a new line inside that code block, which an in-line change cannot carry, and the wrap's apply ruling folded it into this entry, the tree listing of `scripts/` being no census.
ref: .andromeda/runs/2026-11-04T21-59-44-wrap/
