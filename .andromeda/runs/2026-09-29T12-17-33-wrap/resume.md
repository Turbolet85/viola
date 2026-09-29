# Resume point — wrap of 2026-09-29-sideloaded-conpty

**Stopped after P1 on the operator's word** (context at 63 %, over the 60 % line): "run P1 only, write the run dir
resume point naming P2 next, and stop."

**Next: P2 — Reconcile docs** (fan-out → validate → apply → cascade). No `fanout-results.md` exists in this run dir,
so P2 fans out fresh (Setup 2a: `report.md` exists → resume here, not a fresh run dir; no spec master or sidecar is
changed in the tree).

## Done
- Setup: exactly one master `pending` record — `2026-09-29-sideloaded-conpty`; `chunk_dir` =
  `viola-0.1.0/chunks/2026-09-29-sideloaded-conpty/`; no prior `report.md`. The operator pass ran: the oldest pre-CI
  commit is `2d83718`, its parent `fb78ddc` is the basis; the pass's commits `2d83718` · `224efc4` · `8f643f2`.
  Branch `build/viola-0.1.0`, 0 ahead of its upstream at this read. Uncommitted at Setup:
  `.andromeda/runs/2026-09-29T08-52-03-implement/gate-2026-09-29-sideloaded-conpty.json`,
  `viola-0.1.0/chunks/2026-09-29-sideloaded-conpty/evidence/operator-pass.md` (both ride the wrap commit).
- Code-graph refresh (Setup step 7, background): done — rust 2716 nodes / 12126 edges (17 s) · ts 7 / 1 (1 s);
  `.andromeda/cache/.refresh-done` present. P4 records it; P7 stamps `tree.db.commit` after the commit.
- P1: `gate.py scope` → `scope: clean — changed 27 · listed 14 · recorded 13 (companion 3 · mechanical 0 · in-intent 10 ·
  widening 0) · absorbed 0 · excluded 47`. `report.md` authored (every Expected amendment carried, its sites counted per
  master). Evolve `report` checkpoint appended (friction-log lines 706-707).

## Carry into the next phases
- **CI:** the final HEAD `8f643f2` — ci#36566391084, 15/15 green (the overseer verified it). **H2 acceptance:** sideload
  0/200 vs inbox 14/200 (ci#36563868040, `evidence/h2-with-without.md`).
- **P2 escalation candidates already visible in the report:** (a) the DA1 stall — a product finding, owner the route
  entry that first runs viola headless (P5 pin); (b) the test-home convention vs seeded homes (test-plan §Test data) —
  needs a carve-out or a ruling; (c) the first-start cost note (plan implementation note disproved locally).
- **P5 pins owed:** the owner of entries 6 and 18's `red — not this chunk's` (a CARRY or a minted prior-debt entry or a
  `residuals.md` line — the host's integration reds shared with `fb78ddc`); the DA1 finding's owner entry.
- **P3 curation is session-local:** the implement session's corrections live in its conversation, which a new window
  will not hold; after a `/clear` P3 reads the report's *Decisions & corrections* alone — say so at the resume offer.
- **P7 light gate:** entries `bash scripts/agent-run.sh run` and `… pre-push` will re-red on this host (the shared
  host red); they pass the ASSERT only with the basis and the P5 owner both named. The `h2-measure` clippy entry is
  valid only before the removal commit: re-verified from `evidence/h2-measure-clippy.md`, never re-run.
