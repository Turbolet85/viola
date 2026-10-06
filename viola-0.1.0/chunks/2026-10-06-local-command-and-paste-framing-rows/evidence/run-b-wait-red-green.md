# Step 12 — the wait after an added turn, red before green

Both readings are the same command, run from the repository root on the Linux dev host:

```
bash scripts/agent-run.sh run --integration --filter 'test(/verify_window_paste_hint_past_the_gate_maximum_still_stamps/)'
```

The case (`tests/cli_verify.rs`) runs `viola verify` against the fake agent over the synthetic 2.1.0 set with
`--paste-hint-ms 6000`: after the long text's replay and its Stop the agent clears the screen, and draws the
`turn` screen 6 s later. The window is forced open and never sampled (`.claude/rules/testing.md`, 2026-09-27).
The fake agent's option and the case were written first; `src/cmd/verify/typed.rs` was then as the first
implement run left it.

## Red — the case written first, `framing_turns` untouched

Started 2026-10-06T21:09:51Z. Exit 1, `"ok":false`, `"suite":"nextest-integration","passed":0,"failed":1`.
nextest: `FAIL [   8.796s]`, `1 test run: 0 passed, 1 failed, 325 skipped`.

- The assertion that failed is the first, the exit code: left `Some(1)`, right `Some(0)`.
- verify's stdout, from the failure message: lines 1-15 read `pass`; `[16/17] tag-escaping … fail`;
  `[17/17] local-command-clear … fail`; the last line is `stamped 2.1.0  15 pass  2 fail`.
- What the trusted run typed, read from the fake agent's receipt of a second identical run with the failing
  test's home kept (`AGENT_RUN_KEEP_FAILED=1`, started 21:10:20Z, the same exit, summary and `FAIL [   8.790s]`):
  five starts; the third start, the trusted run, holds **two** prompts, the probe prompt (49 bytes) and the long
  text (1 500 bytes). The tag-like text and the local command were never typed.

This is the live round's own reading (`record-round-red.md`: `stamped 2.1.287  15 pass  2 fail`, two prompts in
Run B), as the plan predicted: the tree's settle returns the cleared screen's rows 5 s after the Stop, and the
guard pastes nothing more. No other shape was read.

## Green — after the change to `framing_turns`

Started 2026-10-06T21:11:26Z. Exit 0, `"ok":true`, `"suite":"nextest-integration","passed":1,"failed":0`.
nextest: `PASS [   9.736s]`, `1 test run: 1 passed, 325 skipped`.

- The change: after the long turn and after the tag-like turn the rows the paste guard reads come from
  `Run::box_wait`, which ends only on a quiet screen that holds a compiled literal (its rows), on a poisoned
  screen (no rows) or at `PROBE_DEADLINE` from that turn's Stop (no rows). A quiet screen with no literal keeps
  waiting past `GATE_MAX_WAIT`.
- The case asserts exit 0, the seventeen `pass` lines and `stamped 2.1.0  17 pass  0 fail` as the whole stdout,
  an empty stderr, and that the trusted run typed the four texts in order.
- Its length: 8.8 s red (the 5 s settle, then the agent's hold ends before it reads the Ctrl-C) and 9.7 s
  green, against the predicted 10 s to 11 s and the 45 s kill of the `verify_window_` class. No bound moved.

## What did not move
- The guard: `if !input_box_up(rows.as_deref())` is still at `src/cmd/verify/typed.rs:261` and `:278`, its bytes
  and its lines unchanged; only what feeds it after an added turn changed (`:276`). So the remove-the-guard
  control of `verify_pastes_nothing_more_once_the_turn_screen_shows_a_modal` was not re-run
  (`paste-guard-control.md` holds its two readings). That test reads the guard before the first added paste,
  whose rows still come from the first turn's settle.
- `settled`, `Run::settle` and its other call sites, the first turn's `turn_settle_ms` and `turn` screen, the
  settle after the local command (`:291`), `GATE_MAX_WAIT` and `crates/viola-agent-claude/src/screen.rs`.
