# Remove-the-guard runs — 2026-09-25-security-prerequisites

One-shot mutating controls (plan step 9), run at /implement on this Windows host with toolchain 1.98.1-msvc via
`cargo nextest run -p viola --test <file> --profile ci`. Each guard was neutralised, the test run, the guard restored.
Logs: `.andromeda/runs/2026-09-25T12-41-13-implement/rtg-*.log`.

| run | guard neutralised | reading | restored |
|---|---|---|---|
| (a) | `SQOS_OPEN` = `FILE_FLAG_OVERLAPPED` only (SQOS flags dropped) | **red** — `sqos_identification_open_adopted_reads_identification` FAIL: `server must read SecurityIdentification`, left `2`, right `1`; the control test PASS; rc 100 (`rtg-a.log`) | yes |
| (b) | `SQOS_OPEN` = `SECURITY_SQOS_PRESENT \| SECURITY_IDENTIFICATION` (overlapped dropped) | **red** — the identification test hangs: SLOW at 30/60/90 s, `TIMEOUT [120.036s]`, terminated by the nextest `ci` profile (30 s × 4); the control test PASS; rc 100 (`rtg-b.log`). Reproduces research.md case B on the real test. No `channel_sqos_open*` process survived (Win32_Process census) | yes |
| (c) | `abc` KAT digest last hex digit `d` → `e` | **red** — `sha256_published_vector_matches_digest::case_2_abc` FAIL, left `…15ad` (computed), right `…15ae`; 4 passed, 1 failed; rc 100 (`rtg-c.log`) | yes |

Restored state: both files, 7 tests run, 7 passed, rc 0 (`rtg-restored.log`).

## Operator fold (option A): the `test-only-rust-delta` verdict in `crates/viola-e2e/src/harness/run/mutants.rs`

Run with `CARGO_TARGET_DIR=target/mutants-fold cargo nextest run -p viola-e2e --profile ci -E
'test(/mutants|rust_delta|rust_paths|test_target/)'` (27 selected).

| run | guard neutralised | reading | restored |
|---|---|---|---|
| (d) | classifier forced true: `rust_files.iter().all(\|_\| true)` | **red** — 17 passed, 10 failed, among them the operator's literal-oracle guard `run_mutants_mixed_src_and_test_delta_without_outcomes_stays_outcomes_missing` (left 0, right 1: the mixed diff passed instead of staying `outcomes-missing`); rc 100 (`rtg-d.log`) | yes |
| (e) | classifier forced false: `rust_files.iter().all(\|_\| false)` | **red** — `run_mutants_test_only_delta_passes_by_name_without_running_cargo` FAIL, 26 passed / 1 failed; rc 100 (`rtg-e.log`) | yes |

Restored state: 27 tests run, 27 passed, rc 0 (`rtg-de-restored.log`).

Measurement behind the benches/examples choice (cargo-mutants 27.1.0, throwaway package `mprobe` with one function each
in `src/lib.rs`, `tests/it.rs`, `benches/b.rs`, `examples/e.rs`): `cargo mutants --list-files` → `src/lib.rs` only;
`--list` → two mutants, both in `src/lib.rs`. Auto-discovered test, bench and example targets are never mutated, so all
three directories count as test targets.

Non-mutating guards that re-check the restored state on every gate re-run: plan entries
`grep -c -E 'SECURITY_SQOS_PRESENT \| SECURITY_IDENTIFICATION \| FILE_FLAG_OVERLAPPED'` (last line 1) and the
filtered `run --integration` entry (7 passed).
