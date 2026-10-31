# P5 controls — 2026-10-31-exit-cause-code-catalogue

## The preservation guard, gate entry 13
Fired as the entry over a scratch clone of HEAD `404f7496dbd7`, by `p5-guard-controls.sh` in this run dir (the clone in the
script's own `$(mktemp -d)`, removed by its `trap`).

| Clone's input | Reading |
|---|---|
| unmodified | green, `guard: clean` |
| every path of research's two lists changed: the 40 modified and the new `tests/cross_exit_causes.rs` | green |
| four paths outside the lists (`src/cmd/mcp.rs`, `tests/support/fake.rs`, `Cargo.lock`, an untracked `crates/viola-channel/src/new_file.rs`) | red, `guard: outside 4 files` |

## Entry 10's literal
Entry 10 reads red on an absent input, the binary step 14 creates, so its `"ok":true` needs a must-pass reading. Entry 9 is
that reading: the same `bash scripts/agent-run.sh run --e2e --filter …` one filter operand apart, green with `"ok":true`, 77 tests
(check 4 (9), a form differing by an input operand alone). Implement's red-before-green run is the entry's own control.

## Check 8's reach, by measurement
Over the build the baselines left, `target/debug/viola`, from `/usr/bin/env -i` with an empty `PATH`, in homes this session made
under its scratchpad.

| Input | Reading |
|---|---|
| a fresh home, `wait builder --json` | exit 21, today's document; the home then holds `diagnostics` and no `instances/`: an unwrapped name is reachable, as `inputs#I2` Q2 reads it |
| a regular file at `diagnostics`, no `VIOLA_NAME`, `list --json` | exit 0 and an empty board: **the plan's row was false**, `list` opens no log with no name |
| the same, `list` | exit 0 and the empty board |
| the same, `wait builder --json` | `error: internal error`, exit 1: the catch site, the row as corrected |
| the same, `VIOLA_NAME=caller`, `list --json` | `error: internal error`, exit 1 |

## What the reads changed
- Step 14's `internal-error` row: `wait --json` in place of `list --json`.
- Step 15: `send_unreachable_name_exits_21_not_running` and `wait_last_unreachable_exit_21` stand on a fresh home, so they move to
  the unwrapped hint and `detail`. Research's `instance-dead` sweep had `tests/cli_send.rs:1085` staying; corrected, and an `is not
  running` sweep added.
- Step 13: `first_life` calls `wait_session_start_hooks`, and the `session-live` row needs `listed`; both move with it.
- Step 11: `record` returns `u8` at HEAD, not `Ended`.
- Step 9: `trusted`'s `?` joins the errors that print the `internal-error` document.
- Step 17: the census reads the build pre-push leaves, so it runs after that entry.

## Host
`df -h /tmp`, read before the stamped-home gates: tmpfs 32G, 9.3G used, 23G avail, 30 %. `target/e2e-home` links to
`$TMPDIR/viola-e2e-home-1000`.
