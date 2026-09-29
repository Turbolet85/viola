## 2026-09-29-t15-07-57-wrap — registry migration (U35): the obs-plan Decisions Log leaves the body
**Section:** §12 Obs Decisions Log · §3 → Logging stack (its key file) · §4 Span / Trace Coverage (Scenario "`viola run` start sequence to child spawn"; Scenario "Dialog → `answer`"; Edge flows E2) · §6 Log Coverage (Child / shell spawns)
**Change:** the log moved verbatim to obs-plan-amendments-archive.md (40 entries: the initial entry, D-01…D-36, Review 1 and overseer fix passes 2 and 3). Lifts:
- §3 → Logging stack (hand-landed in its key file after the migration): the OFF third-party targets' captured failure classes (spawn `Err` → `internal-error`, handle-wait `exit_source`, notify → `sse-closed{tail-error}`), the accepted loss, and the seam-gap path (json-subscriber 0.3.0, never `tracing-log` / `LogTracer`) (D-11, D-23, D-24).
- §4 Scenario 1: `pty_backend` is an open string and `sideload_fallback` a closed enum in `diag-line.v1.json`; a degrade stays at info, path- and hash-free (D-36).
- §4 Scenario 4: a dialog hook's `invoked_at` is its true start instant; non-dialog and pre-`dialog_id` hooks emit `hook-invoked{corr:null}` at once (D-07).
- §4 E2: the R8 strip's removed / persistent set and 11-name `IDENTITY_FLOOR`, both name lists comma-joined, no value read (D-34, superseding D-14).
- §6 Child / shell spawns: `diag-line.v1.json` `$defs.subject.enum` closes `subject`; verify's spawn exits carry `child_exit_status` + `duration_ms` (D-35).
**Why:** a Decisions Log is keyed by time — history, not current truth; its in-force items now stand in the body
**Ref:** .andromeda/runs/2026-09-29T15-07-57-wrap/
