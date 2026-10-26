# tests · Test Strategy — category read

## Carried
- (.andromeda/test-plan-detail/test-strategy.md:10) The verification's refusal cases (CLI exit 21, MCP `instance-unreachable`, hook exit 0) must sit in the unit-level rstest exit-code and refusal-order tables, on all three OSes.
  > rstest 0.27.0 `#[case]` tables cover the refusal-order matrices, the exit-code table (0/1/2/10–14/20/21) and the per-`<cli-version>` fixture replay. Runs on all 3 OSes, except the two real-cargo-mutants harness tests, compiled out on macOS (§10 Mutation gate).
- (.andromeda/test-plan-detail/test-strategy.md:11) The chunk's tests must pass under nextest, the one supported runner.
  > nextest is the runner; a plain `cargo test` run (`cargo test --bin viola`) is not supported
- (.andromeda/test-plan-detail/test-strategy.md:14) Server verification crosses the Wrapper channel, Hooks, CLI and MCP contracts, so each needs its integration case over a real endpoint, a real tempfile home and assert_cmd on the real binary.
  > Required. Every arch Standard Contract (Wrapper channel, On-disk, Hooks, CLI, MCP) has a crate-level `tests/` suite, using a real `interprocess` endpoint, a real tempfile home, and assert_cmd against `CARGO_BIN_EXE_viola`.
- (.andromeda/test-plan-detail/test-strategy.md:17) The critical paths the verified clients serve (send, wait/last, answer, wheel, MCP) must stay green E2E against harness-booted wrappers over the fake agent and the rmcp client.
  > Required for all 7 critical paths plus 4 expansion scenarios (Section 6). They run against harness-booted `viola run … -- viola-fake-agent`, using the outer PTY (portable-pty) and the rmcp client.
- (.andromeda/test-plan-detail/test-strategy.md:23) The check added to `viola hook` before its first frame runs inside the timed hook rows, so those rows' `max` gate must still pass.
  > hyperfine 1.20.0 gates the hook deadlines on `max` (the `--perf` arm, the five hyperfine rows — `pre-tool-use` included, on an unstamped perf session — and the per-OS `perf` CI job are built; §10).
- (.andromeda/test-plan-detail/test-strategy.md:26) Any fault case the chunk adds (a mismatched or exited server, a corrupt snapshot) must be injected only through process and file operations or existing mockall seams.
  > Faults come from sysinfo 0.39.6 / `std` process and file operations (`File::set_len` truncation of a stopped wrapper's log, the torn append as landed in place of a kill mid-write; snapshot corruption, SIGSTOP heartbeat freeze, a no-EOF child), plus mockall 0.15.0 faults on the `viola-pty` seam.
- (.andromeda/test-plan-detail/test-strategy.md:29) The chunk must add no per-chunk mutation gate; mutation stays at the epoch boundary.
  > No per-chunk, pre-push or CI mutation gate.
- (.andromeda/test-plan-detail/test-strategy.md:41) New crate-level integration tests (for `viola-channel`'s server-identity read, `viola-state`'s checks, `viola-mcp`'s handlers) must follow the crate `tests/` path pattern.
  > **Crate-level integration tests:** `crates/viola-<area>/tests/<topic>.rs`.
- (.andromeda/test-plan-detail/test-strategy.md:42) Root tests of the verified CLI and hook clients must use the shared `tests/support/` reader, wait and guard, never a copy, and read live-appended files only through the complete-lines reader.
  > No root test file outside `tests/support/` defines its own copy of the reader, the wait, the guard or the result struct. `ndjson.rs` is the one complete-lines reader: every reader of a live-appended file parses only newline-terminated lines, because a Linux reader can observe an append part-way.
- (.andromeda/test-plan-detail/test-strategy.md:43) A new E2E binary for the verification (an MCP or cross-surface case) must carry a prefix the E2E layer regex selects.
  > both homes report in the E2E layer, whose regex is `^(path[0-9]*|tui|mcp|cross|chaos|contract)_` (§3 5-command `run` step 2).
- (.andromeda/test-plan-detail/test-strategy.md:44) Tests of the MCP Tokio client's verification belong where tokio is allowed; the root package and the sync crates must gain no tokio dev-dependency.
  > `crates/viola-e2e` hosts `viola-harness` and the tokio-based test clients (rmcp client), so the root package and the sync crates carry no tokio dev-dependency.
- (.andromeda/test-plan-detail/test-strategy.md:58) The IPC vector the chunk hardens is traced to integration + E2E; the server-verification negatives must land at those layers, beside the Security control negatives.
  > | IPC vector (V1) | integration + E2E | §5 Module ↔ IPC; §6 Scenario 1 + Security control negatives (root `security_negatives_channel.rs`: the Windows listener's remote-client rejection and DACL denial) |
- (.andromeda/test-plan-detail/test-strategy.md:60) The hook client's fail-open-on-mismatch case is a Hook stdin vector case: an assert_cmd hook integration test plus the Security sweep.
  > | Hook stdin (V4) | integration (assert_cmd hook) | §5 hook boundary; §6 Security sweep |
- (.andromeda/test-plan-detail/test-strategy.md:61) The MCP frame tools' `instance-unreachable` on a mismatch is an MCP vector case, driven E2E by the rmcp client.
  > | MCP (V5) | E2E (rmcp client) | §6 Scenarios 2, 5, 7 + Security sweep |
- (.andromeda/test-plan-detail/test-strategy.md:63) The snapshot the verification reads (and any move of the five readers to the classified read) is the Filesystem vector, traced to integration + chaos and E5.
  > | Filesystem (V7) | integration + chaos | §5 Module ↔ disk; §6 Scenario 1, E5 |
- (.andromeda/test-plan-detail/test-strategy.md:67) A chaos case the chunk adds goes in the Chaos suite.
  > | chaos-test | chaos | §6 Chaos suite |
- (.andromeda/test-plan-detail/test-strategy.md:69) The CLI and MCP must give identical unreachable payloads for the same verification failure, held E2E.
  > | cross-surface-coordination | E2E | §6 Scenarios 2, E3 + Cross-surface parity |
- (.andromeda/test-plan-detail/test-strategy.md:70) The OS branches of the server read (Unix peer creds, Windows pipe server pid, macOS euid + dir) must each be tested on its CI matrix runner.
  > | multi-os-compat | CI matrix | §9 Matrix builds |
- (.andromeda/test-plan-detail/test-strategy.md:73) The new unreachable refusals must carry no path, pid or upstream text, held at integration + E2E.
  > | error sanitization / secret logging | integration + E2E | §6 Error sanitization and secret scan; §3 logs |
- (.andromeda/test-plan.md:287) The body's broad line for the integration layer the chunk's IPC cases use.
  > **Integration:** required — every arch Standard Contract has a crate-level `tests/` suite over a real `interprocess` endpoint, a real tempfile home and `assert_cmd`.
- (.andromeda/test-plan.md:305) Verification tests (a mismatched server, a wrapper exiting) must synchronise without retries or sleeps, on logged events and byte offsets, and never touch the real `claude`.
  > no nextest retries (`retries = 0`) and no `std::thread::sleep` / `tokio::time::sleep` used for synchronisation
  >   - waits key on `events.ndjson` byte offsets or `viola wait --after <cursor>`
  >   - CI never touches the real `claude`
- (.andromeda/test-plan.md:309) Every verification test must make its own home.
  > every test creates its own `--home` via tempfile 3.27.0
- (.andromeda/test-plan.md:320) New integration binaries must take the surface prefix that selects their layer.
  > Rust integration binaries use a `<surface>_<topic>.rs` prefix so nextest filtersets can select layers: `cli_`, `hook_`, `tui_`, `channel_`, `chaos_`, `contract_` in root; `path_`, `mcp_`, `cross_` in `viola-e2e`.
- (.andromeda/test-plan.md:323) New tests must follow the `<subject>_<condition>_<expected>` naming.
  > General pattern: snake_case `<subject>_<condition>_<expected>`, for example `send_with_esc_refuses_control_character` or `hook_oversize_stdin_exits_zero_silent`.
- (.andromeda/test-plan.md:868) The pure server-identity decision belongs in §4 unit cases; E2E asserts only the cross-process behaviour.
  > NEVER assert at E2E a case that a lower layer can hold. Parser, refusal-order, threshold and clock cases belong in §4, contract shapes in §5, and §6 asserts only cross-process behaviour (§2 pyramid).
- (.andromeda/test-plan.md:869) The mismatch outcome spans CLI, hook and MCP surfaces, so each needs its own driver step.
  > NEVER assert a multi-surface critical path on one surface only. Every surface a §6 scenario lists needs its own driver step and transition signal.
- (.andromeda/test-plan.md:870) The verification's integration cases must use a real endpoint, not a mocked server identity.
  > NEVER over-mock at integration level (you stop testing the integration)
- (.andromeda/test-plan.md:873) A Linux dev-host reading of the verification proves nothing for the Windows pipe or macOS euid + dir branches; each is verified only on its own runner.
  > NEVER treat a Linux-only local run as proof for Windows or macOS. OS-branch code is verified only on its own runner.

## Bindings
- **IPC vector (V1) ↔ body §1 coverage trigger and §5 Wrapper channel**: the layer row (.andromeda/test-plan-detail/test-strategy.md:58) is served by the required negative "server pid / start-time mismatch vs `snapshot.json` → CLI exit 21 / MCP `instance-unreachable` / hook exit 0 with no body" (.andromeda/test-plan.md:184) and the channel suites' "server verification" (.andromeda/test-plan.md:435); security.md's "Before the FIRST frame" clause is the control both sides test.
- **Hook stdin (V4) ↔ body §6 Security sweep**: the fail-open-on-mismatch case (.andromeda/test-plan-detail/test-strategy.md:60) joins the hook fail-open matrix (.andromeda/test-plan.md:598), whose "channel failure" case (.andromeda/test-plan.md:193) it extends.
- **MCP (V5) / cross-surface ↔ body §5 MCP handlers and §6 Cross-surface parity**: (.andromeda/test-plan-detail/test-strategy.md:61) and (.andromeda/test-plan-detail/test-strategy.md:69) bind to the `TestWrapper` handler tests over the Tokio client (.andromeda/test-plan.md:449) and to MCP/CLI payload equality (.andromeda/test-plan.md:611).
- **Filesystem (V7) ↔ body §6 E5 and §11 Test Data**: (.andromeda/test-plan-detail/test-strategy.md:63) meets E5's unbuilt steps "owed to the route entry \"Server verification before any frame\"" (.andromeda/test-plan.md:590), held by the CARRY 7 founder hold; a mismatched-snapshot case must not hand-write `snapshot.json` outside a reader's own unit cases altering the writer's bytes (.andromeda/test-plan.md:905).
- **Chaos ↔ body §1 chaos trigger and §10 liveness row**: (.andromeda/test-plan-detail/test-strategy.md:26) binds to "an endpoint vanishing during `wait` read `instance-unreachable`" (.andromeda/test-plan.md:237) and the classifier's "pid or start-time mismatch → gone at any age" (.andromeda/test-plan.md:842).
- **multi-os-compat ↔ body §1 trigger and §9 matrix**: (.andromeda/test-plan-detail/test-strategy.md:70) binds to "macOS euid + dir-only server verification" as an OS-branch test (.andromeda/test-plan.md:250) on `macos-latest` (.andromeda/test-plan.md:767), with (.andromeda/test-plan.md:873).
- **Perf ↔ body §10 Spine deadline**: (.andromeda/test-plan-detail/test-strategy.md:23) binds to the hook's `CONNECT_DEADLINE` asserted below `SPINE_DEADLINE` (.andromeda/test-plan.md:834); the added check must fit inside them.
- **Naming / support helpers ↔ body §6 Exit-cause matrix**: a new exit-21 cause written with the shared helpers (.andromeda/test-plan-detail/test-strategy.md:42) needs its own `cross_exit_causes.rs` case with a cause-naming hint (.andromeda/test-plan.md:604).
