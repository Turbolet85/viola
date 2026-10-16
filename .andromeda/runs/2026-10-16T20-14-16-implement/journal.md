# Implement journal — 2026-10-16-unwrapped-sessions-on-the-board

Live `claude` starts: the founder's cap for this chunk is 8 (the record round 6, the attended P4 / P12 leg 2), each
ledgered with `date -u` before it is made (`live-starts.md` in this run dir). A red round is reported, never re-fired.

- 2026-10-16T20:46Z · P2 firing 1: `gate.py run --only 1,2,3,4,5,6` (fmt, clippy, G1, lint probes, the unit suite, the
  new unit cases by name), backgrounded; no live start. Read: green 6 / red 0.
- 2026-10-16T20:46Z · P2 firing 2: the live round, `gate.py run --live-legs` (entries 7 probe, 8 record, 9 census),
  backgrounded; 6 live starts, ledgered in `live-starts.md` before it. Read: `round: COMPLETE · legs fired 1/1`,
  `stamped 2.1.287  23 pass  0 fail`; evidence `round-204738Z.txt`; census after it: no `claude` in a probe dir.
- 2026-10-16T20:48Z · P2 firing 3: the whole block, plain `gate.py run` (legs not fired), backgrounded; `pre-push`
  declares 7 200 s. No live start. Read: green 17 · red 1 (21, `pre-push`: one test of 2 645,
  `cli_answer path4_a_stamp_failing_the_dialog_rows_leaves_dialogs_to_the_human` — `tests/support/home.rs`
  `dialogless_home` pinned `18 pass  4 fail`; the run read `19 pass  4 fail`, the agents row passing). Fixed (19 / 16).
- 2026-10-16T20:53Z · P2 firing 4: `--only 21` (the failing entry), backgrounded. Read: green (86.5 s).
- 2026-10-16T20:55Z · P2 firing 5: the whole block again, plain `gate.py run`, backgrounded. Read: green 17 · red 1
  (11, integration: `cli_send send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins`, exit 0 where 10 is
  asserted; green in firing 3's entry 11 and in `pre-push` 21.2 and 21.3, 21.3 later in this same firing).
  Host record over the red window (`hostwatch.py read`): load mean 40.3, cpu some mean 33 %, verdict QUIET; the two
  green windows read load 37.1 and 41.2. Open: cause not known.
- 2026-10-16T20:57Z · two-sided standalone, the case alone through the harness: this tree 5/5 green
  (`scratchpad/subject.txt`); HEAD's tree (`git archive HEAD` into `target/control-92264a7/`) 5/5 green
  (`scratchpad/control.txt`).
- 2026-10-16T21:00Z · `binary(cli_send)` whole, alternating rounds subject / control ×3, backgrounded. Read: 6/6
  green, no failure (`scratchpad/binpairs.txt`).
- 2026-10-16T21:02Z · the integration entry's filter, alternating subject (11 binaries) / control (10, no
  `cross_list_rows` there) ×2. Read: 4/4 green (`scratchpad/entrypairs.txt`). The red reproduced on neither tree in
  20 runs after it; its cause stays not known and it stays open in the report.
- 2026-10-16T21:06Z · `--only 11`: green (11.3.log, 54.32 s).
- 2026-10-16T21:07Z · the Windows target's clippy, check only, `CARGO_TARGET_DIR=target/wincheck`, backgrounded.
