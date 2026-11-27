# Outcome — 2026-11-27-epoch-5-cleanup (run 2026-11-27T20-43-40Z-implement, written 2026-11-27T21:32:28Z)

**green** — the whole block green at its first firing (1 iteration); smoke trio left to the operator pass. Next → the
OPERATOR PASS: entries 27 (`bash scripts/agent-run.sh boot --session cleanup-smoke`), 28 (`… status --session
cleanup-smoke`), 29 (`… cleanup --session cleanup-smoke`), 30 (`bash scripts/agent-run.sh pre-push`), 31 (`gate.py
hygiene`), 32 (`git diff --quiet && git diff --cached --quiet && git push`), 33 (`ci.py conclusion --sha HEAD --wait
1800`); then the wrap.

## Files changed (13, all in research's lists)
`.config/nextest.toml` · `tests/contract_lints.rs` · `crates/viola-pty/src/pump.rs` · `crates/viola-state/src/liveness.rs`
· `crates/viola-state/src/board.rs` · `crates/viola-state/src/replay.rs` · `scripts/r2-open.sh` · `src/cmd/run.rs` ·
`src/cmd/revive.rs` · `src/cmd/adopt.rs` · `src/cmd/open.rs` · `src/cmd/up.rs` · `tests/contract_error_bodies.rs`.

## The mutation addendum (step 8, once, on the overseer's «go», inputs#I5)
372 in-diff mutants of `viola`, 2026-11-27T20:50:04Z–21:26:08Z, exit 0: baseline Success, 326 caught · 0 missed · 0
timeout · 46 unviable, score 100.0, no survivor. Landed in this chunk only: `evidence/c-mutation-viola.late.json` and
`evidence/addendum.md`, amending the code audit's ledger record (tx 5116, `target_sha` a80df0e5…), the sealed audit run
untouched, no ledger record edited.

## Deviations (with justification)
- Plan step 10 names four existing callers of the `seamed` test helper; the module holds three (`respawn_swaps_…`,
  `respawn_of_a_child_ignoring_the_signal_…`, `respawn_whose_spawn_fails_…`). All three pass `DRAIN_WITHIN`, bodies and
  names unchanged.
- `seamed` is split into `seamed` (its callers' form, plus the drain bound) over a new `pumped` helper taking a `Seamed`
  rig (next child, clock step, asked-at, host output, drain bound), so the new cases can pass their own host output;
  `seamed`'s existing callers are otherwise untouched.
- Steps 10-19 were written while step 8's run was still grading, after reading that cargo-mutants had copied its tree
  (the copy's `pump.rs` byte-equal to the tree's, its `nextest.toml` carrying steps 1-2); nothing was built until the
  run ended. The run's copy and in-diff sites were therefore the plan's (the Constraints entry «Running the addendum after
  steps 18-19» is not crossed: the run measured the pre-18-19 sources).
- `r2-open.sh`'s precondition sets `cleaned=1` before it exits: the script's EXIT trap would otherwise run `cleanup`,
  which calls `hyprctl -j clients` and closes `child` windows, i.e. touch the compositor on a refusal meant to start
  nothing.
- P3 smoke: the trio (27-29, `leg = 'operator'`) was not fired here — the permission classifier refused entry 27's
  `--operator` firing with a verdict ([Destructive or Unwanted Actions]) under the overseer's stop before the operator
  pass; it goes to the operator pass with 30-33.
- scope record: none — `gate.py scope` clean, 0 recorded (changed 13 · listed 13).

## Symbols / APIs
`viola_pty::pump`: private `Seams` gains `drain: Duration`; private `drain(rx, generation, within)` takes the bound;
`pump_with_paste` passes `DRAIN_WITHIN` (product behaviour unchanged). `viola_state::liveness`: private `walk(pid, max,
parent_of)`; `ancestors(pid, max)` keeps its signature and callers (`src/cmd/focus.rs`, `src/cmd/open.rs`).
`src/cmd/run.rs`: `pub(super) trusted`, `pub(super) struct Prologue { exe, persistent }`, `pub(super) fn
start_prologue(home, name, json, strict_modes)`; `revive`'s and `adopt`'s `started` call it, `revive`'s `--list` arm and
its unit cases import `trusted`. `src/cmd/open.rs` `refused` becomes `pub(super)`, `up.rs`'s copy removed and imported.

## Schema / config
`.config/nextest.toml`: the verify class's filter line (both profiles, one line) gains `test(/^path4_a_stamp_failing_the_dialog_rows_leaves_dialogs_to_the_human$/)`
and `(binary(cli_version_gate) & test(/^run_(…five names…)/))`; kills unchanged (class 20 s × 4, default 5 s × 3).

## Guard entries
none (the block holds no `gate.py guard` entry).

## Gates (run 2026-11-27T20-43-40Z-implement, firing 1 of the block)
1-25 green but 22 `recorded` (the host read, CONTENDED by this run's own mutation run). Controls: `control: 12 holds · 0
timeout · 0 does not hold · 0 unreproduced · clone removed`. Planted-hang pair: TIMEOUT 80.005 s under `mutants`, PASS
96.452 s default. Entry 26 (operator, fired once after step 8): green, `Baseline Success 372`.

## Process hygiene
| process | started by | final state |
|---|---|---|
| cargo-mutants (step 8) and its nextest children | this run | terminated (exit 0, 21:26:08Z) |
| planted-hang clone's cargo nextest | this run | terminated |
| gate block / control firings | this run (the tool) | terminated (no survivor line; census below) |
| foot / viola rows of the census (overseer, prototype, product sessions) | NOT DERIVED — not this run's | left running: the host's own sessions — their owners |

`gate.py census --comm cargo-mutants --comm cargo-nextest --comm viola --comm viola-fake-agen --comm foot --args
projects/viola/target --args viola-mutants-scratch`: `census: 8 running`, none cargo-mutants, cargo-nextest or a
`projects/viola/target` binary.

Scratch left (named, removed by no recursive delete): `~/dev/projects/viola-mutants-scratch/m6/` (cargo-mutants' copy
dir), `~/dev/projects/viola-mutants-scratch/m6-out/` (epoch.diff, viola.log, mutants-viola/), the planted-hang clone
`$TMPDIR/{scratchpad}/hang-clone`, the r2 entries' `mktemp -d` dirs under `/tmp`.

## Hygiene and the land
`gate.py hygiene --run-dir … --marker 2026-11-27-epoch-5-cleanup`: `hygiene: refused 1 files — P1 1 · P2 0 · P3 0 · read
240` — the one row `P1 .andromeda/runs/2026-11-27T19-38-11Z-phase/controls-p5.md:19 ×1 · tmp` (the phase run's own
record of a `find` probe naming the session scratch's temp-dir path). Held for the overseer's word, which came
(`inputs#I6`): «in controls-p5.md:19 rewrite ONLY the host path token to <session scratch>, nothing else of the line».
Rewritten: that one token → `<session scratch>`, the rest of line 19 byte for byte. Then the chunk's files landed.

## Decisions & corrections
- The overseer's word at the step 7 stop (inputs#I5, relayed verbatim): «go — fire step 8 once … The addendum lands in
  THIS chunk only … the sealed audit run is not opened or rewritten, and no ledger record is edited. After the run and
  its tally go straight on with steps 10–22; stop before the operator pass.»
- Guard / classifier refusal: `gate.py run … --operator 27` refused by the permission classifier, [Destructive or
  Unwanted Actions]; not routed around.
- Recurrence of a standing learning: host-linux «Never build … through shell quoting» — a stray `cat > /dev/null`
  ahead of a python heredoc blocked on stdin to the 120 s bound; nothing written, redone with the Edit tool.
