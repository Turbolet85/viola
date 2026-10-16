# Wrap P1 — in flight, stopped at the context alarm (85 %)

**Superseded 2026-10-16T23:05Z:** the second window finished P1 from the list below — the card's rows carried, the
new-text listing written and pasted, the site sweep fired (`report-patterns.toml`), `report.md` authored, the report
evolve checkpoint appended. The wrap continues at P2 in this run dir.

The wrap of `2026-10-16-unwrapped-sessions-on-the-board` was invoked "P1 only, then stop (the two-window form)". This
window reached the alarm before the report was authored, so P1 is NOT done: no `chunks/{marker}/report.md` exists,
no new-text listing was pasted, no site sweep was fired, and the report evolve checkpoint did not run. The next window
runs P1 from the step named below, in this run dir.

## Done here (2026-10-16T22:30Z)
- Setup: evolve-system read (byte-equal to the new-session copy, stamp `evolve v1.1`); exactly 1 pending record
  (`master-route.md:74`); no `report.md` → a fresh wrap; run dir `.andromeda/runs/2026-10-16T22-30-02-wrap/`; the
  code-graph refresh fired in the background (`python scripts/code-graph.py refresh`).
- Git-state: branch `build/viola-0.1.0`, 0 ahead of the upstream; the operator pass ran — basis = the parent of the
  oldest pre-CI commit `de9ee3f`, i.e. `92264a7f254c`; the pass's commits `de9ee3f` (chore, pre-CI), `3c1c54d` (fix),
  `020c80b` (fix); the tree carries 4 uncommitted record files (the implement run's gate trail, journal, outcome, and
  the chunk's `evidence/operator-pass.md`). `state.yaml`: last_wrap 2026-10-16T18:43:25Z, session_count 74.
- The implement run dir was re-read from disk whole (`outcome-{marker}.md`, `journal.md`): it is the outcome basis.
- The scope read: `scope: clean — changed 53 · listed 37 · recorded 16 (companion 15 · mechanical 0 · in-intent 1 ·
  widening 0) · absorbed 7 · excluded 68`, base `92264a7f`.
- `inputs.py verify`: `inputs: 2 entries — … n/a 2 · uncited 1 · unparsed 0`; I1 (the founder's answers, relayed,
  2026-10-16T19:54:09Z) cited in scope.md and plan.md; I2 (the operator, phase session, 2026-10-16T20:04:54Z) UNCITED.

## P1 still owes, in order
1. The P5 review card: read the inputs entries' `why` for a `p5-card-{k}.md` (the phase run dir
   `.andromeda/runs/2026-10-16T18-50-06-phase/` holds `p5-card-1.md`); carry its `## Beyond the plan` rows verbatim
   with their authority.
2. `cites.py added --out {run_dir}/new-text.md` (the listing is large: the chunk added ~19 500 lines — read it in parts).
3. The site sweep (`cascade.py sweep --pre-pass`, one pattern per moved fact, the title pattern "Unwrapped sessions on
   the board"); the moved facts are listed in the outcome's Counts line (spawn pairs 7→8, relied 22/19→23/20, the
   fixture set 32→33, fuzz targets 5→6, `process-exit` detail 12→13, the nextest kill line 18→19) plus the plan's
   `Expected amendments (wrap)` list (9 masters' entries) and the drift-base detectors' `check` fields.
4. Author `report.md` from the outcome (13 deviations, the scope record, the P4/P12 leg, the three pushes and the CI
   reads, the process census) — then paste the listing, then the report evolve checkpoint.

## Owed placements P1 must carry (the operator's invocation, 2026-10-16)
- **An owner for the unexplained `cli_send` red** — `viola::cli_send
  send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins`: one red (firing 5, entry 11, exit 0 where 10 is
  asserted), green in 3 other local firings, 20 two-sided reruns on this tree and on `git archive 92264a7f254c`, and
  `PASS` on all three CI runners of `ci#38971772946`. Cause not known. The operator's word: a **WATCH pin** — P1's
  Outcome names it, P5 places it (route-resolve's watch form) on the entry that next touches the send / gate path.
- **The leftover dirs for the founder's desk** (not removed; under the ignored `target/` unless said):
  `target/control-92264a7/` (the control tree and its build), `target/p4-live-20261016T211033Z/` (the leg's two
  session dirs), `target/agents-scale-214213/` (the 1 860-row timing copy), `target/e2e-home/viola-record-20261016T204612Z/`
  (the round's stamped home, on the tmpfs), and the implementing session scratchpad's `control/`, `control.tar` and
  `p4-rig/` (a recursive `rm` was refused by the host's deny rule).
- **Curation candidates** for *Decisions & corrections* — the outcome's own section, plus: a fake agent's registry
  row must replay recorded values (its real cwd leaked a CI checkout path into `--record`); a per-row sysinfo lookup
  in a listing that grows with killed processes pushed a bounded read past its deadline under coverage (one table
  read per listing); a snapshot field written after its event line raced every reader waiting on that line (write
  it before the line can be read); a verify probe placed after a chain of pastes inherited their failures (place it
  right after the turn it measures); the guard refused a `cd`, a `cat` heredoc to a file, a pipeline tool piped into
  a filter, and a recursive `rm`.
