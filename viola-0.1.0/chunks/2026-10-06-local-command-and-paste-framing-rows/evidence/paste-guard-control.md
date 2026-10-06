# The paste guard's remove-the-guard control, and the timing read so far

## The guard
`src/cmd/verify/typed.rs`, `framing_turns`: before each of Run B's added pastes the settled rows must hold the
input-box literal and no modal literal, else nothing more is pasted. Its test is
`verify_pastes_nothing_more_once_the_turn_screen_shows_a_modal` (`tests/cli_verify.rs`): the fake agent's `turn`
screen shows a modal literal beside the input box, and the test asserts that the trusted run typed the probe
prompt alone and that the three framing rows read `fail`.

Both readings are the same command:

```
bash scripts/agent-run.sh run --integration --filter 'test(/verify_pastes_nothing_more_once_the_turn_screen_shows_a_modal/)'
```

| reading | started (UTC) | the guard's first line | result |
|---|---|---|---|
| red | 2026-10-06T20:40:55Z | `if false && !input_box_up(rows.as_deref()) {` (re-read before the run) | exit 1, `"passed":0,"failed":1`; left `"pass"`, right `"fail"`: with the guard off the long text was pasted under the modal screen and its row passed |
| green | 2026-10-06T20:41:08Z | `if !input_box_up(rows.as_deref()) {` (re-read; no `if false` left in the file) | exit 0, `"passed":1,"failed":0` |

The control neutralised the guard before the long and the tag-like pastes. The same check before the local
command's paste has no control of its own.

This is the guard that held in the live record round (`record-round-red.md`).

## Step 9 — timing, as far as it could be read
The first full local `run` after step 7 was not fired: every test that boots a stamped home needs the recorded
framing variants, which the red round did not produce. What was read, from the `cli_verify` binary alone
(2026-10-06 ~20:30Z, 27 tests, all passed, the synthetic 2.1.0 set):

- a complete fake-agent `viola verify` test: 3.9 s to 4.6 s (17 tests between those bounds);
- `verify_kills_the_dialog_and_plan_runs_only_after_their_last_stop_hook`: 5.3 s;
- `verify_window_without_screens_fails_every_interactive_row`: 21.1 s, under its 45 s kill. Without screens Run B
  pastes nothing, so this test's length is unchanged by the three added turns.

No bound in `.config/nextest.toml` was moved. The read against the 20 s kill of the verify-driving binaries is
still owed, and so is the comparison with the same tests at HEAD: no archived run of HEAD's `cli_verify` is left
on this host (`target/run-archive` keeps the newest ten).
