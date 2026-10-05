# Run C / Run D kill after the settle — red before green (CI round 1 fix, option 1)

**Authority:** the overseer's decision, founder-delegated (2026-10-05, relayed by the operator): "option 1 now (settle
the screen before the Run C/D kill, as Run B already does) … Show red-before-green where you can".

**The window.** Run C and Run D were killed the instant the last Stop capture appeared. The Stop hook (an instrumented
`viola hook --capture` under coverage) writes that capture and then exits; a kill inside that exit hangs the hook up
through the fake agent's PTY, which can truncate its coverage profile (the hazard the fake agent's own docs record,
and the class of the three corrupt `.profraw` files of ci#37316283001).

**Forced open, never sampled.** The fake agent's test-only argv mode `--stop-receipt-hold-ms <n>` (capped at 1 000 ms)
holds each Stop hook's receipt `n` ms past the hook's exit. The test
`cli_verify::verify_kills_the_dialog_and_plan_runs_only_after_their_last_stop_hook` runs verify with a 100 ms hold and
counts the Stop hook receipts of the six turns (the print turn, Run B, Run C's three, Run D).

| reading | Run C / Run D end | result |
|---|---|---|
| RED (HEAD `43e6245`'s typed.rs) | `end_by_kill` the instant the last Stop capture is seen | exit 1, `FAIL`, `left: 4` `right: 6`: Run C's last Stop and Run D's Stop were never receipted |
| GREEN (the fix) | `run.settle(<the instant the last Stop was seen>)`, then `end_by_kill` | exit 0, `PASS`, 6 of 6 |

Both readings ran on 2026-10-05 with one command:
`bash scripts/agent-run.sh run --integration --filter 'test(/verify_kills_the_dialog_and_plan_runs_only_after_their_last_stop_hook/)'`.

With the settle, the identity-strip test's hook count is exact again (29; it was a 27-29 range while the kill raced the
last receipt). Measured locally (harness build, no coverage), one verify against the fake over the recorded 2.1.287 set:
2.11 s before → 2.73 s after (Run C 479 → 780 ms, Run D 405 → 719 ms). The test bound (`WITHIN` = 7 s) is unchanged.
