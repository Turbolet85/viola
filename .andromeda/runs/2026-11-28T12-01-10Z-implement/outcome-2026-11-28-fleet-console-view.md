# Outcome — 2026-11-28-fleet-console-view (implement run 2026-11-28T12-01-10Z-implement)

**Outcome: green + smoke** — the block green at its first firing (1 iteration), the three remove-the-guard controls
`holds`, the smoke trio (operator entries 15 · 16 · 17) green. Next → the OPERATOR PASS: entries 18 (`bash
scripts/agent-run.sh pre-push`), 19 (`gate.py hygiene`, fired as written), 20 (the push, clean tree asserted) and
21 (`ci.py conclusion --sha HEAD --wait 1800`, fired as written), then the wrap.

## Files changed
- modified: `crates/viola-core/src/obs.rs` · `crates/viola-state/src/board.rs` · `schemas/diag-line.v1.json` ·
  `src/cmd/console.rs` · `src/cmd/list.rs` · `src/console/frame.rs` · `src/console/mod.rs` · `src/human.rs` ·
  `src/human/board.rs` · `src/human/board/cells.rs` · `src/human/board/rows.rs` · `src/human/board/test_support.rs` ·
  `src/human/console.rs` · `tests/tui_console.rs`
- new: `src/console/fleet.rs` · `src/console/view.rs` · `src/human/fleet.rs`

## Gates (trail `.andromeda/runs/2026-11-28T12-01-10Z-implement/gate-2026-11-28-fleet-console-view.json`)
Summary line: `entries 21 · green 13 · red 0 · recorded 1 · timeout 0 · not-run 7` (the 7 the operator legs 15–21).
Entry 11 (the hostwatch read, `recorded`): `verdict QUIET - 5 s stalled on IO (rule: >= 8 s with io some >= 50%)`.
No `gate.py guard` entry in the plan: no guard pass line to copy.
Controls (`evidence/control-1.txt`): `control: 3 holds · 0 timeout · 0 does not hold · 0 unreproduced · clone removed`.
Smoke (`evidence/smoke-operator-entries.md`): 15 boot green · 16 status green (`state":"ready"`) · 17 cleanup green
(`processes_gone`, `endpoint_gone`).
Matrix: v2-14 is gated, so no ref was written and its live reading was not fired (inputs#I6).

## Deviations (with justification)
- Step 5 places the agents read in `src/console/fleet.rs`. It is built as a closure in `src/cmd/console.rs`
  (`agents_read`: `run::agents::read_here` then `list::observations`, now `pub(crate)`), handed to
  `console::fleet::spawn_agents`. The reason: `cmd::list` is private to `cmd`, so the console module cannot reach
  `observations`. The thread, the 5 s spacing and the channel are as the step says.
- Step 5's «a failed read hands no observations» is built as the thread handing an EMPTY list, so a status read
  `idle`/`busy` before a failure goes back to `unknown`, and the unwrapped rows go too. Keeping the last good list
  would show a stale `busy`. Pinned by `fleet_source_joins_the_newest_observations` and
  `spawn_agents_hands_each_read_and_an_empty_list_for_a_failed_one`.
- Step 6's «once a second, counted on the 100 ms poll» is built as an `Instant` checked against
  `console::READ_EVERY` (1 s) at each key and each poll timeout. The interval rides `console::Sources::read_every`,
  so the unit cases run at zero.
- A board read whose scan ERRS (an `instances/` that cannot be listed: `StateError`, not a refusal) is returned
  as an `io::Error`, so the loop returns it after the restore and the verb's error reaches the `cli` catch site
  (`error: internal error`). The plan names only the refused scan. Pinned by
  `console_run_on_a_failed_board_read_restores_then_returns_the_error`.
- Cases beyond step 10's list, each in a listed or new file: `view_cuts_every_drawn_line_to_the_columns`,
  `view_equal_when_its_lines_are`, `body_rows_are_the_rows_less_the_title_and_the_key_line`,
  `console_frame_draws_the_body_lines_from_row_two`, `console_run_on_a_failed_board_read_restores_then_returns_the_error`,
  `console_run_reads_the_board_again_only_after_its_interval`, `fleet_source_logs_no_skip_line_for_a_clean_home`,
  `fleet_source_drops_an_instance_gone_from_the_scan`, `fleet_source_of_a_widened_home_is_the_refusal`,
  `spawn_agents_hands_each_read_and_an_empty_list_for_a_failed_one`,
  `board_read_of_an_instance_with_no_log_line_has_no_last_activity`,
  `fleet_cut_past_the_width_of_a_drawn_row_keeps_the_columns`.
- Existing literals moved: `board_read_an_unreadable_or_newer_snapshot_takes_the_replay`'s expected `Item` gains
  `last_activity: Some(logged_at())` (step 1: a replayed row takes the time from its log), and the scaffold's
  `console_run_on_q_draws_the_frame_then_quits_restored` moves to the body-row form (step 10 says so).
- Every `tui_console` start, the scaffold's eight cases included, now sets `PATH` to an empty test-scoped bin dir
  (`start` → `start_on`). Without it, the console's new agents thread would read the host's real `claude`
  sessions (test-plan · decision/tests-never-read-host-sessions).
- Scope record: `src/human/board/test_support.rs` · companion · serves `crates/viola-state/src/board.rs` · self.
  `gate.py scope`: `scope: clean — changed 17 · listed 16 · recorded 1 (companion 1 · mechanical 0 · in-intent 0 ·
  widening 0) · absorbed 0 · excluded 407`.

## Surfaced / stuck
none

## Process hygiene
| process | started by | final state |
|---|---|---|
| nextest / viola / viola-fake-agent children of entries 3–14 | this run (the gate block) | terminated — gone at the census (`census: 7 running`, none from `projects/viola/target`) |
| the control clone's firings (entry 6 ×4) | this run (`gate.py control`) | terminated — `clone removed` |
| view-smoke session (supervisor, 2 wrappers, 2 fake agents) | this run (operator entry 15) | terminated — entry 17 `processes_gone: true` |
| the transcript-line monitor (`byadv9vec`) | this run | terminated — TaskStop |
| 3 host `claude` sessions, the viola-lab prototype wrapper and its claude, `~/.viola` `viola mcp`, the product's `viola wait` | NOT DERIVED — not this run's (the host's own sessions) | left running: not this run's — the operator's |

## Decisions & corrections
- Standing learning recurred (host-linux.md §Processes, «never `pgrep -f` on a pattern the invoking shell's own command
  line contains»): the transcript monitor's end test was `pgrep -f "gate.py run --chunk …"`, which its own shell
  matched, so it never ended by itself. It signalled nothing; it was stopped with TaskStop after the block's notice.
- Bash guard refusal: at Setup, `cd` into the skill's references dir was refused (`moves the session cwd`). Re-issued
  once with absolute paths.
- The relayed directive was snapped as inputs#I6 (`relay-1.md`): no dialog, v2-14 gated and unfired, console tests
  under their own pty and test home, no window, no real claude, no read of the operator's home.
