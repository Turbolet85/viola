# Outcome — 2026-11-23-model-and-effort-at-launch (implement run 2026-11-23T10-01-45Z-implement)

**Outcome: green + smoke.** Next → the operator pass's remaining entries 19–22 (pre-push, hygiene, the push of the
operator pre-CI commit, the `ci.py conclusion` read), in this window per `inputs#I5` and `inputs#I7`, then the wrap
in a fresh window. This file supersedes this run's earlier `held` write of the same name.

## Files changed
37 (`gate.py scope`: `scope: clean — changed 37 · listed 26 · recorded 11 (companion 11 · …)`), one new:
`src/run/launch.rs`.

## Gates
- `cargo fmt --all --check` green · `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings` green.
- `cargo nextest run -p viola-core` / `-p viola-agent-claude` / `-p viola-state` / `-p viola --features fake-agent --bins` green.
- The three filtered new-case entries green, non-vacuous: ctl-state 8 tests, ctl-root 35, ctl-revive 1.
- The integration binaries entry: red at its first firing (3 of 206 failed: `e5_list_reads_a_live_wrapper_through_the_replay`
  ×2, `tests/cli_list.rs:235`; `hook_prompts_arrive_normalised_with_their_origin`, `tests/hook_events.rs:695`),
  recorded before the re-fire; fixed in the two listed files; green at `9.2.log` (206 run).
- The harness boot read entry green.
- The host probe: `recorded` (its plan form `--last 15` reads 10:07:02Z–10:22:02Z, CONTENDED, before the quiet window);
  the directive's read `hostwatch.py read --from 2026-11-23T10:21:18Z --to 10:21:45Z`: `QUIET - 0 s stalled on IO`.
- `bash scripts/agent-run.sh run --unit` green (3149 passed) · `--integration` green (635) · `--e2e` green (343).
- Controls: `control: 3 holds · 0 timeout · 0 does not hold · 0 unreproduced · clone removed` (`evidence/remove-the-guard.md`).
- Smoke (operator entries, fired by this run): boot green · status green (`ready`, each item `model`/`effort`, no
  parent, no role) · cleanup green (`evidence/smoke-operator.md`).

## Deviations
- Step 23 / the (state) criterion's revive clause: a DEVIATION on the overseer's word (`inputs#I7` item 2). Revive
  refuses `cwd-missing` over any replayed snapshot, so no child exists to hand the replayed pair to. Proved instead:
  `e5_the_replay_reads_the_launch_pair_the_wrapper_wrote`. No test under the planned name.
- `model_word_takes_the_closed_shape` is a literal table, not rstest (viola-core takes no rstest).
- Readers the plan did not name, moved (listed in the report's Deviations).
- The scope record's 11 companion lines.
- Two lines of the phase run's review cards were re-worded path-free for hygiene, on the overseer's word
  (`inputs#I7` item 3).

## Process hygiene
| process | started by | final state |
|---|---|---|
| gate block, its nextest children, the layers' harness runs | this run | terminated (each call exited) |
| control clone `~/.cache/andromeda-control/977d08bb` | this run | terminated, clone removed |
| smoke session `launch-smoke` (supervisor, 2 wrappers, 2 fake agents) | this run (operator entry 16) | terminated (entry 18: `processes_gone`, `endpoint_gone`) |
| 7 `viola` processes the census lists (the bridge's `mcp`, viola-lab's wrappers, the product's `wait`) | NOT DERIVED: none is this repository's build | left running — not this run's |

## Decisions & corrections
- The overseer's word (`inputs#I7`): the host quiet from 10:21:18Z; step 23 a deviation; the two card lines re-worded.
- Guard refusals: 3, each re-issued once.
- Recurrence of testing.md 2026-10-04 (ext. 2026-11-14): two counted-literal readers outside the plan's sweep.
