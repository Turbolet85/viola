# Session Handoff

**Last Updated:** 2026-09-24T11:46:30Z
**Branch:** build/viola-0.1.0 · 0 ahead of origin/build/viola-0.1.0 as read at this wrap's Setup
**Status:** clean
**Last Commit:** 2026-09-24-log-redaction-and-never-log-floor — feat: catch-site chain to the instance detail file, CLAUDE* canary floor, Unix mutant kill for read_diagnostics_level

## Position
- Done: 2026-09-24-log-redaction-and-never-log-floor.
  - A dispatch error now leaves `process-exit{internal-error}` in the role file and its `chain` only in `instances/<name>/diagnostics/detail-run.ndjson`. `run` stays silent on the terminal. The carrier is `cmd::Failure` + `obs::DetailSink`, written by `obs::report_internal_error`.
  - The NEVER-log floor at HEAD sinks is proven: three planted `CLAUDE_*` canaries appear in no home file and on neither stream (clean run, error path, debug level).
  - The CI red on `809456e` is folded in: the `#[cfg(unix)]` `read_diagnostics_level_file_as_home_is_unreadable` kills `src/obs.rs:193:19`.
  - Deferred for zero sites at HEAD, pinned as CARRYs: veil / skip-all / `ChannelError` → "Wrapper channel"; `drift_report` + hook exit 0 → "Hooks to normalised events"; the `cli` `error: internal error` line → "CLI output tokens".
- Next: /andromeda-phase to promote and plan "Observability gates".
- **CI witness owed (plan operator entries):** on this wrap's pushed sha:
  - read `check-runs` and expect `success` on every check, mutants included;
  - read the `test (ubuntu-latest)` and `test (macos-latest)` job logs, each carrying one `PASS … viola::bin/viola obs::tests::read_diagnostics_level_file_as_home_is_unreadable` line. This is the Linux proof of the mutant kill, per the operator's P4 decision.
- **Recorded:** CI for `809456e` (the previous chunk). Run `35990393334` concluded with 7 checks `success` and `mutants` `failure` (1 missed at `src/obs.rs:193:19`). That red is owned and fixed by this chunk.

## Work done
- 5 source/test files changed (+273 / −12), no new files, no dependency change.
- 17 local gates green on the first run: local mutants counted 5/5 caught; unit 136, integration 84. Smoke ✓ (p3-smoke booted ready, cleanup exact).

## Drift resolved
- 3 amendments, 0 escalations:
  - arch §Stack Error types row + [Error Handling]: anyhow stays in the bin, now named as `main`, dispatch and the catch-site reporter; chains are recorded only in the instance detail file.
  - obs §7 Platform pick restated the same scope.
- obs :54 (§1, the verbatim obs-scope copy) is kept by rule (obs :485).
- Leaves re-derived: `docs/stack.md`, `docs/conventions.md`, `docs/services/viola.md`.

## Notes
- Operator decisions this chunk:
  - Fold the CI mutants red into this chunk, through the loop.
  - Witness the Linux kill by the ubuntu/macOS test job logs: no Docker run, no edit made only to steer the diff.
- Curation: T2 ×2 (`testing.md`: an I/O-error test must fail at the same call on every OS; `verification-harness.md`: `--in-diff` never regenerates an earlier chunk's missed mutant).
- The operator's viola-lab prototype (`viola.exe` 5188, 12172) was running; it is not this project's.
- Last failed command: none open.
