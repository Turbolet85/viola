# WATCH — `viola::cli_send` `send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins`

The folded `watch:` bullet (plan §Implementation notes): every run of `cli_send` this chunk makes is recorded here,
red or green, never re-run for green. `run --unit` does not run it; `pre-push`'s coverage run does.

| # | run | reading | first diagnostic line |
|---|---|---|---|
| 1 | /implement P2 block run 1, entry 19 (`bash scripts/agent-run.sh pre-push`), 2026-10-17T01:33:37Z start, log `$TMPDIR/andromeda-gate/2026-10-17-end-to-end-path-binary-and-harness-legs/implement-2026-10-17T01-21-28/19.log` L424 | green — `PASS [   8.067s] ( 397/2666) viola::cli_send send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins` | — |
| 2 | /implement P2 block run 2, entry 19 (`bash scripts/agent-run.sh pre-push`), started 2026-10-17T01:39:14Z, log `…/implement-2026-10-17T01-21-28/19.2.log` L423 | green — `PASS [   8.098s] ( 396/2666) viola::cli_send send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins` | — |
| 3 | operator pass, entry 19 (`bash scripts/agent-run.sh pre-push`) on the final uncommitted tree, started 2026-10-17T01:44:14Z, log `…/implement-2026-10-17T01-21-28/19.3.log` L424 | green — `PASS [   8.082s] ( 397/2666) viola::cli_send send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins` | — |
