# Remove-the-guard — `binary(contract_windows_mutation_scope)` (plan step 2)

Host: the Linux dev host, 2026-10-04. Command each time:
`CARGO_TARGET_DIR=target/harness cargo nextest run -p viola --features viola/fake-agent --test contract_windows_mutation_scope`
(the gate entry's harness form ran afterwards, see the implement report).

## First green (before the neutralisation)
`3 tests run: 3 passed, 0 skipped` — exit 0. The guard passed on its first write; there is no earlier
red of the guard itself (P5's baseline is `artifact-missing`: the binary did not exist on the untouched tree).

## Neutralised — one listed file dropped from the matrix
Edit: `.github/workflows/windows-mutants.yml:25`, the `viola-pty` item's `files:` line, read back after the edit as
`files: crates/viola-pty/src/lib.rs` (`crates/viola-pty/src/sideload.rs` removed — a whole-file Windows module gated
only at its `mod` line, `crates/viola-pty/src/lib.rs:18-19`, the case a per-file grep misses).

Reading (`--no-fail-fast`): exit 100,
`3 tests run: 2 passed, 1 failed` —
- `FAIL windows_gated_sources_equal_the_workflow_scope_both_ways`, panicked at
  `tests/contract_windows_mutation_scope.rs:319:5` with the problem list `"unlisted: crates/viola-pty/src/sideload.rs"`;
- `workflow_tool_pins_equal_the_ci_test_job_line` and `workflow_trigger_is_dispatch_only_without_inputs` PASS (not
  their subject).

## Restored — insertions only
The same line read back as `files: crates/viola-pty/src/lib.rs crates/viola-pty/src/sideload.rs`.
`git diff --no-index --word-diff=porcelain <red copy> .github/workflows/windows-mutants.yml` — its only changed token:
`+crates/viola-pty/src/sideload.rs` (no `-` token).

Reading: exit 0, `3 tests run: 3 passed, 0 skipped`.
