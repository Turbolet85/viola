# Session Handoff

**Last Updated:** 2026-09-24T13:24:00Z
**Branch:** build/viola-0.1.0 · 0 ahead of origin/build/viola-0.1.0 as read at this wrap's Setup
**Status:** clean
**Last Commit:** 2026-09-24-observability-gates — feat: obs CI gates (lint bans + probes, G1–G4, canary secret scan before scan-gated uploads) and the 2834e4d mutants-timeout cause fix

## Position
- Done: 2026-09-24-observability-gates.
  - Lint bans: clippy denies `print_stdout`, `print_stderr` and `dbg_macro`, and `clippy.toml` bans the tracing level macros. Raw `event!` is caught by a fail-closed grep in `scripts/lint-probes.sh`. Clippy 1.98.1 cannot exempt `obs_event!`'s inner `event!`; that was measured, and obs D-33 records it.
  - CI `lint` job (3 OSes): fmt, clippy, ripgrep 15.2.0, G1 and G3.
  - CI `test` job: G2, G4 (`viola-harness schema-check`) and the canary secret scan (`viola-harness secret-scan`), with every upload gated on the scan.
  - The `2834e4d` mutants red is fixed at its cause: exit-aware readiness bounded at 10 s, and a nextest mutants kill of 10 s (30 s for `viola-e2e`).
- Next: /andromeda-phase to promote and plan "Quality gates". Its PREREQ fires first; see below.
- **CI witness owed right after this push** (overseer):
  - `check-runs` all `success` on the pushed sha;
  - the `test (ubuntu-latest)` log carries one `PASS … viola::cli_fake_agent wrapper_boot_exiting_before_ready_fails_as_exited` line;
  - the ubuntu `mutants` job kills the 3 `#[cfg(unix)]` `file_mode` mutants. Any survivor folds into "Quality gates" (its PREREQ).
- **Recorded:** local light-gate reds, ratified at the P2 escalation and skipped with reasons:
  - the `Cargo.lock`-unchanged probe, a plan-probe defect (one lock edge line; no new package; deny green);
  - `run --mutants` with 3 survivors, which the Windows host cannot kill (see the witness above).

## Work done
- 20 source files (7 new, 13 modified).
- Implement: 28 of 33 gates green, the 2 reds above, 3 operator legs owed. Smoke ✓ (impl-smoke ready, processes gone).
- Real homes: `schema-check` 26 files / 95 lines, 0 failures; `secret-scan` 34 files, 0 hits; G2 `true`.

## Drift resolved
- 23 amendments, 1 escalation (the two reds; resolved by ratified skip plus a witness PREREQ):
  - arch ×7: Stack Code quality row, 3 `target/` paths, tree, Lint, CI jobs and setup;
  - security ×2: the obs canary scan in CI, and still no repo scanner;
  - test-plan ×7: internal `schema-check` / `secret-scan`, 10 s exit-aware readiness, mutants profile and override, runner jq, pinned ripgrep, scan-gated uploads;
  - obs ×7: the raw-tracing ban split, D-33 superseding D-25's exemption clause.
- Leaves re-derived: `rules/observability.md`, `rules/verification-harness.md`, `docs/stack.md`, `docs/commands.md`.

## Notes
- Operator decisions this chunk:
  - check bodies as internal harness subcommands;
  - G2 on runner `jq` (F2 covers jq only), with a pinned, checksum-verified rg install;
  - deadlines fixed by cause and by value;
  - raw-tracing ban = level-macro path ban plus a fail-closed `event!` grep;
  - the `mutants.out/` upload is a CARRY on "Quality gates".
- Curation: T2 ×3 (`testing.md`: mutant-reachable waits must detect exit and stay below the 20 s floor; `testing.md`: keep `#[cfg(unix)]` bodies to a minimal reader; `host-win32.md`: `pwd -W` for native-tool paths).
- Deferred learnings (max-3 cap): adding an already-locked crate to a new workspace member still adds a `Cargo.lock` edge line, so a "lock unchanged" probe is red by construction; assert "no new package" instead (confidence 0.8).
- The operator's viola-lab prototype (`viola.exe` 12172, 14064) was running; it is not this project's.
- Last failed command: none open.
