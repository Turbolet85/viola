# Light-gate red, recorded before any re-run — wrap 2026-11-19T21-07-39Z

- Entry 26 `bash scripts/agent-run.sh run --integration`: `red · exit 0 ✗ (exit 1) · 65.47s` (the gate trail
  `gate-2026-11-19-context-as-a-local-command.json`; its log `$TMPDIR/andromeda-gate/2026-11-19-context-as-a-local-command/wrap-2026-11-19T21-07-39Z/26.log`, $TMPDIR = /tmp).
- The one failing case of 567: `viola-e2e::harness_lifecycle boot_that_never_gets_ready_times_out_and_stops_its_supervisor`,
  FAIL [14.085s]; first diagnostic line: `panicked at crates/viola-e2e/tests/harness_lifecycle.rs:343:5: the boot waited its own 2 s deadline, not the 20 s default: 13.072034752s`
  (a wall-time bound of 10 s; the code and readiness assertions above it held). The harness archive: `target/run-archive/441`.
- Every other entry of the block green (1–8, 27); the round, live and operator legs re-verified from evidence.
- The same case green in implement's entry 26 (`26.log` of the 2026-11-19T20-04-40Z-implement run), in the operator
  pass's pre-push (coverage 3702 passed 0 failed) and in CI ci#44613613553 on the same source tree (`495bb905`; no
  source changed since — the wrap writes no source).
- The host at the red (`hostwatch.py read --last 15`, 2026-11-19T21:50:14Z): `verdict CONTENDED - 145 s stalled on IO:
  21:39:46Z..21:40:56Z 70 s; 21:48:51Z..21:50:11Z 75 s (rule: >= 8 s with io some >= 50%); build tools alive in them:
  escher 145 s, viola 65 s`; escher's build peak 129 processes (rustc 32, cc 32, collect2 32, rust-lld 32); load peak
  195.7; dirty pages at the 256 MB cap. Entry 26 ran inside the second stall.
- Reading: a timing red under measured host IO contention from a neighbour's build; this chunk's diff touches no boot
  readiness path (`crates/viola-e2e/src/harness/run.rs` changed `LEDGER_ROWS` and the live-proof stdout alone).
  Not yet a cause proven: the block is re-run whole once the host reads QUIET, and the red is routed by P5's
  Recurrence-watch row whatever that re-run reads.

## The second firing (the block re-run whole after a QUIET read at 21:53:39Z)
- Entry 26 red again, the same one case: `FAIL [  15.100s] (562/567) viola-e2e::harness_lifecycle boot_that_never_gets_ready_times_out_and_stops_its_supervisor` (`26.2.log`, archive `target/run-archive/445`); every other entry green.
- The host during it (`hostwatch.py read --last 5`, 21:56:51Z): `verdict CONTENDED - 90 s stalled on IO: 21:55:11Z..21:56:51Z 90 s`; escher's next build, peak 126 processes; load peak 217.3.
- The case alone on this tree, three firings (`agent-run.sh run --integration --filter 'test(/boot_that_never_gets_ready_times_out/)'`, beside the gate, logs in the session scratchpad): FAIL 10.483 s · FAIL 14.884 s · FAIL 11.319 s, each beside a CONTENDED read (21:57:29Z, 21:57:46Z, 21:57:59Z). Load average at 21:58:14Z: 216.36, 130.19, 84.47.
- The same case green at 2.617 s in implement's entry 26 and 2.624 s in the operator pass's pre-push (`run-20261119T205327Z/28.log`), on this same source tree.
- `${XDG_RUNTIME_DIR}/viola` holds 29620 entries at 21:58Z (read, not acted on; whether that is new is unmeasured).
- Next: wait for a QUIET read with the 1-minute load under 20, then the block whole once more.

## The quiet side, and the verdict
- The host at 22:01:24Z: `QUIET - 0 s stalled on IO`, load 10.00. The case alone on this tree: `PASS [2.234s]` (22:01:35Z, load 8.61).
- The block whole, third firing: `entries 31 · green 10 · red 0 · recorded 0 · timeout 0 · not-run 21`; the case inside entry 26 `PASS [2.601s]` (`26.3.log`); the host over its run QUIET (`hostwatch.py read --last 5`, 22:04:38Z, load peak 54.3).
- Two-sided on ONE tree (no source changed): red at 10.5–15.1 s in five firings, each under a CONTENDED read naming escher's build (load 190–217); green at 2.2–2.6 s in four firings on a quiet host (implement's 26, the pass's pre-push 28, this solo, this block). The cause measured: the neighbour's build contention on the shared host, against the case's 10 s wall bound; this chunk's edits do not reach the case.
- Disposition (P7.1, «green over a red the read no longer holds»): the red stays on record here; it routes by P5's Recurrence-watch row — a `WATCH:` on the next markerless entry, «A fired scheduled wake keeps the wheel» — never closed by the green. No bound is moved (testing.md 2026-09-28).
