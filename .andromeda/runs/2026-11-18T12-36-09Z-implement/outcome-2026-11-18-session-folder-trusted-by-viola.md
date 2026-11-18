# Outcome — 2026-11-18-session-folder-trusted-by-viola

**Run:** 2026-11-18T12-36-09Z-implement · **Outcome:** stuck — a red live entry (14, `S2`) stops implement before the
operator pass (inputs#I3, the overseer's implement directive, snapped this run); its mend moves a plan entry, a
revision through the phase's door. No report written (the wrap authors it).

## What stuck
- **Entry 14 (`S2`) red: `refused: cap`** (rig exit 3, 0.1 s, before its ledger line: uncounted, no `claude`
  started, `census: 0` after). The start itself showed the closed word `cap`. Cause, read in
  `crates/viola-e2e/src/rig/ledger.rs` `admit`: a base label is admitted while fewer than `--cap` base starts stand in
  the ledger FILE, whatever their label; `S1` and `S2` share `evidence/live-starts.ndjson` and the plan gives each
  `--cap 1`, so `S1`'s ledgered start spends `S2`'s cap. The mend moves entry 14 (its `--cap` read against the shared
  ledger, e.g. `--cap 2`, or a ledger file of its own) — a plan revision; nothing patched here. The `S2` start stays
  unspent under the founder's cap; the `S1` spare (`S1-2`) is unspent.
- Entry 15 (the census after `S2`) not run by the round (stopped at 14); fired by hand: `census: 0`.

## What stands green
- The non-leg block, second firing: entries 1–8 green (fmt · clippy · agent-claude unit · root unit · the changed
  integration binaries · the whole integration layer · the whole E2E layer · release-check) — trail
  `gate-2026-11-18-session-folder-trusted-by-viola.json`.
- **`S1` (entry 12) GREEN**, ledgered 2026-11-18T13:13:26Z, build `f058ab7040082253`: step 1 live verify
  `verify-exit: 0`, `trust-record: pass`, `pass-count: 23/23`, `failing: none`, `step1-floor: held`,
  `step1-verified: yes`; step 2 `first-screen: box`, `run-trust: written`, `entry-flag: true`, `sentinel: equal`,
  `keys: none`, `stop: ctrl-c 4`, teardown `census: 0` · `credentials: removed` · `scratch: absent`,
  `user-config: changed-by-others` (the host's own sessions). Whole summary in `evidence/live-starts.md`.
- **Six controls hold** (`evidence/control-1.txt`): trust-nocall · trust-gate (entry 5) · trust-walk · trust-flagonly
  (entry 3) · trust-exact · trust-home (entry 4); 0 timeout.
- No matrix ref written: `matrix#v2-33`'s acceptance reads `S1` against `S2`, and `S2` is not read.

## Recorded red before a re-fire
- Block firing 1, entry 7 (`run --e2e`) red: `contract_diag_schema::diag_line_schema_event_enum_equals_the_log_format_literals`
  (the detail schema's event enum lacked `trust-record`), `diag_lines_from_real_runs_validate` (`left: 20 right: 17`,
  the role-line count), `contract_bounded_readers::boundaries_each_name_a_bounded_read_that_stands` (the witness named
  the fake's renamed walk test). Fixed as companions (below); firing 2 green. Not an identical-tree re-run.

## Files changed
Listed (research): `crates/viola-agent-claude/src/trust.rs` · `src/trust_record.rs` · `src/cmd/run.rs` ·
`src/cmd/verify.rs` · `crates/viola-core/src/obs.rs` · `schemas/diag-line.v1.json` · `src/bin/viola-fake-agent.rs` ·
`src/bin/viola-fake-agent/test_cases_opts.rs` · `tests/support/home.rs` · `tests/run_cli.rs` · `tests/cli_revive.rs` ·
`tests/tui_passthrough.rs` · `crates/viola-e2e/src/rig/mod.rs` · `crates/viola-e2e/src/rig/isolate.rs` ·
`crates/viola-e2e/src/bin/viola-live-rig.rs` · `crates/viola-e2e/tests/rig_rehearsal.rs`.
Recorded (`doc:2026-11-18-session-folder-trusted-by-viola/scope-record` rev 4; `gate.py scope`: clean, companion 3 ·
in-intent 10 · widening 0):
- `tests/contract_diag_schema.rs` · companion · serves `schemas/diag-line.v1.json` · self
- `schemas/diag-detail.v1.json` · companion · serves `crates/viola-core/src/obs.rs` · self
- `tests/contract_bounded_readers.rs` · companion · serves `src/bin/viola-fake-agent/test_cases_opts.rs` · self
- `tests/support/cli.rs`, `tests/chaos_snapshot_replay.rs`, `tests/cli_controls_not_disableable.rs`,
  `tests/cli_fake_agent.rs`, `tests/cli_version_gate.rs`, `tests/cross_exit_causes.rs`, `tests/cross_killed_test.rs`,
  `tests/channel_error_bodies.rs`, `crates/viola-e2e/src/harness/boot.rs`, `crates/viola-e2e/src/harness/supervise.rs`
  · in-intent · serves `src/cmd/run.rs` · self

## Deviations, with their justifications
1. **Every test-side `viola run` / `revive` and the harness's supervised wrappers given a test-owned user home** (the
   10 in-intent lines). The plan isolated only its new cases (step 9), but the start's trust step reaches every
   stamped start, and those starts inherited the test runner's real `HOME`: on the dev host they would lock, read and
   write the runner's real `~/.claude.json` (scratch folders outside the repository are trusted by no record there).
   Tests: `support::home::unmade_user_home` (a path beside the test home, never made → `config-absent`); supervise: the
   session's own `user/` (seeded `{}` for a stamped session, so every booted wrapper now writes there). Witness, by
   marks only (counts, no key or value printed): the real config's project keys under the test scratch roots read
   `1 of 53` before the gates (2026-11-18T12:46:52Z), `1 of 53` after both block firings (13:08:12Z) and `1 of 54`
   after `S1` (13:15Z, the total moved by the host's own sessions); the lock `absent` each time.
2. **`Wrapper::boot_untrusted_as_user` takes `dir` and `env`** beside the plan's `(stamped, name, user_home, extra)`:
   the home-folder case types in the user home, the parent case in a subfolder, the config-dir case sets
   `CLAUDE_CONFIG_DIR` on the wrapper alone. Added `Wrapper::revive_as_user` for the revive case (same body as
   `revive`, the named user home).
3. **The trust cases' folders sit under `/tmp` (`vt-*`), outside any repository**: on CI `target/e2e-home` is inside
   the workspace, so a folder under it keys at the workspace's git root and the home-folder / revive-key oracles could
   not tell folders apart.
4. **`trust_start_read_back_with_an_added_key_is_lost` asserts the one attempt** (`Attempt::Again(Lost)`) and then the
   start's word `already-trusted`: a racer that adds a key leaves the flag `true`, so the next attempt's walk reads the
   folder trusted and writes nothing more — `lost` is the attempt's reading, not the start's. The `trust-exact` control
   holds on it.
5. **The fake's walk test renamed** `record_trusts_reads_its_own_key_in_the_named_file` →
   `record_trusts_walks_from_the_cwd_to_the_git_root`, and the fake's `record_key` removed (no caller left): its old
   oracle «inside a repository only the root's record counts» is false under the CLI's walk (the cwd's own record
   counts).
6. **Existing oracles moved by the new start line** (testing.md 2026-10-04, extended 2026-11-14): `tests/run_cli.rs`
   `run_with_fake_agent_writes_start_and_exit_lines` (the shape), `run_child_exit_status_is_recorded`,
   `run_with_a_missing_program_exits_1_with_internal_error`, `run_appends_to_an_existing_role_file`,
   `run_self_exit_carries_duration_ms`, `run_config_debug_level_keeps_the_key_set`,
   `run_config_malformed_emits_parse_rejected`; `tests/contract_diag_schema.rs` `diag_lines_from_real_runs_validate`.
7. **The 21 floor rows** (`FLOOR_ROWS` in the rig) are copied from the previous chunk's plan (fct-xjfoe4dc, its S0R-3
   step): this plan names the floor but not its ids.
8. **`StartWord::of` maps verify's `removed` / `key-present` to `lost`** (the start's change produces neither);
   `with_flag` re-sets a flag already `true` (unreachable behind the walk's `already-trusted`).

## Decisions & corrections
- The overseer's implement directive (inputs#I3) set the stop rule this outcome follows: «A RED live entry stops
  implement BEFORE the operator pass … A red that moves a plan entry is revised through the phase's door, not patched
  here.»
- Guard refusals, each re-issued once: a `cat` heredoc to a file (→ a Write-tool script run by path) and a `rec.py`
  output piped into `head` (→ run bare).
- A standing learning met: testing.md 2026-10-04 (extended 2026-11-14) «a change to the number of start records moves
  every test that counts the log» — 8 cases moved, found by running the layers.

## Process hygiene
| process | started by | final state |
|---|---|---|
| S1's `viola verify` + 7 `claude` 2.1.325 children, then `viola run rig-s1` + its `claude` | a leg (entry 12) | terminated — rig `census: 0`, round census none left |
| S2 | a leg (entry 14) | none started (`refused: cap` before the spawn) |
| the controls' clone builds and test children | this run (`gate.py control`) | terminated — `clone removed` |
| the block's test children | this run (`gate.py run`) | terminated (no survivor named) |
| host sessions (16 rows: the overseers, the prototype builders, `viola mcp`, a `claude --continue`) | NOT DERIVED — not this run's | left running: the host's own |

`gate.py census --comm claude --comm viola --comm viola-live-rig --comm viola-fake-agent --args
projects/viola/target/debug/viola` (trail `census-2026-11-18-session-folder-trusted-by-viola.json`): 16 running, none
of them this repository's `target/debug/viola` or a 2.1.325 `claude`.

Leftover on the tmpfs: `target/e2e-home/viola-test-YKYwsZ/` (kept by a hand-run `AGENT_RUN_KEEP_FAILED=1` debug of
the config-dir case; its owner process is gone, so the next test home's sweep removes it).

## Next
Revise the plan through `/andromeda-phase` (its pending guard offers the revision): entry 14's cap against the shared
ledger (or its own ledger). Then re-enter implement for `S2` alone (`--live-legs --entry 14`) under the unspent `S2`
start, then the operator pass.
