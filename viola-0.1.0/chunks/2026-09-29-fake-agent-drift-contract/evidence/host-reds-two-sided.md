# Gate entries 7 and 8 — red on this host, judged against a same-day control

**Record:** `red — not this chunk's: the same failures, by name and message, on the 2d8bc53 tree on this host today;
every red on this chunk's own surface is green run alone → owner: the wrap (P5 pin, the host-reds CARRY)`.
**Acceptance leg: CI** (the plan's operator `ci.py conclusion` entry on the final HEAD).

## The control
A detached worktree of `2d8bc53` (the chunk base: HEAD, no chunk edits) at `<repo parent>/viola-ctl-2d8bc53` —
the same drive, its own `target/`. Each control run used the identical harness command as the subject's gate entry,
fired immediately before or after the subject's, never at the same time. Gate run `2026-09-29T13-30-05-implement`.

## Entry 7 — `run --integration --filter 'binary(tui_pty_seam) | binary(tui_passthrough) | binary(cli_fake_agent) | binary(cli_instance_state)'`

| run (in order) | tree | passed / failed of | failing only here |
|---|---|---|---|
| 1 | this chunk | 39 / 10 of 49 | — |
| 2 | `2d8bc53` control | 32 / 16 of 48 | — |
| 3 | this chunk | 37 / 12 of 49 | none — a strict subset of run 2's 16 |

Messages, both trees: `viola never exited` ×9 (the stamped-home fixture's `viola verify` step,
`tests/support/verify.rs:81`, the 7 s `WITHIN`), `wrapper builder not ready` (`starts w0 c0 r0` → `w1 c0 r0`), and
in runs 2-3 `timed out waiting for start` (receipt lines 0) and `outer pty child never exited`. All three
`tui_pty_seam` tests (the new key-free witness included) and both moved `stdin_hex` pins passed in runs 1 and 3.

## Entry 8 — `bash scripts/agent-run.sh run` (the default selection)

| run (in order) | tree | unit | integration passed / failed of |
|---|---|---|---|
| 1 | this chunk (run-archive 394) | 750 / 0 | 177 / 58 of 235 |
| 2 | `2d8bc53` control | 745 / 0 | 174 / 56 of 230 |

Failing binaries — this chunk: `channel_endpoint` 2 · `cli_fake_agent` 4 · `cli_instance_state` 7 ·
`cli_program_resolution` 1 · `cli_verify` 8 · `cli_version_gate` 8 · `conpty_sideload` 5 · `contract_ledger_probes` 1 ·
`hook_events` 3 · `hook_fail_open` 2 · `run_cli` 10 · `tui_env_strip` 2 · `tui_passthrough` 5. Control:
`channel_endpoint` 2 · `cli_fake_agent` 5 · `cli_instance_state` 7 · `cli_program_resolution` 1 · `cli_verify` 8 ·
`cli_version_gate` 8 · `conpty_sideload` 5 · `contract_diag_schema` 1 · `contract_ledger_probes` 1 · `hook_events` 3 ·
`hook_fail_open` 4 · `run_cli` 10 · `tui_env_strip` 1.

- Failing only on this chunk in this pair (6): `tui_passthrough` ×5 and `tui_env_strip tui_env_viola_names_reach_the_child`.
  Every one fails BEFORE the fake agent's first receipt line: `timed out waiting for start` / `… for env` with
  `receipt lines 0`, `the wrapper never spawned the child` (`tests/tui_passthrough.rs:132`), `outer pty child never
  exited`. The step-4 `size` watcher starts only after `start_receipts` writes `start`/`cwd`/`env`, so no failing test
  reached code this chunk changed. The control failed four of these same `tui_passthrough` tests, with the same
  messages, in entry 7 run 2 above.
  **Run alone on this chunk's tree** (`run --integration --filter 'binary(tui_passthrough) | binary(tui_env_strip)'`,
  quiet host), three times in a row: 7 of 7 passed each time (7.0-7.2 s).
- Failing only on the control (4): `cli_fake_agent wrapper_boot_exiting_before_ready_fails_as_exited`,
  `contract_diag_schema diag_lines_from_real_runs_validate`, `hook_fail_open …case_01_oversize_stdin`, `…case_09_no_snapshot`.
- `contract_ledger_probes` (red on both trees): `viola verify` over the recorded set drives the fake agent's
  UserPromptSubmit bytes, which step 2 changed, so it was checked on its own. This chunk's red read `viola never exited`
  + LEAK at 24.6 s under the parallel run. **Alone on this chunk's tree**, three times in a row: passed each time
  (2.4 s, 1.3 s, 1.1 s) — verify stamps all six rows over `fixtures/claude/2.1.283` with the kept trailing newline.
- `contract_fake_agent_drift` in the archived integration JUnit: `tests="4" failures="0"` (the artifact fresh, written by
  run 1).

## Entry 12 — `bash scripts/agent-run.sh pre-push`
- `sync`: this chunk's uncommitted tree (54 files over `2d8bc53`) into the WSL clone. **linux-tests: green** —
  coverage 948/948 (938 at the last wrap + this chunk's 10 new tests), doctest, playwright 1/1, `gate` green;
  `vm-release` terminated.
- **windows-tests (this host): red** — coverage 924 passed, 61 failed, in the host-reds class binaries only
  (`channel_endpoint`, `cli_fake_agent`, `cli_instance_state`, `cli_verify`, `cli_version_gate`, `conpty_sideload`,
  `contract_ledger_probes`, `hook_events`, `hook_fail_open`, `run_cli`, `tui_env_strip`, `tui_passthrough`).

### Its stage command, `run --coverage`, as two like-for-like pairs (each run standalone, one after the other)

| binary | pair 1: control (1st) | pair 1: chunk (2nd) | pair 2: chunk (1st) | pair 2: control (2nd) |
|---|---|---|---|---|
| total failed | 44 of 975 | 55 of 985 | 61 of 985 | 57 of 975 |
| `channel_endpoint` | 2 | 2 | 2 | 2 |
| `cli_fake_agent` | 2 | 2 | 5 | 5 |
| `cli_instance_state` | 7 | 7 | 7 | 7 |
| `cli_program_resolution` | 0 | 0 | 0 | 1 |
| `cli_verify` | 8 | 8 | 8 | 8 |
| `cli_version_gate` | 5 | 7 | 8 | 5 |
| `conpty_sideload` | 5 | 4 | 5 | 5 |
| `contract_ledger_probes` | 0 | 1 | 1 | 1 |
| `hook_events` | 0 | 3 | 3 | 3 |
| `hook_fail_open` | 2 | 5 | 5 | 3 |
| `run_cli` | 6 | 9 | 10 | 10 |
| `tui_env_strip` | 2 | 2 | 2 | 2 |
| `tui_passthrough` | 5 | 5 | 5 | 5 |

The control itself moved 44 → 57 between its two runs; most of pair 1's chunk-only names (`hook_events`, `run_cli`,
`contract_ledger_probes`, `cli_fake_agent`) are red on the control in pair 2. Pair 2's chunk-only names (5):
`cli_version_gate` ×3 (`wrapper builder not ready` / `ready timed out` — the 7 s start wait) and `hook_fail_open`
`case_02_malformed_json` / `case_09_no_snapshot` (`took 1.19 s` / `1.23 s` over the 1.0 s hook spine bound).

### Those two binaries alone, three rounds per tree (`run --integration --filter 'binary(cli_version_gate) | binary(hook_fail_open)'`, 23 tests)

| round | this chunk | `2d8bc53` control |
|---|---|---|
| 1 | 23 / 23 | 21 / 23 — `hook_forced_panic_fails_open_with_one_role_line_and_one_detail_line` (`took 1.0885656s`), `run_with_a_child_that_is_not_the_cli_records_no_version` |
| 2 | 22 / 23 — `hook_forced_panic_fails_open_with_one_role_line_and_one_detail_line` (`took 1.0427716s`) | 23 / 23 |
| 3 | 23 / 23 | 23 / 23 |

The one quiet-host red is the same test, on the same bound, on both trees.

## The step-4 watcher's own load (measured, since the residual reds are load-bound)
Each tree's boot smoke (`boot --session … --instance builder`, the plan's entry 14 form), then 20 s idle, CPU time per
process by `ExecutablePath` (`TotalProcessorTime`, 15.6 ms tick):

| process | this chunk | `2d8bc53` control |
|---|---|---|
| fake agent (`claude.exe`) | 46.9 ms | 0 ms |
| `OpenConsole.exe` | 15.6 ms | 0 ms |
| wrapper `viola.exe` | 203.1 ms | 203.1 ms |
| `viola-harness.exe` | 15.6 ms | 0 ms |

The 10 ms `CONTROL_POLL` watcher costs ≈ 3 ms/s per fake agent in a PTY, plus its console round trip (≈ 1 ms/s in
`OpenConsole`): ≈ 0.3 % of one core per session, a third of the wrapper's own idle cost. Measured, not zero, and not
enough to separate the trees above: the quiet-host rounds show the control failing the bound at least as often.

## Process census after the runs
`Get-CimInstance Win32_Process` for any `ExecutablePath` under `D:\dev\projects\viola*`: 0 (the same query counts 36
Git processes, so it reads). No survivor from either tree.
