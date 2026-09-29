# Gate entry 6 (`bash scripts/agent-run.sh run`) — red, not this chunk's

**Record (overseer's ruling, 2026-09-29):** `red — not this chunk's: the same failures on the fb78ddc tree on this
host today → owner: the wrap (P5 pin)`. **Acceptance leg: CI**, which ran the same suite green on `fb78ddc`
(ci#36541599814, 15/15) and reads this chunk's push through the plan's `ci.py conclusion` entries.

## The red (this chunk's tree, 2026-09-29, gate run `2026-09-29T08-52-03-implement`)
- First firing: integration layer 166/230 passed, 64 failed (many FAIL + LEAK; wrappers leaked, stopped by exact path).
- Re-firing with the test-side fixes (seeded homes, DA1-answering piped driver, kill on panic — all in
  `tests/support/`): 167/230 passed, 63 failed, no LEAK. Failure shapes: a 7 s start wait timing out; `viola hook`
  over its 1.0 s spine bound (1.04 s).

## Two-sided basis (measured on this host, the same command on both trees, one after the other)
`cargo nextest run --features fake-agent --tests -p viola --no-fail-fast -E 'kind(test)'`, own target dir per tree,
the pre-chunk tree a detached worktree of `fb78ddc` on the same drive.

| tree | round 1 | round 2 | leftover processes stopped after |
|---|---|---|---|
| `fb78ddc` (control, no chunk edits) | 45 of 199 failed | 51 of 199 failed | 5 / 7 |
| this chunk | 54 of 204 failed | 53 of 204 failed | 0 / 0 |

Failing binaries, both trees: `channel_endpoint` 2 · `cli_fake_agent` 2-4 · `cli_instance_state` 7 ·
`cli_program_resolution` 1 · `cli_verify` 8 · `cli_version_gate` 2-7 · `contract_ledger_probes` 1 · `hook_events`
2-3 · `hook_fail_open` 2-3 · `run_cli` 8-10 · `tui_env_strip` 2 · `tui_passthrough` 4-5. Only this chunk:
`conpty_sideload` 5 (the new binary; it never seeds, since it tests the product's write path; alone it passed 8 of 8
runs). Excluding it, the chunk fails 48-49 against the control's 45-51.

## Entry 18 (`bash scripts/agent-run.sh pre-push`) — the same record
- linux-tests (WSL): green — coverage 938/938, playwright 1/1, `gate` no breaches; vm-release terminated clean.
- windows-tests (this host): coverage 917 passed, 58 failed. 53 are in binaries the `fb78ddc` control fails above
  (`channel_endpoint` 2 · `cli_fake_agent` 3 · `cli_instance_state` 7 · `cli_program_resolution` 1 · `cli_verify` 8 ·
  `cli_version_gate` 7 · `contract_ledger_probes` 1 · `hook_events` 3 · `hook_fail_open` 3 · `run_cli` 11 ·
  `tui_env_strip` 2 · `tui_passthrough` 5).
- The other 5 are `conpty_sideload` (the new binary, so the control cannot run it). All 5 fail at the shared
  `stamped_home` fixture's `viola verify` step (`viola never exited`, `tests/support/verify.rs:81`), before any
  sideload code runs (`verify` pins no companion). The control fails 22 and 25 tests with that identical message
  at that same fixture step (`booted_wrapper_fixture_is_ready_and_receipting`, `channel_wrapper_logs_each_call_with_corr_and_conn`,
  `run_refuses_a_live_name`, `path1_start_writes_state_before_the_spawn`, …). Recorded under this basis; the
  classification was flagged to the overseer.

## Host note
Idle outside the suite at the time of the read (CPU 6 %, 34 of 65 GB free); the overseer notes other sessions are
building heavily on this host today. The same tree passed this suite here at the last wrap; why it no longer does is
not established and was not chased (the overseer's ruling).
