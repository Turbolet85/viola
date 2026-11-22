# Outcome — 2026-11-22-paste-write-bounded (implement run 2026-11-22T12-03-19Z-implement)

**Word:** green. Next → the OPERATOR PASS (entries 19 boot, 20 status, 21 cleanup, 22 pre-push, 23 hygiene, 24 push,
25 ci.py conclusion), then the wrap.

## Files changed (16, all in research's Files to modify; 0 new)
crates/viola-pty/src/pump.rs · crates/viola-pty/src/lib.rs · crates/viola-agent-claude/src/screen.rs ·
crates/viola-core/src/lib.rs · crates/viola-mcp/src/tools.rs · src/human/hints.rs · src/human/mirror.rs · src/run/send.rs ·
src/run/send/slot.rs · src/run/wheel.rs · src/cmd/run.rs · src/cmd/verify/typed/run.rs · src/bin/viola-fake-agent.rs ·
src/bin/viola-fake-agent/input.rs · src/bin/viola-fake-agent/test_cases_opts.rs · tests/cli_send.rs

## Gates
- entries 1–14: green (second firing; the first read 1 `red` rustfmt on a scripted edit in src/run/send.rs and 2 `red`
  clippy `manual_is_multiple_of` in a new viola-pty case, both fixed).
- the five controls: `paste-bound`, `key-behind`, `cut-no-enter`, `writer-join`, `cut-mark` — each `holds`, its own case failed
  alone (evidence/control-1.txt, evidence/control-2.txt).
- entry 15 (hostwatch): recorded — CONTENDED, neighbour escher (evidence/hostwatch-1.txt) → held; resumed on inputs#I5.
- entry 16 run --unit green · 17 run --integration green (617 passed, 0 failed) · 18 run --e2e green (336 passed, 0 failed),
  each after a hostwatch read from 12:40:10Z (QUIET but this repository's own 10 s) (evidence/layers-and-watch.txt).
- WATCH (agents-join): no red in 17 or 18 on a QUIET host — 2 green runs.
- step 20: evidence/after-the-change.md (cut 5 293 ms · next 9 ms · during-block 4 001 ms `turn-running` · hook line 4 000 ms).
- no guard entry in the block (no `gate.py guard` line to copy).
- matrix: no coverage row owned by the chunk — nothing written.

## Deviations (with justification)
- writer-join case witnesses the ended thread by its writer's drop flag (the join handle is internal); its control holds.
- the paste-bound case's 1 s upper side is its own result wait; its control reads that wait red.
- extra cases in listed files: send_after_a_cut_a_release_or_a_turn_end_leaves_the_mark, wheel_counts_each_key_read_and_no_hook_move,
  a PasteCut arm in pty_error_display_is_fixed_and_keeps_the_source; six refusal-order rows.
- a queue still full at the paste's deadline is a cut with nothing written (the enqueue was unaddressed by research/plan).
- «1 s into the block» placed by a sleep after the first send-issued record, as the plan words.
- scope record: none — gate.py scope clean, 0 recorded.

## Surfaced / stuck
- none.

## Process hygiene
| process | started by | final state |
|---|---|---|
| gate calls, cargo/nextest builds and test runs | this run | terminated (each with its call) |
| control clones (~/.cache/andromeda-control/942a5a3e, c8aa9261) | this run | terminated — `clone removed` |
| booted test wrappers + fake agents (cli_send cases) | this run (tests) | terminated — `Wrapper::kill` read each child gone |
| 7 other `viola` processes (viola-lab wrappers, the pinned product's wait/list, ~/.viola mcp) | NOT DERIVED — not this run | left running: not this run's |
Census: `gate.py census --comm viola --comm viola-fake-agent --comm cargo --comm cargo-nextest --args projects/viola/target`
→ `census: 7 running`, none under projects/viola/target.

## Decisions & corrections
- Host stop at entry 15 (CONTENDED naming escher) per inputs#I4; resumed on the operator's word inputs#I5.
- A CONTENDED reading whose only build was this repository's own gate run (10 s) read as no neighbour's load.
- Scripted (python) source edits fire no format hook → one rustfmt red; fixed by `cargo fmt --all`.
- clippy `manual_is_multiple_of` on a new test → `is_multiple_of`.
- Guard refusals re-issued once each: a cwd-moving `cd`; a `cat` heredoc append (→ the Edit tool).
- The cut-no-enter control removes both CR guards: a mid-text cut never reaches the commit decision.
