# Red that caused a fix: a torn receipt read in the fixture support

Captured by `/andromeda-implement` P2, gate 20 (`bash scripts/agent-run.sh pre-push`), the run begun
after the flag-literal fix. Folded into this chunk by overseer direction (founder-delegated,
2026-09-27): a red found now folds in even outside the chunk's diff; test-plan §10's zero-flakiness
budget allows no retry, so a later green re-run does not close it.

## The red

- Stage `linux-tests` (WSL2 `Ubuntu`, the `ubuntu-latest` test job's suites under `run --coverage`).
- Synced tree `0e1001414c65efc83527fd11223f6b0e2198f61b` on HEAD
  `17c99c9cd098256405af2b9312dddbc740fce14b`.
- Coverage suite: 591 passed, 1 failed:
  `viola::tui_passthrough tui_host_resize_reaches_the_child` (0.766 s).
- The chain, verbatim from the run's failing block:

```
thread 'tui_host_resize_reaches_the_child' (6206) panicked at tests/support/fake.rs:39:42:
one JSON object per receipt line: Error("EOF while parsing a string", line: 1, column: 29)
```

- The same test passed in the two earlier full Linux runs of this chunk's tree family (592/592 each).

## Cause

`tests/support/fake.rs` `receipt()` read the fake agent's receipt with `read_to_string` and parsed
every `lines()` item with `expect`. The agent appends one line per `write_all`, but a concurrent
reader of a regular file can observe an append part-way (Linux page-cache copy), so the reader saw a
final line without its `\n` and failed parsing it. Every test that polls a receipt (`wait_for`,
`stays_false`, the boot fixtures) inherits the race.

## Fix

`receipt()` consumes only newline-terminated lines: the bytes after the last `\n` are a line the
agent is still writing, never parsed. The unit test `receipt_reader_leaves_a_torn_final_line_unread`
(`tests/cli_fake_agent.rs`) feeds a torn final line: red on the old reader, green on the fixed one
(readings below).

## Readings

Both on the Windows host, `cargo nextest run -p viola --features fake-agent --test cli_fake_agent -E
'test(receipt_reader_leaves_a_torn_final_line_unread)'`:

- **Red, the old reader** (test written first, fixture unchanged): `FAIL [0.055s]`, panicked at
  `tests\support\fake.rs:39:42`: `one JSON object per receipt line: Error("EOF while parsing a string",
  line: 1, column: 17)` — the pre-push chain's site and error, reproduced deterministically.
- **Green, the fixed reader**: `PASS [0.033s]`. The whole root suite with it: 193 passed.

The fixed reader stays strict: a complete line that is not one JSON object (a blank line included)
still fails the test; only the unterminated tail is left unread.

## The class, swept (overseer direction: same class, same fold)

One shared reader, `tests/support/ndjson.rs` (`complete_lines(bytes)`, `read_lines(path)`), reads only
newline-terminated lines; the receipt reader is now `read_lines` too. The sweep: `rg -U --pcre2
'(read_to_string|fs::read)\(…\)…\.lines\(\)'` over `tests`, `crates/*/tests`, `src`, `crates/*/src`,
plus every `events.ndjson` / `diagnostics` / `run-<name>` reference under `tests` and
`crates/viola-e2e/tests`.

Moved onto the shared reader (each parses lines of a file a live process may still append to):

| site | file read |
|---|---|
| `tests/support/fake.rs` `receipt` | fake-agent receipt |
| `tests/support/home.rs` `Starts::read` | role file, in the readiness loop (was tolerant, now strict on complete lines) |
| `tests/cli_instance_state.rs` `lines` (`events`, `role_lines`) | `events.ndjson`, role file |
| `tests/cli_instance_state.rs` the `added` slice of `run_takes_over_a_gone_name_and_appends` | `events.ndjson` |
| `tests/channel_endpoint.rs` `role_lines` | role file |
| `tests/contract_diag_schema.rs` `run_lines` | role file |
| `tests/tui_passthrough.rs` `role_lines` | role file |
| `tests/tui_env_strip.rs` the child-start read | role file |
| `tests/cli_program_resolution.rs` `role_lines` | role file |
| `tests/cli_fake_agent.rs` `fake_agent_exit_no_eof_exits_while_stdout_is_held` | role file |
| `tests/run_cli.rs` `role_lines` | role file |

Swept and left, with the reason:

| site | why it is not this class |
|---|---|
| `tests/run_cli.rs:411` detail file | written by a `viola` that `run_captured` has already reaped |
| `tests/cli_program_resolution.rs:71`, `tests/run_cli.rs:264` | substring checks, no line parse (`:264` polls until present) |
| `tests/run_cli.rs:433`, `tests/channel_endpoint.rs:266` | canary byte scans, no line parse |
| `src/obs.rs:666`, `src/main.rs:303,317`, `crates/viola-state/src/events.rs:133` | unit tests; the write completes in-process before the read |
| `crates/viola-e2e/src/harness/logs.rs` | the harness reader emits a torn line as `{"torn":true}` by design (test-plan §3 `logs`) |
| `crates/viola-e2e/src/harness/run/mutants.rs:195` | cargo-mutants output read after the tool exits |
| `crates/viola-pty/src/lib.rs:1098` | a child's plain-text report in a poll-until-match loop; a torn tail only delays the match |

After the move: the root suite 193 passed on this host; the re-run of the sweep finds only the
`run_cli.rs:411` detail read above.

## Not closed by a re-run

The pre-push run in flight when the direction arrived had been started before this fix; it was
stopped (host chain by verified pid, distro side checked clean) and does not count. So was the
gate run started after the receipt-only fix, once the sweep was directed (stopped at gate 8, its
tree by pid; four unrelated `viola.exe` of the viola-lab prototype left alone). The green that
counts is one run of entries 1-20 on the tree carrying the whole sweep.
