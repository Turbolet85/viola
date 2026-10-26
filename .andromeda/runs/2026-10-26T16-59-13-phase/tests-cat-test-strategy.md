# tests · Test Strategy — category read

## Carried
- (.andromeda/test-plan-detail/test-strategy.md:10) The strict-modes, DACL, pinned-exe and plugin-rewrite cases this chunk adds must be inline unit cases with rstest `#[case]` tables over the refusal order and exit codes, running on all three OSes unless a case is a server-check captured-span case.
  > Inline `#[cfg(test)] mod tests` in all 6 crates plus the root bin. rstest 0.27.0 `#[case]` tables cover the refusal-order matrices, the exit-code table (0/1/2/10–14/20/21) and the per-`<cli-version>` fixture replay. Runs on all 3 OSes, except the two real-cargo-mutants harness tests (§10 Mutation gate) and the server check's captured-span cases, compiled out on macOS, where the server check reads no peer pid:
- (.andromeda/test-plan-detail/test-strategy.md:10) A frame verb's test module that gains a strict-modes case running on macOS must stop being gated whole (`last.rs`, `pause.rs`, `unlink.rs` today) — only its server-check case stays compiled out there.
  > a module is gated whole only when its every case is gated: `src/cmd/link.rs` gates its server-check case alone, and its reply-shape and line cases run on macOS); the decision table `server_verdict_rows` and the macOS `server_pid` case still run there.
- (.andromeda/test-plan-detail/test-strategy.md:11) Every new case must pass under nextest, the only supported runner.
  > nextest is the runner; a plain `cargo test` run (`cargo test --bin viola`) is not supported
- (.andromeda/test-plan-detail/test-strategy.md:14) The entry-point checks (CLI verbs, `hook`, `mcp`, `run`, `verify`) must be proven at integration level over a real tempfile home and the real binary, not a mocked filesystem.
  > Required. Every arch Standard Contract (Wrapper channel, On-disk, Hooks, CLI, MCP) has a crate-level `tests/` suite, using a real `interprocess` endpoint, a real tempfile home, and assert_cmd against `CARGO_BIN_EXE_viola`.
- (.andromeda/test-plan-detail/test-strategy.md:23) The strict-modes check this chunk adds to the hook client before its snapshot read (CARRY 3) must keep every timed hook row under its `max` gate.
  > hyperfine 1.20.0 gates the hook deadlines on `max` (the `--perf` arm, the five hyperfine rows — `pre-tool-use` included, on an unstamped perf session — and the per-OS `perf` CI job are built; §10).
- (.andromeda/test-plan-detail/test-strategy.md:26) A widened-mode home, a tampered pinned exe or a corrupt snapshot (CARRY 19 if ruled in) must be made by explicit `std` file operations, never a failpoint seam.
  > Required (chaos trigger present). Faults come from sysinfo 0.39.6 / `std` process and file operations (`File::set_len` truncation of a stopped wrapper's log, the torn append as landed in place of a kill mid-write; snapshot corruption, SIGSTOP heartbeat freeze, a no-EOF child), plus mockall 0.15.0 faults on the `viola-pty` seam.
- (.andromeda/test-plan-detail/test-strategy.md:29) The chunk carries no mutation gate of its own; mutation is judged at the epoch boundary, and Epoch 6 has none inside it.
  > No per-chunk, pre-push or CI mutation gate.
- (.andromeda/test-plan-detail/test-strategy.md:42) New strict-modes negatives (the 0770 `--home` case, the seeded-home question, single writers) must live in the named root suite prefixes and share `tests/support/` helpers (`home.rs` holds the home and the ConPTY seed), never a local copy.
  > **Sync (tokio-free) E2E suites:** root `tests/{cli,hook,tui,channel,chaos,contract,security_negatives}_<topic>.rs`, sharing helpers in `tests/support/{home.rs,outer_pty.rs,fake.rs,events.rs,cli.rs,ndjson.rs}`.
- (.andromeda/test-plan-detail/test-strategy.md:42) No new test file may define its own reader, wait, guard or result struct.
  > No root test file outside `tests/support/` defines its own copy of the reader, the wait, the guard or the result struct.
- (.andromeda/test-plan-detail/test-strategy.md:58) The strict-modes check lands before the server check that CARRY 18 leaves standing; the server-identity suite stays the off-macOS home of the impostor case and must keep passing.
  > root `security_negatives_server_identity.rs`, off macOS: a CLI frame verb against a server that is not the recorded wrapper;
- (.andromeda/test-plan-detail/test-strategy.md:62) The `--home` 0770 negative and the controls-not-disableable completeness case (CARRY 4) belong to the CLI-and-env vector, held at unit plus integration.
  > | CLI and env (V6) | unit + integration | §4 viola-core/root bin; §5 |
- (.andromeda/test-plan-detail/test-strategy.md:63) The chunk's core is the filesystem vector: its owner-only home, strict-modes and pinned-exe cases trace here, with E5 the replay leg if the founder rules CARRY 19 in.
  > | Filesystem (V7) | integration + chaos | §5 Module ↔ disk; §6 Scenario 1, E5 |
- (.andromeda/test-plan-detail/test-strategy.md:70) The Windows DACL proof of `check_instance` (CARRY 12) and the ConPTY companion owner/DACL check (CARRY 5) must be proven on the CI matrix's Windows runner.
  > | multi-os-compat | CI matrix | §9 Matrix builds |
- (.andromeda/test-plan-detail/test-strategy.md:73) A strict-modes refusal on every arm must carry no path, pid or endpoint, checked by the error-sanitization suite.
  > | error sanitization / secret logging | integration + E2E | §6 Error sanitization and secret scan; §3 logs |
- (.andromeda/test-plan.md:305) The new cases must be deterministic: no retries and no sleeps for synchronisation (the `pin_exe` race of CARRY 16 included).
  > no nextest retries (`retries = 0`) and no `std::thread::sleep` / `tokio::time::sleep` used for synchronisation
- (.andromeda/test-plan.md:308) Each test must let viola create its own home and stamps, which bears on the seeded-home question (CARRYs 6, 13) and the single-writer tests.
  > - Self-bootstrapping fixtures, with no developer-seeded data:
  >   - every test creates its own `--home` via tempfile 3.27.0
  >   - stamps come only from `viola verify` run against the fake agent
- (.andromeda/test-plan.md:320) New integration binaries must take a layer prefix that nextest filtersets select.
  > Rust integration binaries use a `<surface>_<topic>.rs` prefix so nextest filtersets can select layers: `cli_`, `hook_`, `tui_`, `channel_`, `chaos_`, `contract_` in root; `path_`, `mcp_`, `cross_` in `viola-e2e`.
- (.andromeda/test-plan.md:323) New test functions must follow the subject-condition-expected naming.
  > General pattern: snake_case `<subject>_<condition>_<expected>`, for example `send_with_esc_refuses_control_character` or `hook_oversize_stdin_exits_zero_silent`.
- (.andromeda/test-plan.md:868) The check-before-open order and the per-mode refusals belong in unit and integration cases, not E2E.
  > - NEVER assert at E2E a case that a lower layer can hold. Parser, refusal-order, threshold and clock cases belong in §4, contract shapes in §5, and §6 asserts only cross-process behaviour (§2 pyramid).
- (.andromeda/test-plan.md:873) The Windows DACL and seeded-home readings (CARRYs 12, 13) and the two fixed orders on Windows (CARRY 14) cannot be closed by a Linux dev-host run.
  > - NEVER treat a Linux-only local run as proof for Windows or macOS. OS-branch code is verified only on its own runner.

## Bindings
- **Unit gating on macOS ↔ security.md / the route entry "Unix endpoint and home hardening"**: the server check's macOS case stays owed there, while this chunk's strict-modes cases run on every OS (.andromeda/test-plan-detail/test-strategy.md:10).
- **Filesystem vector ↔ body §5 Module ↔ disk and §6 E5**: E5 steps 1–3 stay unbuilt until the readers move onto `read_snapshot_or_replay`, a CARRY owed to this entry and ruled by the founder at P4 (.andromeda/test-plan-detail/test-strategy.md:63; .andromeda/test-plan.md:590).
- **Self-bootstrapping ↔ body §7 Seed strategies**: the `seed_conpty` carve-out is the one seeded home a test makes before viola runs, and CARRYs 6 and 13 decide whether it must give way to viola creating the home first (.andromeda/test-plan.md:308; .andromeda/test-plan.md:631).
- **Perf ↔ body §10 Performance budgets**: the hook's added check must sit inside `CONNECT_DEADLINE` and `SPINE_DEADLINE` (.andromeda/test-plan-detail/test-strategy.md:23; .andromeda/test-plan.md:834).
- **Multi-os ↔ body §1 / §8 Windows DACL stubs**: a foreign-owner SID and a non-persistent-ACL volume stay synthetic SDDL stubs; every other Windows DACL case runs on `windows-2025` (.andromeda/test-plan-detail/test-strategy.md:70; .andromeda/test-plan.md:707).
- **Mutation ↔ body §10 Mutation gate**: the gate is judged only at the epoch boundary (.andromeda/test-plan-detail/test-strategy.md:29; .andromeda/test-plan.md:821).
