# The floor of the killed harness test, measured (plan.md step 3)

**The floor is four waits of the gate's maximum.** Stop rule S6 did not fire, so step 4 moved the kill. No code
changed for this reading. Times are UTC, 2026-10-09; the tree is the base commit's source (`59e791e9d321`).

## Reading 1: boot step 4's command, by hand

The command `crates/viola-e2e/src/harness/boot.rs` `stamp` runs, against the harness-built fake agent, over a home
that did not exist before it (`target/e2e-home/viola-floor-735345/vhome`), with `VIOLA_NAME=floor` set for this one
command so that verify writes its process log (`src/cmd/verify.rs`: no instance, no log), and `VIOLA_DIR` and
`VIOLA_BIN` removed:

```
target/harness/debug/viola --home <root>/target/e2e-home/viola-floor-735345/vhome verify -- \
  <root>/target/harness/debug/viola-fake-agent --cli-version 9.9.9 --fixtures <root>/fixtures/claude \
  --screens --turn-stop --dialogs --framing --trusted-root <root>
```

`fixtures/claude/` holds `2.1.283`, `2.1.287` and `2.1.288`; it holds no `9.9.9`.

| reading | value |
|---|---|
| started | 15:54:56Z |
| exit | 1 |
| wall time, measured around the command | 34.541 s |
| the process's own `process-exit` (`subject` `self`) | `exit_code` 1, `duration_ms` 34507 |
| stdout | 18 lines; stderr 0 lines |
| host load at the start | 39.75 (1 min) |

The `process-exit` lines of `diagnostics/cli-floor.ndjson` whose `subject` is `verify-pty-probe`, one per
interactive run, in order:

| run | `duration_ms` | `child_exit_status` | ended |
|---|---|---|---|
| 1 | 8577 | 1 | 15:55:05.798 |
| 2 | 8561 | 1 | 15:55:14.360 |
| 3 | 8563 | 1 | 15:55:22.932 |
| 4 | 8562 | 1 | 15:55:31.497 |

Four lines, each at or above `GATE_MAX_WAIT` (8500 ms, `crates/viola-agent-claude/src/screen.rs`), 34263 ms
together. The two runs before them, the version probe and the print probe, took 11 ms and 12 ms. So the 34.5 s is
the four waits and nothing else of size: 34263 of 34507 ms.

## Reading 2: the test alone, under profile `ci`

`CARGO_TARGET_DIR=target/harness cargo nextest run --workspace --features viola/fake-agent --profile ci -E
'(kind(test)) & (test(/boot_with_an_unknown_cli_version/))'`, the form the harness's `run --integration --filter`
builds, started 15:55:55Z, exit 0. Its status lines, with nextest's own padding:

```
        SLOW [> 30.000s] (───) viola-e2e::harness_lifecycle boot_with_an_unknown_cli_version_is_verify_failed
        PASS [  34.482s] (1/1) viola-e2e::harness_lifecycle boot_with_an_unknown_cli_version_is_verify_failed
     Summary [  34.485s] 1 test run: 1 passed (1 slow), 341 skipped
```

## Against the forecast

The plan forecast four durations of at least 8500 ms and a PASS near 34.5 s. Measured: 8577, 8561, 8563 and
8562 ms, and PASS at 34.482 s. The audit's reading of the same test under profile `ci` was 34.477 s.

Under the `mutants` profile the test's kill was 30 s (the `package(viola-e2e)` override, 15 s × 2), below this
floor by construction. The `verify_window_` class's kill there is 45 s (15 s × 3), about 10.5 s above it.

## Left on the host

The home of reading 1 stands on the tmpfs behind the `target/e2e-home` link
(`target/e2e-home/viola-floor-735345/`). It was not removed by this chunk.
