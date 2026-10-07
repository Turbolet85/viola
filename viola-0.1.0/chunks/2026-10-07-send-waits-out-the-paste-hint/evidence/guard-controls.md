# Remove-the-guard controls (step 5)

Three guards, each neutralised inside the function the cases call, the named cases read red, the guard restored,
the site re-read and the cases read green again. One-shot runs, never committed. Every run is under the nextest
`mutants` profile with `--no-fail-fast`, so each named case's own line is printed (the profile's `max-fail = 1`
would stop scheduling at the first red). Host: the Linux dev host, cargo-nextest 0.9.146. Times are `date -u`,
2026-10-07.

The restore check for each: the site re-read after the restoring edit (the lines are quoted below), and the file
compared byte for byte (`cmp`) with the copy saved before the first control. `cmp` exits 0 for
`crates/viola-agent-claude/src/screen.rs` and for `src/run/send.rs` after the last control.

## 1. The waiting arm in `Screen::verdict`

Guard, `crates/viola-agent-claude/src/screen.rs`:

```
        } else if now.saturating_duration_since(waiting_since) < GATE_MAX_WAIT {
            GateStep::Wait
        } else {
```

Neutralised (12:29:54Z to 12:30:00Z): the arm's `GateStep::Wait` replaced by
`GateStep::Done(Readiness::InputNotReady)`, the verdict the product gave before this chunk. Site re-read after the
edit and before the run.

Commands:
- `cargo nextest run -p viola-agent-claude --profile mutants --no-fail-fast -E 'test(/waits_for_the_input_box/)'`
- `cargo nextest run --bin viola --features fake-agent --profile mutants --no-fail-fast -E 'test(/waits_for_the_input_box/)'`

Red, both exit 100:

```
        FAIL [   0.005s] (5/7) viola-agent-claude screen::tests::verdict_verified_without_a_literal_waits_for_the_input_box::case_1_at_the_first_quiet_instant
        FAIL [   0.006s] (7/7) viola-agent-claude screen::tests::verdict_verified_without_a_literal_waits_for_the_input_box::case_2_one_ms_short_of_the_maximum
     Summary [   0.007s] 7 tests run: 5 passed, 2 failed, 319 skipped
        FAIL [   0.005s] (1/2) viola::bin/viola run::gate::tests::wait_ready_verified_waits_for_the_input_box_until_the_bound
        FAIL [   0.005s] (2/2) viola::bin/viola run::send::tests::send_on_a_verified_screen_without_a_literal_waits_for_the_input_box_to_the_bound
     Summary [   0.006s] 2 tests run: 0 passed, 2 failed, 484 skipped
```

The five that stay green with the guard removed read a verdict the arm does not decide: the two cases at and past
8 500 ms, the two input-box cases (decided by the quiet period) and the modal and poisoned case.

Restored (12:30:09Z to 12:30:12Z), the site re-read as the guard above, the same two commands, both exit 0:

```
     Summary [   0.005s] 7 tests run: 7 passed, 319 skipped
     Summary [   0.167s] 2 tests run: 2 passed, 484 skipped
```

## 2. The wheel read after the gate's wait

Guard, `src/run/send.rs`, the second `human_typing` read, after the gate's verdict:

```
    if let Some(detail) = slot.wheel.human_typing() {
        return ctx.refuse(
            None,
            RefusalReason::HumanTyping,
            detail.map(HumanTyping::as_str),
        );
    }
```

Neutralised (12:30:22Z to 12:30:35Z): `.filter(|_| false)` appended to the read, so the block never returns.
Site re-read after the edit and before the run.

Commands:
- `cargo nextest run --bin viola --features fake-agent --profile mutants --no-fail-fast -E 'test(/after_the_gate_wait/)'`
- `cargo nextest run --test cli_send --features fake-agent --profile mutants --no-fail-fast -E 'test(/a_human_key_during_the_gate_wait/)'`

Red, both exit 100:

```
        FAIL [   0.025s] (1/3) viola::bin/viola run::send::tests::send_after_the_gate_wait_human_typing_comes_before_turn_running
        PASS [   0.026s] (2/3) viola::bin/viola run::send::tests::send_a_turn_started_after_the_gate_wait_is_turn_running
        FAIL [   0.206s] (3/3) viola::bin/viola run::send::tests::send_a_human_key_after_the_gate_wait_is_human_typing
     Summary [   0.207s] 3 tests run: 1 passed, 2 failed, 483 skipped
        SLOW [>  5.000s] (───) viola::cli_send send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins
     TIMEOUT [  10.004s] (1/1) viola::cli_send send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins
     Summary [  10.004s] 1 test run: 0 passed, 1 timed out, 15 skipped
```

What each red read:
- `…human_typing_comes_before_turn_running`: the reply was `not-delivered` / `turn-running`, the turn's rung,
  where the case expects `human-typing`.
- `…a_human_key_after_the_gate_wait_is_human_typing`: the send did not stop after the gate. Its clock read eleven
  seconds past the expected reading, which is the first assertion the case reaches; that length fits a
  confirmation window waited out on this clock (inferred from the code, not asserted by the run).
- The end-to-end keystroke case: killed by the profile's 10 s line while it waited for the send to exit. The kill
  is the red reading; the case's own assertions were not reached, so what the send did past the gate is not read
  here (with the guard in, the same case reads the refusal in 7.1 s).
- `…a_turn_started_after_the_gate_wait_is_turn_running` stays green: its guard is the turn read, still in place.

After the killed run: no `viola`, fake-agent or `claude` process whose executable or cwd is under this repository
was left (read by `/proc/<pid>/exe` and `/proc/<pid>/cwd`). The killed test's home `viola-test-*` stayed under
`target/e2e-home/`, as a runner-killed test's home does; the fixture's own sweep removes it at the next new home.

Restored (12:30:57Z to 12:31:06Z), the site re-read as the guard above, the same two commands, both exit 0:

```
     Summary [   0.026s] 3 tests run: 3 passed, 483 skipped
        SLOW [>  5.000s] (───) viola::cli_send send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins
        PASS [   7.089s] (1/1) viola::cli_send send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins
     Summary [   7.090s] 1 test run: 1 passed (1 slow), 15 skipped
```

## 3. The turn read after the gate's wait

Guard, `src/run/send.rs`, right after the wheel read:

```
    if slot.wheel.turn_running() {
        return ctx.not_delivered(None, NotDelivered::TurnRunning);
    }
```

Neutralised (12:31:13Z to 12:31:14Z): `&& false` appended to the condition. Site re-read after the edit and
before the run.

Command:
`cargo nextest run --bin viola --features fake-agent --profile mutants --no-fail-fast -E 'test(/after_the_gate_wait/)'`

Red, exit 100:

```
        PASS [   0.025s] (1/3) viola::bin/viola run::send::tests::send_after_the_gate_wait_human_typing_comes_before_turn_running
        PASS [   0.025s] (2/3) viola::bin/viola run::send::tests::send_a_human_key_after_the_gate_wait_is_human_typing
        FAIL [   0.206s] (3/3) viola::bin/viola run::send::tests::send_a_turn_started_after_the_gate_wait_is_turn_running
     Summary [   0.207s] 3 tests run: 2 passed, 1 failed, 483 skipped
```

The red case's send did not stop after the gate: its clock read eleven seconds past the expected reading, the
same first assertion as in control 2. The two human-typing cases stay green: the wheel read, still in place,
decides them.

Restored (12:31:22Z to 12:31:23Z), the site re-read as the guard above, the same command, exit 0:

```
     Summary [   0.026s] 3 tests run: 3 passed, 483 skipped
```
