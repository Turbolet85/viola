# tests · Integration Test Strategy — category read

## Carried
- (.andromeda/test-plan-detail/integration-test-strategy.md:18) Any new or re-verified listener test (the first-pipe-instance property (b), the remote-client rejection (a)) must bind a real endpoint in a temp home and must create the squatter first through the interprocess server API, never `try_overwrite`.
  > Wrapper channel: real endpoint bound by `viola-channel` server in a temp home, with frames validated by jsonschema. The squatter listener is created first via the interprocess server API, never `try_overwrite`.
- (.andromeda/test-plan-detail/integration-test-strategy.md:18) The pipe DACL read-back the chunk re-verifies must compare Windows' canonical SDDL (protected, exactly two allow ACEs, user and SYSTEM), never the literal set; the DACL-effect test (c) must not weaken this oracle.
  > On Windows the pipe DACL is set from `D:P(A;;GA;;;<user-SID>)(A;;GA;;;SY)` and read back as Windows' own canonical SDDL, `D:P(A;;FA;;;<sid>)(A;;FA;;;SY)`: protected, exactly two allow ACEs, the user and SYSTEM. The user SID may render as an alias (`LA` for the built-in Administrator, as measured in CI run 36313377307), so the test compares the canonical form, never the literal it set.
- (.andromeda/test-plan-detail/integration-test-strategy.md:19) The IPC boundary's tools are pinned: a pipe-mode read-back (remote-client rejection) or DACL-effect probe uses these versions, windows-sys 0.61.2 for the Windows reads.
  > **Tools:** viola-channel client, interprocess 2.4.4, jsonschema 0.57.0, windows-sys 0.61.2
- (.andromeda/test-plan-detail/integration-test-strategy.md:32) Every integration test the chunk adds takes a fresh tempfile home of its own at a not-yet-existing path under `target/e2e-home`, so the CI scans cover it.
  > **Setup / teardown lifecycle:** there is a fresh tempfile home per test (the rstest `home` fixture). Like harness homes (§3 `boot` step 2), it is a not-yet-existing `home` path inside a `tempdir_in("<workspace root>/target/e2e-home")` parent, so viola creates it and obs-plan's CI gates, secret scan and upload cover integration runs too.
- (.andromeda/test-plan-detail/integration-test-strategy.md:32) A Windows x64 test that starts `viola run` (the squatted-name start refusal it re-verifies) takes the `seed_conpty` carve-out as it stands; the chunk adds no other pre-created home.
  > One carve-out (Windows x64, chunk 2026-09-29-sideloaded-conpty, the operator's ruling at that wrap): `Wrapper::boot` and every piped or outer-PTY `viola run` start first seed `<home>/bin/<key>/conpty/{OpenConsole.exe,conpty.dll}` through `tests/support/home.rs` `seed_conpty` (hard links, copy fallback, from one per-run copy under `target/conpty-seed/<key>/`, byte-identical to the embedded companions), so for them the test creates the home before viola runs.
- (.andromeda/test-plan-detail/integration-test-strategy.md:33) A wrapper or listener held by a test (the squatter, the first listener of a held-name pair) must be released by a `Drop` guard, with nothing shared between tests.
  > Wrappers started in a test are held in a `Drop` guard that calls `Child::kill()` then `wait()`. Nothing is shared between tests.
- (.andromeda/test-plan-detail/integration-test-strategy.md:42) The chunk's channel-level listener tests belong in the crate or root channel suites.
  > `send`/`wait`/`last`/`answer`/`pause`/`release`/`link`/`unlink`/`hook.dialog`/`hook.event` via `crates/viola-channel/tests/` and root `tests/channel_*.rs`
- (.andromeda/test-plan.md:46) Remote pipe clients and the DACL's effect on another principal are held untestable on hosted runners; work items (a) and (c) must be asserted at the check boundary or owned as a Windows-only live measurement, never claimed from a hosted run.
  > - **Entity:** `viola-channel`: JSON-RPC 2.0 over ndjson on `interprocess` local sockets — partially testable: same-user framing, dispatch, refusal order, server verification and the squatted-name refusal are testable; the cross-user parts (a peer of another euid, remote pipe clients, the Windows pipe DACL) cannot be driven on hosted runners and are asserted at the peer-credential check boundary.
- (.andromeda/test-plan.md:88) A second-user open or squat stays an untestable zone on hosted runners; the DACL-effect item (c) must respect it.
  > - Cross-OS-user attacks (a second local user squatting or connecting to the endpoint, reading the home). Hosted runners provide one account (security excerpt, Vectors 1 and 7).
- (.andromeda/test-plan.md:185) The IPC vector's squatted-endpoint negative must stand (re-verified, exit 1 unchanged).
  > - squatted endpoint → `run` exit 1
- (.andromeda/test-plan.md:187) The IPC vector's non-inheritance negative must stand for the pipe handles the chunk re-verifies.
  > - no inherited channel handles in the child
- (.andromeda/test-plan.md:250) The named-pipe and Windows DACL branch tests must run on the Windows runner as OS-branch tests.
  > - **Required test type:** full workspace and fake-agent E2E on all three runners. OS-branch tests cover named pipe vs Unix socket, ConPTY vs openpty, Windows DACL / strict-modes vs Unix modes, and macOS euid + dir-only server verification.
- (.andromeda/test-plan.md:287) The listener's admission behaviour is a Standard Contract under the integration layer: a crate-level `tests/` suite over a real `interprocess` endpoint.
  > - **Integration:** required — every arch Standard Contract has a crate-level `tests/` suite over a real `interprocess` endpoint, a real tempfile home and `assert_cmd`.
- (.andromeda/test-plan.md:305) No test the chunk adds or fixes (the held-name listener, the budget case_4) may sleep for synchronisation or rely on a retry.
  > - no nextest retries (`retries = 0`) and no `std::thread::sleep` / `tokio::time::sleep` used for synchronisation
- (.andromeda/test-plan.md:320) A new root integration binary must carry its layer prefix (`channel_` for the listener's admission cases).
  > - Rust integration binaries use a `<surface>_<topic>.rs` prefix so nextest filtersets can select layers: `cli_`, `hook_`, `tui_`, `channel_`, `chaos_`, `contract_` in root; `path_`, `mcp_`, `cross_` in `viola-e2e`.
- (.andromeda/test-plan.md:323) New test functions follow the naming pattern.
  > - General pattern: snake_case `<subject>_<condition>_<expected>`, for example `send_with_esc_refuses_control_character` or `hook_oversize_stdin_exits_zero_silent`.
- (.andromeda/test-plan.md:376) Windows handle non-inheritance stays a viola-channel unit case, re-verified, not moved up a layer.
  > - **viola-channel:** the ndjson frame codec at the `MAX_FRAME` boundary, the JSON-RPC error mapping, the FNV-1a endpoint golden vectors, the peer-credential decision with injected euids, and Windows handle non-inheritance.
- (.andromeda/test-plan.md:411) The integration driver for the Windows DACL read-back is windows-sys 0.61.2.
  > - **Windows DACL read-back:** windows-sys 0.61.2.
- (.andromeda/test-plan.md:421) The body's broad IPC boundary line the chunk's tests must satisfy.
  > - **Module ↔ IPC:** a real endpoint bound in a temp home with frames validated by jsonschema, the squatter created first through the server API (never `try_overwrite`), and the Unix socket modes and the Windows pipe DACL read back — the DACL compared in its canonical form, never the literal it set.
- (.andromeda/test-plan.md:476) Cross-user peers are covered by the peer-credential boundary unit test and the same-user SDDL read-back; a DACL-effect case (c) not measurable on a runner falls back to that coverage plus the owed live measurement.
  > - Cross-OS-user peers. Covered by the peer-credential boundary unit test (§4) and same-user SDDL read-back (§5).
- (.andromeda/test-plan.md:606) A negative for a security-plan control that had none (the remote-client rejection) belongs to the security control negatives suite; the CARRY's `tests/security_negatives_channel.rs` coordinate is re-verified against it.
  > - **Security control negatives** (`security_negatives_*.rs`): one negative test per security-plan control that had none — the Unix socket directory refusal, Windows client SQOS at the identification level, raw-channel `link` / `unlink` name validation, a Windows `--home` outside the profile given a protected DACL (its test home never under `%TEMP%`), umask-independent modes, and single writers of `stamps.json` and `snapshot.json`.
- (.andromeda/test-plan.md:724) The squatter is a fake server bound through the interprocess server API before `run`.
  > - **Fake servers:** the interprocess server API binds the squatter before `run`, and `viola-fake-agent` is the fake CLI; no HTTP fake servers.
- (.andromeda/test-plan.md:767) The chunk's acceptance reads on the `windows-2025` leg, which carries the named-pipe and DACL / SDDL read-back cases.
  > OS: `windows-2025` (ConPTY, named pipe, DACL / SDDL read-back, `.cmd` refusal)
- (.andromeda/test-plan.md:813) Windows-only listener code is covered only by the Windows job's per-OS coverage gate, which it must pass.
  > - Gates are enforced per OS job with `cargo llvm-cov nextest … --fail-under-lines 85 --fail-under-functions 95 --fail-under-regions 80`. Per-OS gating is deliberate: `#[cfg(windows)]` / `#[cfg(unix)]` code compiles only on its own runner.
- (.andromeda/test-plan.md:851) The CARRY's budget case_4 (red once under plain `cargo test`) is a flake under this budget: its root cause must be fixed within this chunk, never retried, ignored or parked.
  > **Zero-flakiness budget:** flaky tests are NOT tolerated. The nextest `retries = 0`. If a test flakes once, it gets quarantined immediately and fixed at the root cause, never with a retry-once budget. For viola, quarantine means the chunk stays red until the root cause is fixed within that chunk. `#[ignore]` is not allowed as a parking place.
- (.andromeda/test-plan.md:868) The first-pipe-instance property (b) and the pipe-mode read-back (a) belong at the listener's own layer, not at E2E.
  > - NEVER assert at E2E a case that a lower layer can hold. Parser, refusal-order, threshold and clock cases belong in §4, contract shapes in §5, and §6 asserts only cross-process behaviour (§2 pyramid).
- (.andromeda/test-plan.md:873) Windows endpoint claims are verified only on the Windows runner, never by the Linux dev host.
  > - NEVER treat a Linux-only local run as proof for Windows or macOS. OS-branch code is verified only on its own runner.
- (.andromeda/test-plan.md:889) The held-name and squatter tests must not use `try_overwrite` or share an endpoint name across tests.
  > - NEVER use interprocess `try_overwrite`, or reuse an endpoint name across tests. Take the endpoint only from `snapshot.json`.
- (.andromeda/test-plan.md:890) A fix of case_4's log capture under libtest threads must not reach for process-global env.
  > - NEVER use `std::env::set_var` in tests. It is `unsafe` in edition 2024 and races across threads; use `Command::env` per child.
- (.andromeda/test-plan.md:919) Case_4 may not be made green by serialising the suite.
  > - NEVER serialise tests (`max-threads = 1`): isolation comes from the per-test home.

## Bindings
- **Module ↔ IPC DACL read-back ↔ security-plan §Authentication & Authorization ("IPC access control (Windows)")**: the test's set literal and canonical oracle mirror the security-plan's SDDL `D:P(A;;GA;;;<user-SID>)(A;;GA;;;SY)`, `accept_remote(false)` and first-instance clause the chunk asserts (.andromeda/test-plan-detail/integration-test-strategy.md:18)
- **Module ↔ IPC ↔ the body's §1 testability limit and §6 untestable list**: remote pipe clients and the DACL's effect on another principal are untestable on hosted runners, so items (a) and (c) are asserted at the check boundary or owed to "Windows-only live measurements" (.andromeda/test-plan.md:46), (.andromeda/test-plan.md:476)
- **Squatter creation ↔ the body's §8 fake servers and §11 Integration ban**: the squatter is bound through the interprocess server API before `run`, never `try_overwrite`, no endpoint name reused (.andromeda/test-plan-detail/integration-test-strategy.md:18), (.andromeda/test-plan.md:724), (.andromeda/test-plan.md:889)
- **Setup lifecycle ↔ test-plan §7 Test Data & Fixtures (`seed_conpty`, `stamped_home`)**: a Windows x64 `viola run` start seeds the companions before viola runs; every other home is viola-created (.andromeda/test-plan-detail/integration-test-strategy.md:32)
- **Channel suite location ↔ the body's §2 naming and §6 security control negatives**: listener cases go in `crates/viola-channel/tests/` or root `tests/channel_*.rs`, a control's first negative in `security_negatives_*.rs` (.andromeda/test-plan-detail/integration-test-strategy.md:42), (.andromeda/test-plan.md:606)
- **Zero-flakiness ↔ the scope's CARRY (budget case_4)**: the case's red under plain `cargo test` is fixed at its root cause within this chunk, not retried, ignored or serialised (.andromeda/test-plan.md:851), (.andromeda/test-plan.md:919)
- **Windows leg ↔ test-plan §9 CI Integration and §10 Quality Gates**: the `windows-2025` job carries the DACL / SDDL read-back and gates Windows-only code on its own coverage (.andromeda/test-plan.md:767), (.andromeda/test-plan.md:813)
