# Curation — wrap 2026-11-15T11-38-30 of 2026-11-15-live-round-on-the-trimmed-product

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + host-linux.md: "A script that names a path under `target/e2e-home` keeps it as spelled and never resolves it" (confidence 0.9)
    Proof: report Decisions & corrections 4 — `rel()` resolved the link to `$TMPDIR/viola-e2e-home-1000/…`, `save_state` raised after entry 9's verdict and `round-state.json` was not written (implement 2026-11-15T08-33-57). Signals: measured +0.4 · «never» +0.3 · detail +0.2. Tier: the host leaf (the link is a host fact); host-linux.md loads always, so judged at Tier 1's bar: one sentence, 293 B.
  Tier 3 (.claude/docs/session-learnings.md): + "Check a Python script's syntax with `ast.parse`, never `py_compile`, inside a run or work dir" (confidence 0.8)
    Proof: report Decisions & corrections 14 — hygiene's P2 refused the bytecode (implement 2026-11-15T09-38-40) and a recursive `rm` was denied (implement 07-52-35); `ast.parse` used from 09-49-21 on. Signals: measured +0.4 · detail +0.2 = 0.6, + no-other-home +0.2 (no master, route annotation, rule or hook carries it).
  Extended: T3/session-learnings.md: "2026-10-19 — Claude Code 2.1.287's folder-trust dialog: the default pointer sits one row above the trust row" + "on 2.1.325 the options are unnumbered, the pointer on the non-trust row; a cwd whose `hasTrustDialogAccepted` reads true raises no dialog, so a start there takes no key" (confidence 0.8)
    Proof: report Decisions & corrections 11 (step 3c's numbered-row reader read `selected` null on 2.1.325; fixed in step 3f, inputs#I29) and Addendum 2 (inputs#I41: the overseer's no-start measurement of the CLI's own trust record; entry 27 `trust_record=true · keys_before_box=0`, `evidence/operator-27.txt`). Signals: measured +0.4 · detail +0.2 = 0.6, + no-other-home +0.2 (no master row states the CLI's trust record; the relay's optional row was not proposed).
  Filters: 2 dup (both recurrences → handoff) · 3 task-specific · 3 below threshold · 0 conflict · 1 deferred (→ handoff)
  No-other-home: "Check a Python script's syntax with `ast.parse`, never `py_compile`" · "2.1.325's trust dialog form and the CLI's own trust record"
  CLAUDE.md size: 133/200 · T1 3.4 KB, 0 over 600 B

## Every Decisions & corrections entry, its disposition
1. The founder's words on the round — not a learning: decisions of this chunk, recorded in the report, the inputs and the masters.
2. `date -u` recurrence ×4 — rejected, Filter 1 dedup on a DEFECT entry (CLAUDE.md USER:session-learnings «A timestamp written into any record … is read from `date -u`») → handoff `recurrence-despite-learning`.
3. Recorder scrub vs a test home named `home` — rejected, Filter 1 dedup on a DEFECT entry (`.claude/rules/testing.md` 2026-10-05 «Never give `viola verify --record` a viola home with a path component named `home`») → handoff `recurrence-despite-learning`.
4. Never resolve the `target/e2e-home` link — applied, Tier 2 host-linux.md.
5. A cursor-drawn dialog is invisible to a stripped-stream literal search — rejected, Filter 4: 0.6 (measured +0.4, detail +0.2); its home is the chunk's plan and rigs, a committed contract document this chunk wrote, so no conditional signal.
6. `input-not-ready` is not a residue — rejected, Filter 2 task-specific (one rig's witness words).
7. Live screens kept local — not curated: amended into security-plan `decision/live-session-screens-kept-local` (TX 1508).
8. A firing that raised before its `start` row spent no start — rejected, Filter 2 task-specific (this round's ledger and its no-re-fire rule).
9. Each live start makes its own fresh cwd — rejected, Filter 4: 0.6; carried by the plan's rig section.
10. Status line from the screen's status row, not an argv poll — rejected, Filter 4: 0.6; carried by the plan (CARRY 3's step).
11. 2.1.325's unnumbered trust dialog — extended, Tier 3 (the 2026-10-19 trust-dialog entry), with this window's trust-record facet.
12. Trust keys before `session-start` — not curated: amended into architecture, obs-plan and test-plan rows (TX 1507, 1509, 1510).
13. A reused name keeps the earlier start's lines — not curated: test-plan `decision/start-lines-read-from-start-offset` and obs-plan `decision/events-stream-never-rotated` (TX 1509, 1510).
14. `py_compile` writes `__pycache__` — applied, Tier 3.
15. Never chain `rec.py land` onto a `gate.py hygiene` call — passed (0.7: measured +0.4, «never» +0.3), → handoff Deferred learnings under Filter 5's cap (ranked fourth).
16. Guard refusals — not a learning: telemetry (`guard-refused`), the friction stream.
17. The fake agent keeps a typed DEL — not curated: a carry on the route (the fake agent's DEL handling carry stands, inputs#I50 item 4).
18. The real CLI erases the box on DEL — not curated: entry 12's measurement, owned with 17's carry.
19. The question dialog stays drawn after the expiry — not curated: amended into architecture `contract/dialog-hold-end/null-reply` (TX 1507).
20. A revision's invoking-line word — not a learning: a letter silence, pipeline telemetry.
21. My own slips — not learnings.
22. The rig's hold journal names entry 10's log — rejected, Filter 2 task-specific.
23. The live trail as fired — not a learning: the report's trail (now ten starts, Addendum 2).
24. The kept tmpfs home and the local frames — not a learning: the operator's desk list in the handoff.
- This window: the Addendum 2 `put` refused for a section file without its heading — not a learning: a tool refusal re-issued once, telemetry.
