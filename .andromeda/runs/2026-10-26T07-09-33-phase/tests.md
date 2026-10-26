# tests extract

## Relevance
relevant — test-plan names the server pid / start-time mismatch as a required negative on every client surface (CLI, MCP, hook), on all three OSes with a macOS variant.

## Constraints
- test-plan §1 Coverage triggers (security-vector-coverage, IPC) requires a negative test for "server pid / start-time mismatch vs `snapshot.json`": CLI exit 21, MCP `instance-unreachable`, hook exit 0 with no body. All three surfaces are in scope, not only the CLI one.
- test-plan §5 Integration (Wrapper channel, detail `integration-test-strategy.md`) requires the mismatch case on Linux and Windows to give CLI exit 21 and a hook exit 0 with no body. Its MCP half is the same impostor case in `cross_exit_causes.rs`: `call_tool("send")` → `is_error:true` with `{"error":"instance-unreachable","detail":null}`. macOS verifies euid + dir only.
- test-plan §6 Exit-cause matrix (detail `suite-exit-cause-matrix`) lists "server verification fails" as its own exit-21 cause. It is a `#[cfg(not(target_os = "macos"))]` same-user impostor listener on the recorded endpoint, plus a `#[cfg(target_os = "macos")]` case that loosens the per-user socket dir to 0755 and asserts exit 21. A same-user impostor inside a valid 0700 dir on macOS is an accepted limitation and is not asserted. Each case asserts the exit code, the `--json` document and a last `hint:` line naming its cause. A hint shared with another exit-21 cause (strict-modes, endpoint gone, unwrapped row) fails the table. No hint names `release`.
- test-plan §1 Coverage triggers (hook stdin) and §6 Security sweep require every `viola hook` fail-open path to exit 0 with empty stderr and no body. A refused verification in the hook client (`hook.event` on every event, and `hook.dialog` for `pre-tool-use` / `permission-request`) is one more such path. The fail-open matrix's `Instant` bound (`< 1.0 s`) applies to it.
- test-plan §10 Performance budgets gates the hook rows (`session-start`, `user-prompt-submit`, `stop`, `session-end`, `pre-tool-use`) on hyperfine `max < viola_core::SPINE_DEADLINE`. The hook's `CONNECT_DEADLINE` (750 ms) must stay below that constant (`connect_deadline_is_below_the_spine_deadline`). A pid + start-time check added before the first frame runs inside that budget. Whether its cost moves the `max` is research's question.
- test-plan §1 Coverage triggers (multi-os-compat) and §9 Matrix builds require OS-branch tests on each runner: named pipe vs Unix socket, and macOS euid + dir-only server verification. test-plan §11 Test Strategy bans treating a Linux-only local run as proof for Windows or macOS.
- test-plan §6 Scenario E5 (detail `scenario-e5-snapshot-corruption-replay-recovery`) names "Server verification before any frame" as the owner of the five other snapshot readers taking the replay, and of steps 1–3. It also names two signals for the building entry to restate (`wheel:"human"` does not hold after a start, and the replay writes no file). The scope holds that move behind the founder's word (CARRY 7). Whether any E5 step is built here is P4's to plan behind that stop rule. Nothing from E5 is assumed landed.

## Patterns to follow
- Impostor and squatter listeners are created through the interprocess server API, never `try_overwrite` (test-plan §5 Module ↔ IPC; §8 Fake servers). The client takes the endpoint only from `snapshot.json` (test-plan §6 Drivers, ipc-internal).
- MCP adapter-level cases drive the transport-free `viola_mcp::Tools` handle against the `viola_channel::Server` test wrapper `TestWrapper` in `crates/viola-mcp/tests/handlers.rs`. Its snapshot is written through `viola_state::snapshot::write_snapshot` and the handlers use the Tokio client. The full stdio path stays E2E with the rmcp client (test-plan §5 MCP; §6 Drivers).
- Table-driven negatives use rstest `#[case::label]` rows (`viola-e2e`: a labelled case table in one `#[test]`). Names follow `<subject>_<condition>_<expected>` (test-plan §2 naming; §4 Conventions).
- A snapshot comes from a real wrapper run, or from the writer itself. A reader's unit case may only alter the writer's own bytes (test-plan §7 Seed strategies; §11 Test Data). How to get a live recorded pid that differs from the listening process without hand-writing `snapshot.json` is research's question.
- Each test gets a fresh home under `target/e2e-home` with a `Drop` guard on started wrappers. Env is set per child with `Command::env`, and `env_clear()` re-adds `LLVM_PROFILE_FILE` (test-plan §5 Setup / teardown; §7 Test data lifecycle; §11 Integration).

## Anti-patterns to avoid
- NEVER sync on `sleep(N)`. Wait on an `events.ndjson` offset, `viola wait --after` or a status field, including in a test of the CARRY 2 exiting-wrapper window (test-plan §11 E2E).
- NEVER use interprocess `try_overwrite` or reuse an endpoint name across tests (test-plan §11 Integration).
- NEVER hand-write `snapshot.json`, `ledger/stamps.json` or `budget.json` in a booted-wrapper test (test-plan §11 Test Data).

## Contract bindings
- tests ↔ harness: test-plan §3 `cleanup` step 4 reads `viola last <name> --json` exit 21 plus the endpoint gone as `endpoint_gone`. `boot` readiness reads `snapshot.json`'s `pid`, `started_at` and `endpoint` (§3 `boot`). Client verification must keep both readings as they are.
- tests ↔ security: the §6 Exit-cause matrix and §6 Security control negatives (Unix socket directory refusal: client verb exit 21, hook exit 0 with no frame; Windows client SQOS) test the security-plan IPC client-side server verification row. The dated exceptions closing is a security-plan amendment and has no matching test-plan row.
- tests ↔ obs: per test-plan §6 Error sanitization, the refusal document and hint carry no absolute path, pid or `Caused by`. The secret scan and the G4 schema check cover any new diagnostics line (obs-plan §9 / `schemas/diag-line.v1.json`).

## Acceptance criteria contributions
- An impostor-listener case on the recorded endpoint gives CLI exit 21 with the `instance-unreachable` document, a distinct last `hint:` line, and `viola hook` exit 0 with empty stdout and stderr and no frame written, on Linux and Windows. The macOS case with a 0755 socket dir gives exit 21 (per test-plan §6 Exit-cause matrix; §5 Wrapper channel).
- The MCP impostor case in `cross_exit_causes.rs`: `call_tool("send")` returns `is_error:true` with `{"error":"instance-unreachable","detail":null}` (per test-plan §5 Wrapper channel).
- `run --perf` hook rows still read `max < viola_core::SPINE_DEADLINE` through `gate --require perf`, and the hook fail-open cases stay under their `< 1.0 s` `Instant` bound (per test-plan §10 Performance budgets).
- The new tests pass on all three CI runners through `viola-harness gate`. Coverage holds at lines ≥ 85 %, functions ≥ 95 %, regions ≥ 80 % per OS, with nextest `retries = 0` (per test-plan §10 Coverage thresholds; §10 Zero-flakiness budget).
