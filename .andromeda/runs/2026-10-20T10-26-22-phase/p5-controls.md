# P5 controls — chunk 2026-10-20-epoch-5-cleanup-ii

The script is `.andromeda/runs/2026-10-20T10-26-22-phase/p5-controls.sh`, run from the project root, finishing by 2026-10-20
11:00Z (date -u read 11:00:38Z right after it). Each control is a `git clone --shared` of HEAD 844de97682c8 in the script's own `$(mktemp -d)`, removed at its
exit. Every entry was fired AS THE ENTRY: `gate.py run --plan <the plan> --entry {n} --project-root {clone}`, with no
`--run-dir`. Each control fired its entry unmodified first, then with the control's input.

| entry | clone input | reading |
|---|---|---|
| 11 | unmodified | red · `last line '0' ✗ (last: '2')` — the same as the real tree |
| 11 | must-pass: `typed.rs` and `board.rs` cut to 100 lines | green · exit 0 |
| 12 | unmodified | red · `exit 1 ✗ (exit 0)` — the same as the real tree |
| 12 | must-pass: `mod tests {` replaced by `mod tests;` in `ledger.rs` | green · exit 1 |
| 13 | unmodified | green · exit 0 (last line 14) |
| 13 | known-positive: one more `human::refuse(` appended to `src/main.rs` | red · `last line '14' ✗ (last: '15')` |
| 14 | unmodified | red · `exit 1 ✗ (exit 0)` — the same as the real tree |
| 14 | must-pass: `relied_here` renamed in `ledger.rs` | green · exit 1, 0 B |
| 19 | unmodified | red · `exit 0 ✗ (exit 1)` — machete names `serde`, the same as the real tree |
| 19 | must-pass: `serde.workspace = true` dropped from `crates/viola-mcp/Cargo.toml` | green · exit 0 |
| 20 | unmodified (the chunk folder copied in) | green · `guard: clean` |
| 20 | an allowed file changed (`ledger.rs`) | green · `guard: clean` |
| 20 | must-fail: `src/main.rs` edited, plus an untracked `src/new_file.rs` | red · `last line 'guard: clean' ✗ (last: 'guard: outside 2 files')` |

Typed controls, each differing from its entry's `run` by one operand alone (validation.md check 4 (9)), each run as
`/usr/bin/bash -o pipefail -c '{command}'` from the project root:

| entry | control | reading |
|---|---|---|
| 10 | the same `cargo nextest run --workspace --features fake-agent -E 'test(/(^|::)tests::(…)(::|$)/)'` form, its name set `live_endpoint_without_a_snapshot_is_none` | exit 0 · `1 test run: 1 passed, 2979 skipped` — the form selects. nextest prints `test` singular for one case and `tests` for more, so the entry's `9 tests run: 9 passed` atom holds the plural |
| 17 | the same `bash scripts/agent-run.sh run --e2e --filter 'binary(…)'` form over `binary(mcp_tools)` | exit 0 · `"ok":true`, 6 passed, 0 failed — the form selects an existing `mcp_` binary in the E2E layer |
