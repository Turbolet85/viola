# Classified — tests · integration-test-strategy.b1

## Answers
integration-test-strategy#1.1 · yes · direct crate-API calls with shared rstest fixtures; the new root `tests/cross_exit_causes.rs` is an rstest `#[case]` table, and the scope names rstest for root tests
integration-test-strategy#1.2 · yes · the concurrent-append rule on `run-<name>.ndjson` names a colliding second `run`; `run.rs`'s collision check is an edited unit the duplicate-run table case reaches
integration-test-strategy#1.3 · yes · the fake `claude` on a test-scoped PATH and the Windows `.cmd` shim fixture; the scope names the script-child case and the `internal-error` case with `claude` absent from PATH
integration-test-strategy#1.4 · yes · the squatter bound first through the server API, never `try_overwrite`, and the socket-directory and pipe-DACL checks; the scope names the squatted-endpoint case and the server check
integration-test-strategy#1.5 · yes · the `.cmd` child refusal (`cfg(windows)`) is the behaviour the scope's script-child case names; the resize and ConPTY clauses are not reached
integration-test-strategy#1.6 · yes · the hook's fail-open contract (exit 0, no body, no stderr) and the impostor rows of `tests/hook_fail_open.rs`, which the server-verification cause must keep
integration-test-strategy#2.1 · no · the placed-copy exec witness in `viola-state` `fs.rs`; the chunk edits no placement code, and the pinned-hash case corrupts the copy after the product's write
integration-test-strategy#3.0 · no · standing S1 · each test takes a fresh tempfile home under `target/e2e-home`, wrappers held in a `Drop` guard, Windows x64 companion seeding
integration-test-strategy#4.1 · yes · the MCP impostor and widened-instance cases with `{"error":"instance-unreachable","detail":null}` (the document the chunk moves), and the server pid-mismatch exit 21 with hook exit 0
integration-test-strategy#4.2 · yes · the strict-modes refusals of `run`, `viola revive` (its `strict-modes-failed` document), `verify` and the frame tools, each with the no-line rule and the `instance-unreachable` document
integration-test-strategy#5.1 · yes · the root `cli_*.rs` `--json` per-exit-code documents, and the `viola list`, verify, plugin install, links and machine-contract files the chunk edits or joins to the matrix
</content>
