@amendment marker=2026-11-06-version-done-check
title: the portable-pty pin at =0.9.0
section: §1 Obs Scope Summary (the `viola-pty` entity); §4 Span / Trace Coverage (span-trace-coverage.md: the cli `viola run` row); the logging-stack contract (the dropped-`log`-records bullet)
change:
  - The `viola-pty` entity reads «PTY seam (spawn, read, write, resize, wait, kill) over portable-pty `=0.9.0`» (was `=0.8.1`).
  - The cli (`viola run`) row's library cell reads «none (portable-pty `=0.9.0` logs through `log`; its target is OFF and failures are captured at the seam, D-11)» (was portable-pty 0.8.1), the rest of the row unchanged.
  - The logging-stack bullet reads «`log` records from portable-pty `=0.9.0` are dropped» (was portable-pty 0.8.1), the rest of the bullet unchanged.
why: the chunk moved `portable-pty` `=0.8.1` → `=0.9.0` in the workspace dependencies, `crates/viola-pty/src/lib.rs` compiling unchanged and `pty_backend()` still `openpty`, and retired the `=0.8.1` pin with its Windows rationale; only the version moves, the report not saying whether 0.9.0 emits through `log` differently.
ref: .andromeda/runs/2026-11-06T20-57-05-wrap/
@amendment marker=2026-11-06-version-done-check
title: the thiserror derive census on the product sources
section: §7 Error Capture & Reporting (error-capture-reporting.md: the `thiserror::Error` derive census)
change:
  - The census reads 9 derives «as re-measured at chunk 2026-11-06-version-done-check, a grep of `thiserror::Error` over the product sources, `src` and `crates` outside `crates/viola-e2e`», the one derive there test-only (was «as measured at chunk 2026-10-31-sanitised-error-surfaces, a grep of `thiserror::Error` over `src` and `crates`»); the count stays 9, the nine types and `AgentError`'s `ProgramNotFound` unchanged.
why: the grep over `src` and `crates` reads 10 rows, 9 outside `crates/viola-e2e`, the 10th test-only; the operator directive at this wrap orders the rule re-worded to the product sources with the count staying 9.
ref: .andromeda/runs/2026-11-06T20-57-05-wrap/
@amendment marker=2026-11-06-version-done-check
title: process-start's fields on Linux only
section: §6 Log Coverage (log-coverage.md: the `process-start` row)
change:
  - The `process-start` row carries the header's 2 cells, its `subject` values' pipes escaped (F434 whole-fact edit).
  - `pty_backend` reads (`openpty`) alone (was `conpty-sideload|conpty|openpty`).
  - The `sideload_fallback` field and its Windows degrade clause (`hash-mismatch|unreadable|load-failed|not-built`, `strict-modes-failed`, D-36) are gone; every other field unchanged.
why: `pty_backend()` stays `openpty` on `=0.9.0`, with no ConPTY or sideload path on Linux; the operator directive at this wrap orders the F434 whole-fact edit at the header's 2 cells.
ref: .andromeda/runs/2026-11-06T20-57-05-wrap/
