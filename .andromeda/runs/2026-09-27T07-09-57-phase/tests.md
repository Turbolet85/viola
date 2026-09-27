# tests extract

## Relevance
Relevant. This chunk lands the `viola-channel` entity and its ipc-internal surface. Test-plan §1, §4, §5 and §6 already name its test cases, and several of them were deferred to "Wrapper channel" (see the amendment history below).

## Constraints
- **Unit cases (test-plan §4, viola-channel).** Required:
  - frame codec tests at `MAX_FRAME` − 1, `MAX_FRAME` and `MAX_FRAME` + 1;
  - JSON-RPC error mapping for -32700 / -32600 / -32601 / -32602 (with `data:{supported,wrapper}`) / -32603 (exactly `"internal error"` with `data:null`);
  - FNV-1a endpoint golden vectors: the Windows `\\.\pipe\` form, the Unix `$XDG_RUNTIME_DIR/viola/`, `$TMPDIR/viola/` and `/tmp/viola-<uid>/` forms;
  - the peer-credential decision function, tested with injected `(peer_euid, own_euid)`;
  - a `#[cfg(windows)]` check that `GetHandleInformation` reports `HANDLE_FLAG_INHERIT` clear on both the server and client pipe handles. This is the only Windows non-inheritance evidence (test-plan §12 open question).
- **Integration against a real endpoint (test-plan §5, Module ↔ IPC and "Wrapper channel" cross-module patterns).**
  - A real endpoint in a temp home, with frames checked by jsonschema 0.57.0 validators built from the product's schemars schemas.
  - A frame over `MAX_FRAME` must get -32600 and the socket must close (the next read returns 0 bytes). The scope's "refused, never truncated-and-parsed" is necessary but not sufficient.
  - A higher `params.v` must get -32602.
  - Unix: socket dir 0700 and file 0600, read back with `std::fs::metadata`.
  - §5 also requires reading back the Windows pipe SDDL `D:P(A;;GA;;;<user-SID>)(A;;GA;;;SY)`. That collides with the scope's open P4 fork (SDDL is Epoch 6). Whether the §5 read-back lands now or is amended to follow Epoch 6 is P4's decision.
- **Property suite and fuzz (test-plan §6 Property suite, §2 Property-based row).**
  - A proptest 1.11.0 property on ndjson framing at `MAX_FRAME` ± 1, `cases: 512`, with `proptest-regressions/` committed.
  - A cargo-fuzz target `fuzz/fuzz_targets/<parser>.rs` in the separate `fuzz/` workspace, using `arbitrary` 1.4.2 inputs and a seeded `fuzz/corpus/<parser>/`. It joins the corpus-replay gate (test-plan §9 Fuzz replay row). This is the first of the "seven parser targets join as they land".
- **Windows client SQOS (test-plan §6 Security control negatives).**
  - A `security_negatives_*.rs` case against the viola client: a test-owned same-user pipe server calls `ImpersonateNamedPipeClient`, and `TokenImpersonationLevel` must read `SecurityIdentification`, never `SecurityImpersonation` or higher.
  - The open recipe is already pinned by `tests/channel_sqos_open.rs` (test-plan §6 Security control negatives). Whether the chunk's client reuses that exact open is research's question.
- **Start sequence and squat (test-plan §6 Path 1, steps 4–5 and verification; §6 Exit-cause matrix).**
  - The snapshot carries `endpoint` "from Wrapper channel on".
  - A squatted endpoint makes `run` exit 1. The squatter is created first through the interprocess server API on `viola_channel` endpoint name(`builder`, H2).
  - Each exit-1 cause (squatted, already live) needs its own `hint:` line in `cross_exit_causes.rs`. A hint shared across causes fails the table.
  - The same section requires exit 21 when the endpoint is gone after `cleanup`.
- **Harness readiness and cleanup (test-plan §3 `boot` Readiness, §3 `cleanup` step 4 and Verification).**
  - Readiness adds the snapshot `endpoint` once this chunk binds one.
  - Cleanup step 4 requires the recorded endpoint to be unconnectable (interprocess connect → NotFound; the Unix `.sock` path absent).
  - `endpoint_gone` is `null` "until their surfaces exist" (test-plan §3 `cleanup` Verification). Whether this chunk flips it to a real check is the question the amendment must settle.
- **Sync-crate and log gates (test-plan §9 Lint and Supply-chain rows; §3 Log format).**
  - `viola-channel` joins `scripts/sync-crates.txt`, which drives the Lint `cargo check` (no `--features`) and the sole-root `deny-sync.toml` tokio ban.
  - Process-log `channel-request` / `channel-response` lines must carry `corr` = the JSON-RPC request `id`, copied unchanged and never renamed.
  - A null `corr` is written as key absence (test-plan §3 Log format, Null encoding).

## Patterns to follow
- **Test locations and naming** (test-plan §2 Test directory + naming conventions): inline `#[cfg(test)] mod tests`; crate-level `crates/viola-channel/tests/<topic>.rs`; root sync E2E as `tests/channel_<topic>.rs`; function names `<subject>_<condition>_<expected>`.
- **Tables and oracles** (test-plan §4 Fixture pattern; §11 Unit): rstest `#[case::readable_label]` tables for the error-code and framing matrices, with expected codes and vectors written as literals and never imported from the product.
- **Endpoint and wrapper lifecycle** (test-plan §6 Drivers table, ipc-internal row; §5 Setup / teardown):
  - read the endpoint only from `snapshot.json`;
  - hold long-lived `viola run` wrappers in a `Drop` guard (`kill()` then `wait()`);
  - use a fresh not-yet-existing home under `target/e2e-home/` with its `owner.json` record (test-plan §3 Test data bootstrap, Cleanup).
- **Drivers** (test-plan §5 Driver(s)): the `viola-channel` sync client for well-formed traffic; a raw interprocess 2.4.4 `local_socket::Stream` for malformed and oversize frames, and for raw clients that skip client-side checks.
- **Fault seams** (test-plan §8 What to mock, §11 Mocking): process and file operations plus mockall doubles on seam traits only. Per-child env goes through `Command::env`.

## Anti-patterns to avoid
- Using interprocess `try_overwrite`, reusing an endpoint name across tests, or taking the endpoint from anywhere but `snapshot.json` (test-plan §11 Integration).
- `sleep` for synchronisation, retries, `#[ignore]` as a parking place, or a fix-by-reasoning without a root cause. A flake keeps the chunk red (test-plan §10 Zero-flakiness budget; §11 E2E and CI). This applies directly to the viola-pty resize watch (fold item 9) and the `harness_session_boots_reports_logs_and_tears_down` recurrence watch (fold item 11).
- Adding a failpoint crate or any test dependency that pulls tokio into the sync `viola-channel` (test-plan §11 Mocking).

## Contract bindings
- **tests ↔ obs log format** (test-plan §3 Log format; obs-plan §3 D-10 `conn`, D-30).
  - tests owns the harness-grepped fields.
  - obs's `schemas/diag-line.v1.json` making `corr` `required` for corr-bearing events (fold item 2) is checked by the tests-owned body behind G4 (test-plan §6 Schema conformance; §3 `schema-check`).
  - The `conn` / `srv_conn` additions are obs's to define. The tests side must not rename or remove them.
- **tests ↔ obs redaction** (test-plan §6 Error sanitization and secret scan, Canary): the fixed-`Display` `ChannelError` and the `#[derive(Redact)]` payloads must keep channel `error.data` free of paths, serde paths and `Caused by`. The canary must never reach home-level `diagnostics/*.ndjson`.
- **tests ↔ security.**
  - The SQOS negative (test-plan §6) binds to security.md §Local trust boundary.
  - The squat `run` exit 1 (test-plan §6 Path 1 / Exit-cause matrix) binds to the security-plan start-order refusal.
  - The Windows SDDL read-back (test-plan §5) binds to the Epoch 6 admission entry. That is the open P4 boundary fork.
  - The sync-crates tokio ban (test-plan §9 Lint and Supply-chain) binds to security Vector 9.
- **tests ↔ arch.** Snapshot `endpoint` feeds harness `boot` readiness and `cleanup` step 4 (test-plan §3). The exclusive-bind arbiter of two concurrent starts is observable only as `run` exit 1 (test-plan §1 Critical paths, `run` start sequence).

## Acceptance criteria contributions
- `cargo nextest run -p viola-channel` and the root `channel_*` tests pass on all three OSes. They cover (per test-plan §4 viola-channel; §5 Wrapper channel):
  - framing at `MAX_FRAME` ± 1;
  - -32600 plus socket close on an oversize frame;
  - -32602 with `data:{supported,wrapper}` on a newer `v`;
  - -32601 on an unknown method;
  - -32603 exactly `"internal error"` / `null`;
  - the endpoint golden vectors;
  - Windows `HANDLE_FLAG_INHERIT` clear.
- The proptest framing property runs at `cases: 512` with `proptest-regressions/` committed, and `fuzz/fuzz_targets/<parser>.rs` exists with a seeded `fuzz/corpus/<parser>/`. `agent-run run --fuzz-replay` followed by `gate --require fuzz-replay` is green (per test-plan §6 Property suite; §9 Fuzz replay).
- The Windows `security_negatives_*.rs` SQOS case reads `SecurityIdentification`. A squatted endpoint makes `viola run` exit 1 with its own `hint:`. On Unix, the fake-agent `fds` receipt shows only 0/1/2 plus the PTY slave (per test-plan §6 Security control negatives; §6 Path 1 / Exit-cause matrix; §6 E2).
- Per-OS coverage is lines ≥ 85, functions ≥ 95, regions ≥ 80. The two-leg mutation union has 0 missed and 0 timeout, with `unviable <= caught` on each leg. `pre-push` is green before the pre-CI commit (per test-plan §10 Coverage thresholds and Mutation gate).

## Relevant amendment history
- **2026-09-25-security-prerequisites.** Pinned the SQOS `CreateFileW` + `FILE_FLAG_OVERLAPPED` open recipe and its no-SQOS control in `tests/channel_sqos_open.rs`, ahead of the client. Recorded that the viola-client `security_negatives_*.rs` SQOS case "lands with `viola-channel`, its first consumer". Also added the `test-only-rust-delta` mutation verdict, which matters if a pass touches only test targets.
- **2026-09-25-pty-wrapper-on-windows.** Re-pinned E2's Unix fds-only half to "Wrapper channel" through its route CARRY. On Windows, non-inheritance stays a product unit fact. This is fold item 6.
- **2026-09-27-instance-state-and-start-order.** Staged readiness (`process-start` lines, then snapshot `pid` / `started_at` / `child_pid`, with `endpoint` explicitly left to "Wrapper channel", then heartbeat).
  - Introduced `owner.json` homes and the gone-owner sweep.
  - Set `AGENT_RUN_KEEP_*=0` under `run --mutants`.
  - The chunk's readiness and Path 1 wording (test-plan §3 `boot`, §6 Path 1) will need the endpoint half made concrete.
- **2026-09-24-three-os-ci-headless-harness-skeleton (interim cleanup).** Made `endpoint_gone` `null` until the endpoint surface exists. This chunk is that surface, so expect an amendment to test-plan §3 `cleanup` Verification.
- **2026-09-24-quality-gates.** Seeded the fuzz pipeline with `viola_name` as the pre-parser target, with "the seven parsers" joining as they land. Also set the separator-agnostic coverage regex. The channel framing target is the first real parser target.
- **2026-09-24-supply-chain-and-workflow-gates.** Made the Lint `cargo check` and the sole-root tokio ban read `scripts/sync-crates.txt` (an empty list fails). `viola-channel` joins by being listed (fold item 1).
- **2026-09-26-local-linux-pre-push-gate.** Defined the `pre-push` stages: `linux-tests` runs coverage on Linux only, and `windows-leg` runs mutation only. Fold item 10 (a Windows llvm-cov `test` stage) is expected to amend the test-plan §3 `pre-push` stage list and Closed enums.
- **2026-09-24-observability-gates.** Declared `schema-check` (the G4 body). The fold-item-2 negative test (a corr-bearing line without `corr` is rejected) runs through it.
