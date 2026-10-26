# tests · Test Strategy — category read

## Carried
- (.andromeda/test-plan-detail/test-strategy.md:10) The new listener assertions (remote-reject mode read-back, first-instance property) must land as inline unit tests that run on every OS, including the Windows CI leg.
  > Inline `#[cfg(test)] mod tests` in all 6 crates plus the root bin. rstest 0.27.0 `#[case]` tables cover the refusal-order matrices, the exit-code table (0/1/2/10–14/20/21) and the per-`<cli-version>` fixture replay. Runs on all 3 OSes, except the two real-cargo-mutants harness tests, compiled out on macOS (§10 Mutation gate).
- (.andromeda/test-plan-detail/test-strategy.md:13) The wrapper-channel contract's Windows admission is asserted at integration level over a real `interprocess` endpoint and a real tempfile home, never a mocked pipe.
  > Required. Every arch Standard Contract (Wrapper channel, On-disk, Hooks, CLI, MCP) has a crate-level `tests/` suite, using a real `interprocess` endpoint, a real tempfile home, and assert_cmd against `CARGO_BIN_EXE_viola`.
- (.andromeda/test-plan-detail/test-strategy.md:28) The chunk carries no mutation acceptance criterion: mutation runs only at the epoch boundary or on demand.
  > No per-chunk, pre-push or CI mutation gate.
- (.andromeda/test-plan-detail/test-strategy.md:39) Unit tests for `server.rs` / `server/win.rs` / `endpoint.rs` must sit inline at the bottom of the source file they test.
  > **Unit tests:** inline `#[cfg(test)] mod tests` at the bottom of each source file.
- (.andromeda/test-plan-detail/test-strategy.md:40) A new crate-level integration test for the listener must live under `crates/viola-channel/tests/<topic>.rs`.
  > **Crate-level integration tests:** `crates/viola-<area>/tests/<topic>.rs`.
- (.andromeda/test-plan-detail/test-strategy.md:41) Root tests touched for the squatted-name start refusal must keep the root suite naming and share `tests/support/` helpers.
  > **Sync (tokio-free) E2E suites:** root `tests/{cli,hook,tui,channel,chaos,contract}_<topic>.rs`, sharing helpers in `tests/support/{home.rs,outer_pty.rs,fake.rs,events.rs,cli.rs,ndjson.rs}`.
- (.andromeda/test-plan-detail/test-strategy.md:41) No new or edited root test may copy the reader, wait, guard or result struct out of `tests/support/`.
  > No root test file outside `tests/support/` defines its own copy of the reader, the wait, the guard or the result struct.
- (.andromeda/test-plan-detail/test-strategy.md:57) The IPC vector (V1) this chunk closes on Windows is traced to integration + E2E coverage at §5 Module ↔ IPC and §6 Scenario 1.
  > | IPC vector (V1) | integration + E2E | §5 Module ↔ IPC; §6 Scenario 1 |
- (.andromeda/test-plan-detail/test-strategy.md:69) The Windows-only branches are verified through the CI matrix, so the `windows-2025` leg is the proof surface.
  > | multi-os-compat | CI matrix | §9 Matrix builds |
- (.andromeda/test-plan-detail/test-strategy.md:74) The folded CARRY (case_4 red under plain `cargo test`) falls under the no-flakes discipline trigger, which applies at every layer.
  > | discipline (no screen parsing / no blanket approval / no flakes) | all | §6 Scenario 4 negative; §11 |
- (.andromeda/test-plan.md:46) Remote pipe clients and the DACL's effect on a foreign principal cannot be driven on hosted runners; item (c) must assert at the boundary the plan names or stay a Windows-only live measurement.
  > partially testable: same-user framing, dispatch, refusal order, server verification and the squatted-name refusal are testable; the cross-user parts (a peer of another euid, remote pipe clients, the Windows pipe DACL) cannot be driven on hosted runners and are asserted at the peer-credential check boundary.
- (.andromeda/test-plan.md:88) A second-user squat or connect stays in the untestable zone; the chunk must not claim it as CI-asserted.
  > Cross-OS-user attacks (a second local user squatting or connecting to the endpoint, reading the home). Hosted runners provide one account (security excerpt, Vectors 1 and 7).
- (.andromeda/test-plan.md:185) The IPC negative set requires the squatted endpoint to refuse `run` with exit 1, unchanged.
  > squatted endpoint → `run` exit 1
- (.andromeda/test-plan.md:187) The IPC negative set requires no inherited channel handle in the child; the non-inheritance assertion must stay.
  > no inherited channel handles in the child
- (.andromeda/test-plan.md:250) Named-pipe and Windows DACL OS-branch tests are required on the Windows runner.
  > full workspace and fake-agent E2E on all three runners. OS-branch tests cover named pipe vs Unix socket, ConPTY vs openpty, Windows DACL / strict-modes vs Unix modes, and macOS euid + dir-only server verification.
- (.andromeda/test-plan.md:305) The CARRY's fix must not rely on retries or a sleep for synchronisation.
  > no nextest retries (`retries = 0`) and no `std::thread::sleep` / `tokio::time::sleep` used for synchronisation
- (.andromeda/test-plan.md:309) Every new endpoint test binds in its own fresh tempfile home.
  > every test creates its own `--home` via tempfile 3.27.0
- (.andromeda/test-plan.md:320) A new root integration binary for the channel takes the `channel_` prefix (or the existing layer prefix) so nextest filtersets select it.
  > Rust integration binaries use a `<surface>_<topic>.rs` prefix so nextest filtersets can select layers: `cli_`, `hook_`, `tui_`, `channel_`, `chaos_`, `contract_` in root; `path_`, `mcp_`, `cross_` in `viola-e2e`.
- (.andromeda/test-plan.md:323) New test functions follow the `<subject>_<condition>_<expected>` naming.
  > General pattern: snake_case `<subject>_<condition>_<expected>`, for example `send_with_esc_refuses_control_character` or `hook_oversize_stdin_exits_zero_silent`.
- (.andromeda/test-plan.md:349) The CARRY's case must pass under plain libtest `cargo test` as well as nextest, since both runners are the framework.
  > **Framework:** cargo test (libtest, current Rust stable pinned in `rust-toolchain.toml`, ≥ 1.96; MSRV 1.96, edition 2024), executed through cargo-nextest 0.9.146 (process-per-test, JUnit).
- (.andromeda/test-plan.md:376) `viola-channel`'s unit layer owns Windows handle non-inheritance; the new listener-property unit cases sit beside it.
  > **viola-channel:** the ndjson frame codec at the `MAX_FRAME` boundary, the JSON-RPC error mapping, the FNV-1a endpoint golden vectors, the peer-credential decision with injected euids, and Windows handle non-inheritance.
- (.andromeda/test-plan.md:411) The pipe's DACL and mode read-back use windows-sys 0.61.2 as the integration driver.
  > **Windows DACL read-back:** windows-sys 0.61.2.
- (.andromeda/test-plan.md:421) The squatter is created first through the server API, never `try_overwrite`, and the DACL is compared in canonical form, never the literal set.
  > a real endpoint bound in a temp home with frames validated by jsonschema, the squatter created first through the server API (never `try_overwrite`), and the Unix socket modes and the Windows pipe DACL read back — the DACL compared in its canonical form, never the literal it set.
- (.andromeda/test-plan.md:476) Cross-OS-user peers are covered only by the peer-credential unit test and same-user SDDL read-back.
  > Cross-OS-user peers. Covered by the peer-credential boundary unit test (§4) and same-user SDDL read-back (§5).
- (.andromeda/test-plan.md:488) Path 1 keeps the squatted-endpoint exit-1 signal.
  > a duplicate run, a squatted endpoint and a tampered pinned exe each exit 1
- (.andromeda/test-plan.md:606) A Windows control that had no negative test (remote-client rejection) gets one in the `security_negatives_*.rs` suite.
  > **Security control negatives** (`security_negatives_*.rs`): one negative test per security-plan control that had none
- (.andromeda/test-plan.md:724) The fake squatter is a real listener bound through the interprocess server API before `run`.
  > the interprocess server API binds the squatter before `run`
- (.andromeda/test-plan.md:767) The `windows-2025` matrix leg is the named-pipe and DACL / SDDL read-back runner.
  > `windows-2025` (ConPTY, named pipe, DACL / SDDL read-back, `.cmd` refusal)
- (.andromeda/test-plan.md:813) `#[cfg(windows)]` listener code must meet the coverage gates on the Windows job itself.
  > Gates are enforced per OS job with `cargo llvm-cov nextest … --fail-under-lines 85 --fail-under-functions 95 --fail-under-regions 80`. Per-OS gating is deliberate: `#[cfg(windows)]` / `#[cfg(unix)]` code compiles only on its own runner.
- (.andromeda/test-plan.md:851) The CARRY's once-red case keeps the chunk red until its root cause is fixed inside this chunk; `#[ignore]` is barred.
  > If a test flakes once, it gets quarantined immediately and fixed at the root cause, never with a retry-once budget. For viola, quarantine means the chunk stays red until the root cause is fixed within that chunk. `#[ignore]` is not allowed as a parking place.
- (.andromeda/test-plan.md:868) Listener properties are asserted at the lowest layer that can hold them, not at E2E.
  > NEVER assert at E2E a case that a lower layer can hold.
- (.andromeda/test-plan.md:873) A Linux dev-host run is no proof of the Windows pipe behaviour; acceptance reads the Windows runner.
  > NEVER treat a Linux-only local run as proof for Windows or macOS. OS-branch code is verified only on its own runner.
- (.andromeda/test-plan.md:889) Tests never use `try_overwrite` nor reuse an endpoint name across tests.
  > NEVER use interprocess `try_overwrite`, or reuse an endpoint name across tests. Take the endpoint only from `snapshot.json`.
- (.andromeda/test-plan.md:919) The CARRY may not be fixed by serialising tests.
  > NEVER serialise tests (`max-threads = 1`): isolation comes from the per-test home.
- (.andromeda/test-plan.md:925) No retry budget may green the CARRY's case.
  > NEVER set nextest `retries` above 0.

## Bindings
- **Integration over a real endpoint ↔ body §5 Module ↔ IPC**: the category's real-`interprocess`-endpoint rule (.andromeda/test-plan-detail/test-strategy.md:13) is realised by the squatter-first, canonical-DACL read-back pattern (.andromeda/test-plan.md:421) with windows-sys as driver (.andromeda/test-plan.md:411).
- **IPC vector V1 ↔ security-plan §Authentication & Authorization (IPC access control, Windows)**: the trace row (.andromeda/test-plan-detail/test-strategy.md:57) covers the negatives the body lists (.andromeda/test-plan.md:185, .andromeda/test-plan.md:187); the security-plan clause is the control under test.
- **Untestable cross-user zone ↔ scope item (c)**: remote-client and foreign-principal DACL effects are not drivable on hosted runners (.andromeda/test-plan.md:46, .andromeda/test-plan.md:88), so item (c) asserts at the boundary (.andromeda/test-plan.md:476) or is owed to the Windows-only live measurements route entry.
- **multi-os-compat ↔ body §9 Matrix / §10 per-OS coverage**: the trace row (.andromeda/test-plan-detail/test-strategy.md:69) binds to the `windows-2025` leg (.andromeda/test-plan.md:767) and per-OS gates (.andromeda/test-plan.md:813).
- **No-flakes discipline ↔ body §10 Zero-flakiness / §11 CI**: the trace row (.andromeda/test-plan-detail/test-strategy.md:74) binds the folded CARRY to root-cause fixing (.andromeda/test-plan.md:851) under both runners (.andromeda/test-plan.md:349), with no retries or serialisation (.andromeda/test-plan.md:925, .andromeda/test-plan.md:919).
- **Security control negatives ↔ security-plan controls**: a Windows admission control without a negative gets one in `security_negatives_*.rs` (.andromeda/test-plan.md:606).
