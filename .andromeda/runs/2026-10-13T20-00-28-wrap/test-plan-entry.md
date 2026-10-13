
## 2026-10-13-interrupted-verify-cleanup — verify's stop table, the Windows matrix count
**Section:** §5 Integration Test Strategy → CLI (the `tests/cli_verify.rs` bullet) · §9 CI Integration → Pipeline structure (the Mutation row)
**Change:**
- `tests/cli_verify.rs` gains `verify_stopped_by_a_signal_removes_only_its_own_dirs_and_exits_1` (`cfg(unix)`, rstest cases `sigint` and `sigterm`). It runs a verify started and not waited (`tests/support/verify.rs` `verify_started`, no `--turn-stop`, a cwd and `TMPDIR` the case owns), signalled by `libc::kill` at the pid it started once the receipt shows its third `cwd` line and first `prompt` line.
- The case asserts: the print probe's, Run A's and Run B's dirs gone while planted foreign dirs stand with their bytes; stdout empty; the exact plain pair; exit 1; no `ledger/stamps.json`; every spawn pair closed; the log ending `process-exit{exit_code:1, detail:"interrupted"}` with no `panic` line, schema-valid.
- Unit cases in `stop.rs`, `typed.rs` and `version_gate.rs` back it. On Windows the console routine is called directly (three cases on `windows-2025`).
- The Mutation row's Windows leg was nine jobs with the root package four; now ten jobs with the root package five, the fifth `viola-cmd-verify-stop` over `src/cmd/verify/stop.rs`.
**Why:** The stop path is new behaviour proven against the fake agent only, with no live start (`inputs#I1`). Real Windows console delivery is owed to "Windows-only live measurements" (the P5 card's row, the operator's word). The matrix line is forced by `tests/contract_windows_mutation_scope.rs`. The root `Instant::now() + WITHIN` count is unchanged at 28 in 20 files (the rule: the pattern under `tests/`): the receipt watch reuses `fake::wait_for`.
**Ref:** .andromeda/runs/2026-10-13T20-00-28-wrap/
