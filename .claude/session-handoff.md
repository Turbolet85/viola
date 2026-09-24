# Session Handoff

**Last Updated:** 2026-09-24T15:08:00Z
**Branch:** build/viola-0.1.0 · 0 ahead of origin/build/viola-0.1.0 as read at this wrap's Setup
**Status:** clean
**Last Commit:** 2026-09-24-quality-gates — feat: per-job gate verdict, per-OS coverage, MSRV 1.96, two-leg mutation union, seeded fuzz replay

## Position
- Done: 2026-09-24-quality-gates.
  - New `viola-harness gate` with per-job `--require`.
  - `run --coverage`: one instrumented run per OS, floors 85/95/80.
  - `run --fuzz-replay` and the `fuzz/` workspace with the `viola_name` target.
  - Mutation runs as a two-leg matrix (ubuntu + windows) with a `mutants-verdict` union.
  - New `msrv` job on 1.96 via rustup.
  - Zero-retries contract test.
  - The `mutants.out/` upload is removed.
- Next: `/andromeda-phase` to promote and plan "Workspace tree and code-graph planes". Its PREREQ fires first; see below.
- **CI witness owed right after this push** (the plan's operator entries):
  - check-runs all `success` on the pushed sha;
  - the `mutants-verdict-ubuntu-latest` artifact lists the 3 `replace file_mode` mutants `caught`;
  - the `msrv` log carries `rustc 1.96`.

  Any red folds into "Workspace tree and code-graph planes" (its PREREQ).
- Local numbers (Windows host, product files only):
  - coverage: lines 97.86 / functions **95.40** / regions 98.05. Functions has 0.4 points of headroom;
  - local mutants leg: 118 mutants, 2 missed, both equivalent on Windows (`file_mode -> None`, `fuzz_host_supported -> false`). The CI union is the verdict.

## Work done
- Source: 9 files modified, 6 new, plus the `fuzz/` tree. All changes are in test-only `viola-e2e`, CI and `fuzz/`; no product crate changed.
- Gates: 35 green, 2 recorded, 4 operator legs owed. Smoke ✓.
- v1-19 is implemented; the wrap flips it to verified.

## Drift resolved
- 54 amendments: arch 14, security 12, test-plan 24, obs 3, a11y 1 group of 4 sites. Leaves re-derived: stack, commands, workflow, tests-summary, `rules/testing.md`, `rules/verification-harness.md`, `rules/security.md`.
- 2 escalations, ratified by the overseer:
  - `fuzz/Cargo.lock` sits outside `cargo deny` as a test-only exemption; its audit is CARRYd to the next chunk;
  - two unscanned uploads are admissible by content: the verdict JSON (repo-relative names only) and the nightly `fuzz/artifacts/`.
- Disproved and amended:
  - the coverage JUnit is at `target/nextest/ci/junit.xml`, not `llvm-cov-target`;
  - the forward-slash ignore regex never matched on Windows;
  - the prior PREREQ's `file_mode` premise: the CI misses were the `#[cfg(not(unix))]` stub.

## Notes
- **Operator decisions this chunk:**
  - the windows mutants leg with a union verdict, now;
  - seed fuzz with `ViolaName::try_new` now;
  - the macOS-only residual goes to "Unix endpoint and home hardening" as a CARRY;
  - leans approved: rustup, not dtolnay; no `concurrency:` block (zizmor pedantic declined); the `mutants.out/` upload removed.
- **Route:** 1 PREREQ and 9 CARRYs.
  - The next entry gets the CI witness, the fuzz-lock audit, and the `cargo tree` / `cargo modules` rows.
  - Six parser entries each get a property test, fuzz target and corpus: Wrapper channel, Hooks, Readiness gate, Confirmed send, The board, Resumable SSE feed.
  - "Unix endpoint and home hardening" gets the macOS mutants leg.
- **Host changes:** installed Rust 1.96, `nightly-2026-09-20`, `llvm-tools-preview` on 1.98.1, and cargo-llvm-cov 0.9.1 (replacing 0.8.5). Created `target/msrv-probe` and `target/llvm-cov-target` (~3.3 GB).
- **Deferred learnings** (max-3 cap): `git check-ignore -q` on a not-yet-existing path reads "not ignored" for a trailing-slash dir pattern (`target/`), so a gitignore probe on an unborn directory is a false negative (confidence 0.8).
- The operator's viola-lab prototype (`viola.exe` 12172, 56736) was running; it is not this project's.
- Last failed command: none open.
