# Wrap resume point — 2026-09-28-capability-ledger-and-viola-verify

**Stopped after P1 on the operator's word** (context at 67 %, over the 60 % line): "run P1 only, write the run dir resume
point naming P2 next, and stop. CI is verified by the overseer: run 36460408121 on 6486276, 18/18 green. The route
directives come with the resume."

**Next: P2 — Reconcile docs** (fan-out of the seven doc-agents against `chunks/{marker}/report.md`). No fan-out ran, no
`fanout-results.md` exists, no spec master or sidecar was touched: Setup 2a's resume reuses the report as is and P2 fans
out from scratch. P3–P7 follow in order.

## Done in this run
- Setup: chunk `2026-09-28-capability-ledger-and-viola-verify` (the one master `pending` record, master-route line 31);
  no prior `report.md`; run dir `.andromeda/runs/2026-09-28T18-10-28-wrap/` (reuse it); tools dir
  the skill's `andromeda-tools/scripts` (resolved from the skill base dir at invocation).
- Setup 4 (git state, read at 18:10Z): branch `build/viola-0.1.0`, 0 ahead of its upstream; the operator pass ran —
  oldest pre-CI commit `01f22aa`, basis parent `9b4f6f4`; the pass's commits `01f22aa`, `65dd401`, `1027f87`, `8cc9f14`,
  `a5f7a67`, `6486276`; tree dirty only with the implement run dir's gate trail, `evidence/operator-pass.md` (entry 34),
  and this run's files.
- Setup 5 intent reference: working-route :53 entry + `plan.md` §Acceptance Criteria (12) and §Implementation notes'
  `Expected amendments (wrap)` (13 + the matrix line) — all dispositioned in the report.
- Setup 7: code-graph refresh fired and finished — `.andromeda/cache/.refresh-done` present; rust 2631 nodes / 11760
  edges (17 s), ts 7 nodes / 1 edge (1 s). P4 reads it; P7 stamps `tree.db.commit` after the commit.
- P1: `gate.py scope` → `scope: clean — changed 30 · listed 23 · recorded 7 (companion 3 · mechanical 1 · in-intent 3 ·
  widening 0) · absorbed 4 · excluded 41` (trail in this run dir); `report.md` authored; the P1 evolve checkpoint appended
  (friction-log ids `2026-09-28T18:13:40Z-a`, `-b`).

## Carried for the later phases (from the report — read it, this is only a pointer list)
- P2: the overseer-directed obs-plan §7 amendment (panic backtrace = raw frames, reason 351 of 403 ms), beside the plan's
  13 expected amendments; three `Spec claims disproved` entries to disposition (obs-plan §7 force_capture · plan step 8
  `#[files]` · obs-plan §4 `verify` stderr summary).
- P5: route directives come with the resume (the overseer). Known inputs: the tail entry "Verify-stamped test homes and
  harness" after :53; the plan's route CARRYs (:58, :60, :62, :68, :95, Epoch 4/5 owners; CARRY 4 WSL root-install moves
  on; CARRY 5 E1 applied); the head-of-queue chunk the :53 wrap inserts per the founder's 17:59 ruling (mutation testing
  leaves chunks and CI for the epoch-boundary audit), which takes the CARRY `evidence/macos-mutants-baseline-carry.md`;
  H2 (`evidence/h2-ci-red.md`) is the founder's, not folded; the founder ratification of 2026-09-28 12:39:40 (stamps read
  without strict-modes until :95) is recorded at wrap, not a halt.
- P7.1: operator entries re-verify from `evidence/operator-pass.md` (entries 28–34, CI ci#36460408121 green on
  `6486276`, verified by the overseer); P7.3: this chunk claimed 0 matrix capabilities (gate no-op expected).
