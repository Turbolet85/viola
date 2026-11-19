# Curation — 2026-11-19-a-live-stopfailure-payload (wrap 2026-11-19T17-39-37Z)

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + verification-harness.md: "pre-push's env -i leaves XDG_RUNTIME_DIR unset, viola's socket under the temp root: a test across a private temp root probes and skips by a printed reason"
                                              + verification-harness.md: "Sweep hazard: snapshot.json keeps pid and child_pid under data"
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 2 dup · 3 task-specific · 0 conflict · 0 deferred (1 recurrence → handoff)
  CLAUDE.md size: 135/200 · T1 4.2 KB, 0 over 600 B

## Applied
- verification-harness.md Session Additions, 2026-11-19 (pre-push and the runtime dir).
  Proof: the operator pass's pre-push firing 1 red, `rig_rehearsal_sf_contained_reads_its_own_network_namespace`
  `send-1-reply: instance-unreachable:instance-dead`, firing 2 green after the `no-runtime-dir` probe
  (`evidence/operator-19.txt`; report, Decisions & corrections).
- verification-harness.md Session Additions, 2026-11-19 (the snapshot sweep hazard).
  Proof: the first `sf` rehearsal read `net-ns: unread` on a top-level read of the snapshot (report, Decisions &
  corrections, «Sweep hazard»).

## Every Decisions & corrections entry, dispositioned
Implement's:
- The implement directive's cap (one `SF`, one spare, hostwatch by hand, `--entry 13`) — not a learning: this chunk's cap.
- Host hazard: a scripted edit fires no format hook (entry 1 red) — rejected (Filter 1, duplicate of implement SKILL.md
  P1's rule, a DEFECT record) → handoff `recurrence-despite-learning: implement SKILL.md P1 (scripted edit, cargo fmt)`.
- The pre-push red and its mend — applied (Tier 2, above).
- Sweep hazard, the snapshot's `pid` / `child_pid` under `data` — applied (Tier 2, above).
- The user-config witness `changed-by-others` — rejected (Filter 1, duplicate of host-linux.md 2026-11-17).
After implement's:
- The wrap directive (inputs#I6): the operator away, forks to the overseer — not a learning: this wrap's routing.
- The answer card's rows — not a learning: the founder's rulings, recorded in the sidecars.
- «Leaves hold no figure the code states» — rejected (Filter 1, duplicate of the curation guide's own rule).
- The operator pass: no correction — nothing to weigh.
This wrap's conversation:
- The overseer's wording condition on the three architecture excepts (inputs#I7) — rejected (Filter 2, task-specific:
  three cells' words), carried by the sidecar entries.
- The settlement in the fit's form (inputs#I7) — rejected (Filter 2, task-specific), carried by the sidecar entries.
- A row cell refusing a date (rows-check `history`) — not a learning of this project: the pipeline's own rule, said
  by the tool.
