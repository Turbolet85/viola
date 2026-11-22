# The light gate's red the read no longer holds — recorded before any re-run

Gate: `gate.py run --chunk 2026-11-21-key-after-a-resize --run-dir .andromeda/runs/2026-11-22T09-39-36Z-wrap`
(trail `gate-2026-11-21-key-after-a-resize.json`; logs `$TMPDIR/andromeda-gate/2026-11-21-key-after-a-resize/wrap-2026-11-22T09-39-36Z/`).

- Entry 5, `bash scripts/agent-run.sh run --integration --filter 'binary(rig_rehearsal)'` — `red · exit 0 ✗ (exit 1)`,
  30.71 s: `48 tests run: 41 passed, 7 failed` — `rig_rehearsal_cs_keeps_the_stamp_and_reads_the_config_top_level`,
  `rig_rehearsal_v_keeps_a_failed_runs_first_screen_locally`, `rig_rehearsal_ts_verifies_then_runs_and_reads_the_written_record`,
  `rig_rehearsal_v_reads_the_trust_record_row_and_no_record_left`, `rig_rehearsal_vh_verifies_the_given_home_and_reads_names_only`,
  `rig_rehearsal_cc_confirms_context_under_the_kept_stamp`, `rig_rehearsal_r3w_reads_the_wheel_kept_after_a_fired_wake`
  — every one a rehearsal that runs `viola verify` over the fake agent; first diagnostic line (the CS case):
  `assertion \`left == right\` failed: label: rehearse-CS` with `verify-exit: 1 · pass-count: 28/29 · failing: agents-join`;
  the four cases that print verify's rows all read `pass-count: 28/29 · failing: agents-join`.
- Entry 23, `bash scripts/agent-run.sh run --integration` (the whole layer, these seven among it), the identical tree,
  10:15:08Z–10:16:27Z — green.
- The host record for the gate (`hostwatch.py read --trail` this run's gate trail): `CONTENDED for viola — 118 s
  stalled on IO`, escher's `cargo-mutants` tier alive throughout; entry 5 `stalled 26 s` of its 31 s.

Cause: NOT measured. A reading only: verify's one bounded agents read inside Run B missed its bound while the host
stalled on IO. No retry rule, no loosened entry. Routed by P5's Recurrence-watch row (a `WATCH:` on «Paste write
bounded»), the wrap returned to P5 before the flip.
