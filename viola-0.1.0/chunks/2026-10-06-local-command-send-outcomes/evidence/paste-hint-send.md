# A `send` issued while the paste hint stands: the readiness-gate reading, measured end to end

The gate is unchanged (inputs#I2, option B). This is the measurement under the fake agent, with its control.

## The sequence
`send_under_the_paste_hint_on_a_verified_cli` (`tests/cli_send.rs`), two rstest cases, each on its own
`stamped_home` (a verified CLI: the full gate reads the compiled literals), the wrapper's fake agent started with
`--fixtures fixtures/claude --framing --turn-stop`:

1. `viola send builder --json` with the recorded 1 500-byte long text (taken from the `paste-1` variant). It is
   confirmed, exit 0.
2. The test waits for the turn's `turn-ended` record, then for the Stop hook's line in the fake agent's receipt.
   The agent draws its next screen right after that line: a cleared screen under the hold, the `turn` screen
   without it.
3. `viola send builder --json` with a second text.

The two cases differ in one argument: `hint` adds `--paste-hint-ms 3000`, `no_hint` adds nothing.

## The readings (2026-10-06T23:20:53Z run, this host, the decision wired)

| case | second send's exit | its document | records after its cursor | receipt |
|---|---|---|---|---|
| `hint` | 13 | `{"v":1,"refusal":"not-delivered","detail":"input-not-ready"}` | one `send-refused`, data `{"refusal":"not-delivered","detail":"input-not-ready"}`, no cursor; no second `send-issued` | one prompt, the long text; the second text was never typed |
| `no_hint` | 0 | `{"v":1,"ok":{…,"cursor":L}}` with `L` the log's length before the send | (not asserted) | two prompts, the second one the second text |

So on a verified CLI a `send` issued while the hint stands ends `input-not-ready` with nothing typed, about
300 ms after the screen goes quiet, and the same sequence without the hold is delivered. The 5 s maximum plays no
part: the cleared screen is quiet and holds no literal, so the verdict comes at the first quiet instant
(research M7).

What a driver is then told: `hint: builder was not ready for input; viola wait builder, then send again`
(`src/human.rs`, `send_hint`). In this window the turn has already ended, so a `viola wait` has no later event to
wake on. The remedy is not chosen here; it is carried to "First live test and self-drive".

## The hold, and each case's length

| run (UTC) | hold | `hint` | `no_hint` | note |
|---|---|---|---|---|
| 2026-10-06T23:16:58Z | 6 000 ms | 10.549 s | 4.884 s | before the decision was wired; the two cases pin behaviour that already existed, so both passed |
| 2026-10-06T23:20:53Z | 3 000 ms | 7.437 s | 4.747 s | the decision wired |

The plan named a 6 000 ms hold. At 6 000 ms the `hint` case read 10.549 s on this host. In CI it runs under the
20 s kill of `binary(cli_send)` (`.config/nextest.toml`, 10 s × 2), which it clears. But the case carries no
`send_window_` or `verify_window_` prefix, so under the nextest `mutants` profile its kill is 10 s (5 s × 2), and
10.549 s is over it: a mutation run's unmutated baseline would have lost the test. The hold was shortened to
3 000 ms, as the plan's note directs for a case that nears its kill ("shorten the hold; never move the bound").
No bound moved.

What the hold has to outlast is the second send's gate verdict: one `viola send` process start, then the 300 ms
quiet period. That span was not timed on its own; 3 000 ms is ten quiet periods.

Each case's length is mostly its stamp: a whole `viola verify` against the fake agent read 3.4 s to 3.7 s in the
same runs (`verify_pastes_no_local_command_once_the_tag_turn_screen_shows_a_modal`). The `no_hint` case is the
stamp, the boot and two sends.

## What is not measured
- The real CLI. The hint window on 2.1.287 was measured once (8.0 s from the paste, 6.5 s after the Stop;
  the prior chunk's `evidence/rehearsal-shapes.md`); no `send` was run in it against a live session, and none is
  here (inputs#I1).
- The three CI runners. The lengths above are this host's.
