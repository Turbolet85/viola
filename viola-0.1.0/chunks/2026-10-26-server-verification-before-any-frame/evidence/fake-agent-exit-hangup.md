# The fake agent's exit out of the hangup's reach (the wrap's P7.1 red, folded: inputs#I13, inputs#I14)

## The mechanism, read in source
- portable-pty 0.8.1 makes the PTY child its session's leader with the terminal as its controlling tty
  (`unix.rs:220` `setsid()`, `:234` `ioctl(0, TIOCSCTTY, 0)`), so the fake agent under `viola run` is that leader.
- A hangup of that terminal (the master closed: a killed wrapper) signals the session leader with SIGHUP and SIGCONT
  (Linux `tty_signal_session_leader`); portable-pty's own `Child::kill` on Unix is SIGHUP first, then SIGKILL after a
  200 ms grace (`lib.rs:333-361`), which the wrapper's `kill_and_reap` and every `Pty::kill` reach.
- The Epoch 5 fix moved the agent's hooks and status line out of the foreground group (`process_group(0)`,
  `chunks/2026-10-20-epoch-5-cleanup/evidence/profraw-cause.md`). The agent itself cannot move: `setpgid` refuses a
  session leader (POSIX `EPERM`). So the change takes its EXIT out of the hangup's reach instead: from its exit's first
  step (`\x03`, or stdin's end) it sets SIGHUP to `SIG_IGN` (`src/bin/viola-fake-agent.rs` `ignore_hangup`, called at
  both of `read_stdin`'s exits, before `close_hooks`). A hangup that lands mid its exit-time coverage write no longer
  cuts the profile; one that lands before the exit begins still ends it as before (no profile at all, which the merge
  does not refuse). The `--exit-no-eof` holder, spawned after it, inherits the disposition; it is outside the group
  already. No product code changed; the fake agent is behind `fake-agent`, never in a release build.

## The test and its red-before-green (`tests/cli_fake_agent.rs`, Linux only)
`fake_agent_exit_outlives_a_hangup_that_lands_while_it_exits`: a scripted Stop whose hook is the agent's own
`statusline-echo` (the marker shows the hook ran), held open by the existing capped `--stop-receipt-hold-ms 1000`
(testing.md 2026-09-27: force the window open with a test-only hold); `\x03` starts the exit, which waits the hold out in
`close_hooks`; the test waits for `/proc/<pid>/status` `SigIgn` bit 0 (SIGHUP) while the agent runs, checks the Stop
hook's receipt line is still unwritten (the hangup lands inside the exit), sends SIGHUP to the child it spawned, and
asserts exit code 0 with no signal and the one `Stop:true` hook line written after the hold.

| reading | tree | command | result |
|---|---|---|---|
| red | `ignore_hangup` neutralised (`SIG_IGN` → `SIG_DFL`), 2026-10-26T11:43:50Z | `cargo nextest run --features fake-agent -E 'binary(cli_fake_agent) and test(/fake_agent_exit_outlives/)'` | exit 100, `1 test run: 0 passed, 1 failed` — «the agent exited (Some(ExitStatus(unix_wait_status(0)))) before the exit ignoring SIGHUP» at `tests/cli_fake_agent.rs:915:9` |
| green | restored (`SIG_IGN`, re-read at `src/bin/viola-fake-agent.rs:1098`), 2026-10-26T11:44:22Z | `cargo nextest run --features fake-agent -E 'binary(cli_fake_agent)'` | exit 0, `43 tests run: 43 passed, 0 skipped`; the new case `PASS [ 1.009s]` |
| stress | the same, 2026-10-26T11:44:34Z → 11:45:14Z | the first form with `--stress-count 40` | exit 0, `40/40 stress run iterations: 40 passed` |

## The profile censuses (the instrumented build block run 1's `pre-push` left)
Build: `target/llvm-cov-target/debug/viola-fake-agent` written 2026-10-26T11:46:45Z, after the fix; `/tmp` 25G avail,
22 % before. Each profile offered alone to `llvm-profdata merge` (`scripts/profraw-census.sh --case`).

| census | case | runs | passed | profiles | per run | refused |
|---|---|---|---|---|---|---|
| `svf-exit-1` | `chaos_wait_vanish chaos_wait_parked_wrapper_killed_exits_21` (a killed wrapper hangs up the agent's terminal; Epoch 5's kill-shaped case) | 200 | 200 | 801 | 4.005 | 0 |
| `svf-exit-2` | `cli_fake_agent fake_agent_exit_outlives_a_hangup_that_lands_while_it_exits` (the hangup inside the exit) | 40 | 40 | 120 | 3 exactly | 0 |

Reading: in `svf-exit-2` every run leaves the test process's, the agent's and its Stop hook's profile, whole, though a
SIGHUP reached the agent inside its exit each time — the exit-time write the fix protects. `svf-exit-1` reads 0 refused,
as the Epoch 5 fix's census did (800 profiles, 4 a run); one run left a fifth profile. Limits: the wrap's red (1 cut-short
fake-agent profile of 12 583, none in 1 600 census runs or 8 parent pre-push runs, `basis-loop.md`) was never reproduced
on demand, so these readings show the mechanism closed and nothing refused, not the original file's writer named.
