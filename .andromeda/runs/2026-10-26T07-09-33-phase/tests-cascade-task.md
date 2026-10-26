# Task — tests · 2026-10-26-server-verification-before-any-frame

## The chunk
Builds server verification before the first frame in every client that writes a frame to a wrapper's endpoint: the connected server's pid and start time are matched against the `pid` and `started_at` the instance's `snapshot.json` records (server pid from the endpoint itself, Unix peer credentials / Windows pipe server process id; start time from sysinfo; macOS euid + dir only, a recorded gap).
The clients: CLI `send`, `wait`, `last`, `answer`, `pause`, `release`; `viola hook`'s `hook.event` (every event) and `hook.dialog` (`pre-tool-use` · `permission-request`); `viola mcp`'s `send` / `wait` / `last` / `answer` tools over the Tokio client; any client `run`'s start, `revive`, `list` or `verify` holds is read at P3.
On a mismatch, or a check that cannot be made: CLI verbs exit 21 (instance unreachable), MCP tools return `instance-unreachable`, `viola hook` fails open (exit 0, nothing on stdout or stderr, no frame written).
Closes the dated liveness-only exceptions (the first, fourth to eighth) at the wrap; the strict-modes half may stay with "Home and code-bearing file integrity" (a P4 boundary question); an unreadable or newer `snapshot.json` keeps today's outcome (read as absent, exit 21 / fail open) unless the founder rules otherwise, and the move of the five snapshot readers to the classified read is held behind that same ruling.
Owns the Windows CARRY of a `viola hook` reaching an exiting wrapper's pipe within milliseconds of its exit (SessionEnd's direct-append fallback skipped): P3 reads whether a pid + start-time check changes it, else it is named unclosed.

## Reached files
- src/cmd/client.rs · edits the CLI clients' liveness-only pre-check and its snapshot read
- src/cmd/send.rs · edits the `send` frame's pre-frame check
- src/cmd/wait.rs · edits the `wait` frame's pre-frame check
- src/cmd/last.rs · edits the `last` frame's pre-frame check
- src/cmd/answer.rs · edits the `answer` frame's pre-frame check
- src/cmd/pause.rs · edits the `pause` frame's pre-frame check
- src/cmd/release.rs · edits the `release` frame's pre-frame check
- src/cmd/hook.rs · edits the `hook.event` and `hook.dialog` client paths and its two snapshot reads
- src/cmd/hook/statusline.rs · edits its snapshot read (held to the replay ruling)
- src/cmd/run.rs · edits the collision check's snapshot read (held to the replay ruling)
- crates/viola-mcp/src/tools.rs · edits the `send` / `wait` / `last` / `answer` tools' pre-frame check
- crates/viola-channel/src/client.rs · edits the sync client's connect (the server identity read)
- crates/viola-channel/src/tokio_client.rs · edits the Tokio client's connect (the server identity read)
- crates/viola-channel/src/endpoint.rs · edits the endpoint's server pid read (peer credentials / pipe server process id)
- crates/viola-state/src/liveness.rs · edits `live_endpoint` and the pid + start-time match
- crates/viola-state/src/snapshot.rs · re-reads `read_snapshot` and the classified read
- crates/viola-state/src/strict.rs · re-reads the existing strict-modes checks
- .claude/rules/security.md · edits the dated-exceptions list of §Local trust boundary
- viola-0.1.0/verification-matrix.json · edits row `v1-41`

## Needs
- The IPC coverage trigger's case: a server pid / start-time mismatch against `snapshot.json` gives CLI exit 21, MCP `instance-unreachable`, and a hook exit 0 with no body — a negative test per client family.
- The wrapper channel's server-verification pattern in the integration layer: a real endpoint in a temp home, the endpoint read only from `snapshot.json`, an impostor or squatter server bound through the server API (never `try_overwrite`), never a reused endpoint name.
- The viola-channel unit cases for the peer-credential decision with injected euids, extended to the server pid / start-time decision.
- The viola-state liveness classifier: pid or start-time mismatch reads `gone` at any heartbeat age, fed directly with no clock; the heartbeat 5.0 s / 5.1 s boundary row stands beside it.
- The hook fail-open matrix (security sweep): channel failure and now a verification failure each exit 0, empty stdout and stderr, no frame and no event line written; `hook_fail_open.rs` runs in the integration layer.
- The hook connect deadline: the crate-private `CONNECT_DEADLINE` (750 ms) stays below `viola_core::SPINE_DEADLINE`, and the verification added before the frame stays inside the perf rows (`session-start`, `user-prompt-submit`, `stop`, `session-end`, `pre-tool-use`) gated on hyperfine `max`.
- The exit-cause matrix (`cross_exit_causes.rs`): a verification mismatch is a cause of exit 21 whose `--json` document and last `hint:` line name it, no hint shared across causes, no driver-facing hint naming `release`.
- Path 3's killed-wrapper signal (CLI exit 21, MCP `instance-unreachable`) and the chaos case of an endpoint vanishing during `wait`, which must still read unreachable through the new check.
- E1's `send` naming an unwrapped row still exits 21 (`tests/cli_list.rs`).
- The MCP adapter tests in `crates/viola-mcp/tests/handlers.rs`: the `TestWrapper` writes its snapshot through `viola_state::snapshot::write_snapshot` and the handlers use the Tokio client, so the recorded pid / start time must name the serving process; a mismatch case returns `instance-unreachable` with a code-only `isError`.
- Cross-surface parity: for the same unreachable outcome the MCP and CLI payloads agree.
- The multi-os trigger: the OS-branch verification is tested on its own runner — Unix peer credentials, Windows pipe server process id on `windows-2025`, macOS euid + dir only on `macos-latest`; a Linux-only run proves no other OS.
- The cross-user peer remains untestable on hosted runners and is asserted at the peer-credential check boundary only.
- The snapshot-hold case: an unreadable or newer-`v` `snapshot.json` keeps reading as absent (exit 21 / hook fail open) unless the founder rules otherwise; a test of it plants the bad snapshot only by altering bytes the wrapper wrote, never a hand-written one.
- E5 (snapshot corruption → replay recovery): steps 1–3 are owed to this entry while `run`'s collision check calls `read_snapshot`; built here only if the replay ruling lands, and then its two set-against signals (`wheel:"human"`, a rebuilt snapshot) are restated.
- The Windows exiting-wrapper window (test-plan §3 `run` step 2, tests wait for the endpoint to be gone): whether the check changes the SessionEnd direct-append outcome is read, and a test is added only if it does.
- Error sanitization: the new unreachable and fail-open outcomes carry codes and fixed messages only — no pid, path or start time in any `--json` document, MCP `isError` or channel `error.data`.
- The security-negatives suite (`security_negatives_*.rs`): the server-verification control gets its negative there if it has none elsewhere, beside the Windows SQOS identification case it builds on.
- The surfaces' testability rows: viola-channel's "server verification" testable part and the cross-user carve-out; the root bin's exit-code mapping unit cases cover the new cause.

## Standing conventions
- S1 · a new root integration file takes a `<surface>_<topic>.rs` prefix (`cli_`, `hook_`, `channel_`, `chaos_`, `security_negatives_`; `mcp_`, `cross_`, `path_` in viola-e2e) · .andromeda/test-plan-detail/test-strategy.md
- S2 · test functions are snake_case `<subject>_<condition>_<expected>`, table cases rstest `#[case::label]` (viola-e2e a labelled case table, viola-core a literal table) · .andromeda/test-plan-detail/unit-test-strategy.md
- S3 · each test owns a fresh home at a not-yet-existing path under `target/e2e-home`, started wrappers in a `Drop` guard, nothing shared · .andromeda/test-plan-detail/integration-test-strategy.md
- S4 · no sleep for synchronisation; waits key on `events.ndjson` offsets or `viola wait --after`, read through `tests/support/ndjson.rs` · .andromeda/test-plan-detail/test-strategy.md
- S5 · never hand-write `snapshot.json`, `stamps.json` or `budget.json`; a malformed state file only by altering the writer's own bytes · .andromeda/test-plan-detail/test-data-fixtures.md
- S6 · env per child via `Command::env`, `.env_clear()` re-adds `LLVM_PROFILE_FILE`, a test-started `viola` drops `VIOLA_NAME`, `VIOLA_DIR` and `VIOLA_BIN`, path oracles on `resolved(home)` · .andromeda/test-plan-detail/integration-test-strategy.md
- S7 · literal oracles for exit codes and refusal codes, compared by integer and kebab value, never imported from the product or by `Debug` · .andromeda/test-plan-detail/unit-test-strategy.md
- S8 · zero flakes: nextest `retries = 0`, no `#[ignore]` parking, a flake keeps the chunk red · .andromeda/test-plan-detail/quality-gates-coverage-targets.md
- S9 · per-OS coverage gates (lines 85, functions 95, regions 80) with the fixed ignore regex · .andromeda/test-plan-detail/quality-gates-coverage-targets.md
- S10 · after every E2E test the error-sanitization and secret scan and the diag schema conformance check run · .andromeda/test-plan-detail/e2e-test-strategy.md

## Not needed
- E3 `link` / `unlink` and the implicit-link cases: "Explicit session links" stays held and nothing here serves them.
- Path 6 budget governor and `budget.json` reading: the gate is not touched (the statusline's snapshot read is reached only under the replay hold).
- Paste validation, `control-character`, `empty-text` and the typed-text rule of Path 2.
- Recorded `fixtures/claude/` hook, screen, status line and agents fixtures, their hygiene walk and the `viola verify` ledger-probe contract: no CLI behaviour or ledger row is added.
- The `Mcp.<variant>.json` recorded exchange replay: no tool shape changes.
- Mutation testing: run at the epoch boundary through `/andromeda-code-audit`, no per-chunk gate.
- Property and fuzz suites: no parser surface is added.
- Supply-chain gates, cargo deny, the tokio bans and the rmcp feature assertion: no dependency moves.
- Windows listener DACL, remote-client rejection and first-instance admission: landed with 2026-10-25-windows-endpoint-admission.
- Unix socket directory and mode hardening: owned by "Unix endpoint and home hardening".
- The ConPTY vendor verification, sideload and release-check gates.
- `viola list` board layout, row integrity and human-mode SGR output.
- `revive`'s preflight and the `claude agents --json` read.
- Path 4 decision bodies, the dialog hold and the version gate of Path 7.
