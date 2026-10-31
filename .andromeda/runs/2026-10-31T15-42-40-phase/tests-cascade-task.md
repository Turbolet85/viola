# Task — tests · 2026-10-31-exit-cause-code-catalogue

## The chunk
Gives every cause of exit 21 (`instance-unreachable`) and of exit 1 its own distinct closed code on `--json`, MCP and the process log, carrying no path and no pid, each code matching the human `hint:` line of the same cause.
Builds the exit-cause table `tests/cross_exit_causes.rs`, an rstest `#[case]` table with one negative test per cause, each case asserting the exit code, the `--json` document and a last stderr `hint:` line naming that cause; a hint shared across causes fails the table.
Folds in the freight: `viola revive`'s refusals join the catalogue, `viola revive --json` gains a document for the refusals `run`'s start makes after a passed preflight, the unwrapped-name hint is built, and `viola list` joins the matrix and the security-control table.
Moves the contract of the one `instance-unreachable` document with `detail: null`, on the CLI and on MCP, and updates the test assertions that document reaches; where the code is carried is a P4 choice.
Keeps the exit-code set and every refusal's place unchanged unless P4 names otherwise, and tallies the `verify_stopped_during_the_resume_run_removes_its_dir_and_exits_1` watch on its CI reads.

## Reached files
- tests/cross_exit_causes.rs · edits the exit-cause `#[case]` table, a new file
- crates/viola-core/src/lib.rs
- src/cmd/client.rs · edits `refused_document`, `refuse_untrusted`, the liveness pre-check, the server check
- src/cmd/send.rs
- src/cmd/wait.rs
- src/cmd/last.rs
- src/cmd/answer.rs
- src/cmd/pause.rs
- src/cmd/release.rs
- src/cmd/link.rs
- src/cmd/unlink.rs
- src/cmd/list.rs
- src/cmd/plugin.rs
- src/cmd/mcp.rs
- src/cmd/run.rs · edits its start refusals, `checked_exe`, `refuse_tree`, the collision check
- src/cmd/revive.rs · edits the document after a passed preflight
- src/cmd/verify.rs · edits its refused home tree
- src/human/hints.rs · edits the unwrapped-name hint, each cause's own hint
- src/human/mirror.rs
- crates/viola-mcp/src/result.rs
- crates/viola-mcp/src/tools.rs
- crates/viola-channel/src/client.rs · edits the vanished server read as `instance-unreachable`
- schemas/diag-line.v1.json · edits the `process-exit` `detail` values
- tests/cli_machine_contract.rs
- tests/cli_revive.rs
- tests/cli_list.rs
- tests/cli_links.rs
- tests/cli_verify.rs
- tests/cli_plugin_install.rs
- tests/security_negatives_strict_modes.rs
- tests/security_negatives_server_identity.rs
- tests/contract_error_bodies.rs
- tests/cli_error_bodies.rs
- crates/viola-mcp/tests/handlers.rs
- crates/viola-e2e/tests/mcp_tools.rs
- .andromeda/test-plan.md · edits §6's security-control table, `viola list`'s row
- tests/support/fake.rs · re-reads the 7 s `WITHIN` of `wait_for`, the watch's margin

## Needs
- The Exit-cause matrix suite: `cross_exit_causes.rs` goes from not built to built, with its one negative test per cause, its three assertions per case and the shared-hint failure.
- The matrix's rows that live in other binaries: the server-verification cause in `tests/security_negatives_server_identity.rs`, the frame verbs' strict-modes exit 21 in `tests/security_negatives_strict_modes.rs`, the home-tree refusals of `run` and `verify` in `tests/run_cli.rs` and `tests/cli_verify.rs`, and `revive`'s refusals in `tests/cli_revive.rs`; each built cause is a case of the binary its row names.
- The exit-21 causes, each with its own code: not running, unwrapped name, strict modes, server verification, endpoint lost mid-call.
- The exit-1 causes, each with its own code: already live, stale heartbeat, squatted name, pinned-hash mismatch, script child, strict-modes or socket-directory refusal, internal error; P3 closes this list against the code.
- `viola revive`'s refusals and details (`strict-modes-failed`, `already-live`, `no-session` with its two stderr pairs, `session-live`, `cwd-missing`), each with its own hint, and its `--json` documents on both arms.
- The `revive --json` gap owed here: `batch-script-child`, `pinned-hash-mismatch`, `squatted-name` and a `claude` not on `PATH` as `internal-error` print no document today after a passed preflight.
- The state of `session-live`: the scope's CARRY says not built, while the matrix and `.claude/rules/security.md` describe it as landed; P3 reads which holds, and its row follows.
- The CLI `--json` and exit-code contract: the one document per exit code, today's exit-21 document and exit-1 refusal documents, and every plan clause that names them, which the contract choice amends.
- The MCP surface: today's `instance-unreachable` error payload and refusal text, `isError` payloads holding codes only, and the strict-modes refusal of the MCP frame tools.
- Cross-surface parity: the MCP structured refusal and detail equal the CLI `--json` refusal and detail for the same refusal, once causes are told apart.
- The reply screens and the census, which bind every new code: a word that fails `viola_core::is_code_word` reads as `unknown` or null, a peer fault reads through `viola_channel::sanitised_fault`, every error body and writer stays named in `tests/contract_error_bodies.rs`, and no code holds a path, a pid or upstream text.
- The fixed `-32603` internal-error message and null data: an `internal-error` code must stay inside that rule.
- The process log's closed list: the `process-exit` `detail` values of `schemas/diag-line.v1.json`, the closed unreachable causes already logged in each frame verb's span, and a log form or a named absence for each cause that no line carries today.
- The no-line rule: a refused strict-modes check at `run`, `viola revive`, `viola verify`, the frame verbs, the hook and the MCP frame tools writes no line, and P3 re-verifies that mechanism and `Refusal::StrictModes.detail()` at HEAD; the exits by surface are 1 for `run`, `mcp`, `revive` and `verify`, 21 for other verbs and 0 for the hook.
- Hints: each cause's own last `hint:` line in human mode, the unwrapped-name hint and `e1_send_naming_an_unwrapped_row_is_unreachable`, which proves only the exit 21 today; no hint on any row names `viola release`.
- The surfaces joining the table or named outside it by a reason: `viola list` (exits 0, 21, 1, its exit 21 the whole-board refusal), `viola plugin install`'s exit 21, and `viola mcp`'s refused home check (exit 1 with both streams empty).
- The security-control table's `viola list` row, and the capabilities `verification-matrix.json#v1-18` and `verification-matrix.json#v1-39`, the second claimed with its gap closed.
- Path 1's refusals as table cases: a duplicate run, a squatted endpoint, a tampered pinned exe and a refused home tree, each exit 1, with the checks that run before a log opens staying before it.
- How those cases must be set up: a squatter is bound first through the server API and never by `try_overwrite`, the endpoint comes only from `snapshot.json`, a tampered pinned copy goes through the carve-out for corrupting a file the product wrote, and a server-identity impostor through the carve-out for its test-built `snapshot.json`.
- The stale-heartbeat case: the chaos SIGSTOP form, the 5 s staleness, the name refused to a new `run`, and the carve-out that lets a test block on a product deadline only when the verdict is the product's own and the deadline is clock-driven in a unit case.
- The endpoint-lost case: Path 3's killed wrapper and the chaos case of an endpoint vanishing during `wait`, read at both the CLI and MCP.
- The script-child case: a `.cmd` or `.bat` child refused with exit 1, driven on the Windows runner, and the chaos case that holds it; the `internal-error` cause's `claude` absent from a test-scoped `PATH`.
- The server check's OS bounds: Linux and Windows cases only, the macOS case staying the recorded gap of the route entry "Unix endpoint and home hardening"; the socket directory's refusal takes a code only if the code holds it today.
- The bounds the code must stay inside: the typed exit set 0, 1, 2, 10–14, 20, 21, the root bin's exit-code mapping unit cases, and `hook`'s contract (exit 0, no body, no stderr, `hook-decision` detail its only cause record), which the fail-open matrix holds.
- The new file's place in the layers: the `cross_` prefix and its layer filterset as the root `tests/` file the scope names, and rstest being available to root tests and not to `viola-e2e`; plus the watch's 0/3 green tally, with no bound moved.

## Standing conventions
- S1 · Each new test takes its own fresh tempfile home at a not-yet-existing path under `target/e2e-home`, with started wrappers in a `Drop` guard and the Windows x64 companion seeding as written · §5 Setup / teardown lifecycle, .andromeda/test-plan-detail/integration-test-strategy.md
- S2 · Test functions and files follow `<subject>_<condition>_<expected>` and `<surface>_<topic>.rs`, and each rstest case name is a readable label · §2 naming conventions and §4 Conventions
- S3 · Root test files share `tests/support/` and copy none of it, and a reader of a live-appended file goes through `tests/support/ndjson.rs` · §2 Directory pattern
- S4 · A test-started `viola` drops `VIOLA_NAME`, `VIOLA_DIR` and `VIOLA_BIN`, env is set per child with `Command::env`, `.env_clear()` re-adds `LLVM_PROFILE_FILE`, and a path oracle compares `resolved(home)` · §5 CLI pattern and §11 Integration
- S5 · Expected exit codes and refusal values are literals in the test, never imported from the product, asserted as the integer and the serde kebab-case value, never by `Debug` string · §11 Unit
- S6 · No sleep for synchronisation, waits key on logged events or a status field, and nothing parses the rendered child screen · §11 E2E and §2 Agent-runnable invariants
- S7 · On-disk product state comes only from real `viola` runs, a test-made home goes through `viola_state::fs::create_private_dir`, and `stamps.json`, `snapshot.json` and `budget.json` are never hand-written · §7 Seed strategies and §11 Test Data
- S8 · Each case sits at the lowest layer that can hold it, and §6 asserts only cross-process behaviour · §11 Test Strategy and §2 Test pyramid
- S9 · Every OS-specific branch compiles and is tested on its own runner, and a Linux run proves nothing for Windows or macOS · §11 Test Strategy and §9 Matrix builds
- S10 · No retries, no `#[ignore]` as a parking place, and a flake keeps the chunk red until its root cause is fixed · §10 Zero-flakiness budget
- S11 · New code stays inside the per-OS coverage gates of 85 lines, 95 functions and 80 regions, with no threshold lowered and no ignore regex widened · §10 Coverage thresholds and §11 CI
- S12 · Each new or changed process-log line validates against its schema and holds no `Cookie` or stripped-variable value, and a failure reports by place and class, never the bytes · §6 Schema conformance and Error sanitization

## Not needed
- Paths 5, 6 and 7's wheel, budget and version-gate flows and their exits 10 to 12: those causes are already distinct and not re-opened.
- Path 2's paste, confirmation matcher and local-command decision, with their `not-delivered` reasons: exit 10 to 14 causes, not re-opened.
- Path 4's dialog flow, decision bodies and their insta snapshots: the hook's contract is unchanged and no decision body moves.
- Scenario E2's `CLAUDE*` strip and the identity floor: no environment handling changes.
- `cross_list_rows.rs`, the `viola list` board's layout and the `claude agents --json` row enrichment: only `list`'s refusal joins this chunk.
- The fake agent's scripts and modes, and the contract suite's byte compare of its hook stream with the recorded hook payloads: no hook payload moves and nothing is re-recorded.
- The ledger-probe rows and the multi-version replay of fixture sets: no ledger row or CLI version changes.
- The property suite and the cargo-fuzz targets: no parser changes.
- The mutation gate and `windows-mutants.yml`: the plan holds no chunk gate for mutation, and a mutation run is the boundary audit's.
- Workflow authoring rules (SHA pins, permissions, concurrency, the cache, artifact uploads): the chunk adds no CI job and edits no workflow.
- The harness contract (session record, `boot`, `cleanup`, the `gate --require` grammar): no harness command changes.
- `scripts/conpty-vendor.sh`'s verification and the sideload's own fail-open fallback: neither is a cause of exit 21 or exit 1.
- Scenario E4 and the web racks: the frontend stays deferred to 0.2.0.
