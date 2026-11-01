# Task — tests · 2026-11-01-unix-endpoint-and-home-hardening

## The chunk
Builds the verification of the per-user Unix socket directory (owner, mode, not a symlink) before a wrapper binds and before a client connects; a directory that fails is refused, never used and never narrowed in place, and the socket stays 0600 by chmod after the bind, a secondary control only.
Builds the peer euid check on both sides of the local socket: the wrapper against a connecting client, and every Unix client against the server it reached, before its first frame.
Applies Unix strict modes on the paths this entry reaches and makes the board's home check one rule, so `viola list` no longer passes a 0750 home while it holds no instance and refuses it naming the instance once it holds one.
Records the macOS pid gap as a named carve-out (macOS reports no peer pid, so the server check there is a match with no comparison), adds the macOS exit-cause case (the per-user socket directory loosened to 0755, exit 21), and gives each new refusal a closed code, a hint and a row of the exit-cause table, with no new exit code.
Re-derives at HEAD the root kill path's wait on a connect probe and whether the takeover of a killed wrapper's leftover socket file holds beside the new checks, and measures the hypothesis that harness `cleanup` reads `endpoint_gone:false` and exits 1 after force-killing a wrapper on Unix.

## Reached files
- tests/cross_exit_causes.rs · edits the exit-cause table, its rows for the socket-directory refusal and the peer euid mismatch
- board.rs · edits `board::scan`'s home check (`check_path` then `check_instance`); the scope names this file by its bare name, its directory unstated
- tests/support/home.rs · re-reads `holder_gone`, the root kill path's connect probe
- crates/viola-e2e/src/harness/cleanup.rs · re-reads its Unix rule, the socket path gone, behind `endpoint_gone:false` after a force-kill

## Needs
- The per-user socket directory in each of its three places (`$XDG_RUNTIME_DIR/viola/`, `$TMPDIR/viola/`, `/tmp/viola-<uid>/`): the check of owner, 0700 and not-a-symlink at wrapper bind and at client connect, with cases for a loosened, a symlinked and a foreign-owned directory, each left as found.
- `bind_endpoint`'s `create_private_dir` failure, an anyhow error at the catch site today: the verified directory's refusal replaces it with a closed code and a hint, and the tests read that code, not an internal failure.
- The security-vector IPC trigger's assertion of a Unix socket directory at 0700 and a file at 0600, the Module-to-IPC read-back of the Unix socket modes, and the security-control negatives' Unix socket directory refusal and umask-independent modes cases.
- The socket's 0600 by chmod after the bind, never through `ListenerOptionsExt::mode`, which fails the bind on macOS; tests hold the file's mode as a secondary control beside the directory check.
- The peer euid decision on both sides: the channel's peer-credential check with injected euids at the unit layer, because a peer of another euid and a foreign-owned directory cannot be driven on hosted runners (the one-account untestable zone).
- The client-side euid check across every Unix client: the CLI frame verbs, `viola list` and other connecting verbs, the hook's `connect`, the MCP frame tools and `viola_channel::tokio_client::TokioClient`, with a case that a mismatch writes no frame.
- The per-user socket directory stands outside a test's `--home`: a case that loosens or replaces it must re-point it for its own child through those variables, so a parallel test's wrapper never meets the widened directory.
- Test-side servers that a verifying client now reaches: the squatter bound first through the server API, the server-identity impostors and `TestWrapper` in `crates/viola-mcp/tests/handlers.rs`, each still reaching its verdict past the new directory and euid checks.
- The surface readings of each new refusal: CLI `--json` document and last `hint:` line, MCP `instance-unreachable`, hook exit 0 with no body, `run` exit 1 before its log; the scope names both exit 21 and a `run` refusal and leaves which is which to P3.
- The exit-cause matrix suite's table rules over the new rows: one negative test per cause asserting the exit code, the `--json` document and a hint naming its cause, `no_two_codes_share_a_hint`, `no_hint_names_release`, and the top comment listing causes held outside the table.
- The macOS exit-cause case, the socket directory loosened to 0755 giving exit 21, and the server-verification macOS case the exit-cause matrix suite says is owed to this route entry (beside `tests/security_negatives_server_identity.rs`, whose negatives compile out on macOS).
- The macOS carve-out over tests: no test reads the match with no comparison as a comparison and no weaker comparison stands in for the peer pid; the multi-os-compat trigger's euid and dir-only server verification and the `macos-latest` matrix row carry it.
- The board's one home rule: `viola list`'s whole-board refusal (`strict_modes_on_the_board_is_exit_21_and_opens_no_log`), `ScanRefused::Instance`, a 0750 home with no instance and with one, and the board cases of `tests/cli_list.rs`.
- Unix strict modes at the `viola-state` unit layer and the filesystem security-vector negatives: the write rule and the read rule, the instance check, the home-tree check, and a home another user may read being refused.
- The strict-modes refusal table over surfaces and its no-line rule (`run`, `viola revive`, `viola verify`, `mcp` exit 1; hook 0 with no output; MCP frame tools `instance-unreachable`; other verbs CLI 21), which a new reading of a refused home or directory must agree with.
- `hook`'s contract: a refusal of this chunk reaches it only as a fail-open, exit 0, no body, no stderr, added to the security sweep's hook fail-open matrix for a refused directory and an euid mismatch.
- The hook's time bound over the added checks: the checks run inside `CONNECT_DEADLINE` under `viola_core::SPINE_DEADLINE`, the `hook_fails_open_silently_within_the_spine_bound` case and the hyperfine `max` rows.
- A killed wrapper's leftover socket file and the next bind's takeover beside the directory and euid checks, with the `.lock` start arbiter, Path 1's squatted-endpoint exit 1 and the exit 21 readings of a killed wrapper in Path 3 and the chaos endpoint-vanishing case left true.
- The `holder_gone` measurement: the root kill path waits on a connect probe because a wait on the file being absent never ended; the probe is re-derived at HEAD with the client-side checks standing.
- Harness `cleanup` on Unix: exit 1 on failure and its idempotence, measured against `endpoint_gone` after a force-kill, the hypothesis no run has measured.
- Cross-surface parity for each new cause of exit 21: the MCP `isError` body `{error, detail}` equals the CLI `--json` document less `v`, its text `refused: instance-unreachable`.
- Error sanitization over the new refusals: a code and hint carrying no absolute path of the refused directory, no anyhow chain, read by `tests/contract_error_bodies.rs`'s witnesses and its census of writers.
- The typed exit set held at 0, 1, 2, 10–14, 20, 21 by the root bin's exit-code mapping, and the CLI-and-env vector's rule that no env, flag or config disables one of the new controls.
- Capability coverage: `v1-44` (only the owning user connects, every client verifies, the macOS gap recorded), `v1-53` (the same end-to-end suite on Linux and macOS, one live Linux run) and the socket-directory leg of `v1-18`.

## Standing conventions
- S1 · Integration binaries named by the `<surface>_<topic>.rs` prefix and test functions in `<subject>_<condition>_<expected>` snake_case · §2 Test directory + naming conventions
- S2 · Each test seeds its own fresh tempfile home under `target/e2e-home`, made through `viola_state::fs::create_private_dir` where the test makes it, with no state shared between tests · §5 Setup / teardown lifecycle and §7 Seed strategies
- S3 · Env set per child with `Command::env`, never `std::env::set_var`, and `.env_clear()` re-adding `LLVM_PROFILE_FILE` · §11 Integration
- S4 · No sleep for synchronisation, waits keyed on an explicit signal, nextest `retries = 0` and no `#[ignore]` parking · §11 E2E and §10 Zero-flakiness budget
- S5 · The filesystem is real and never mocked, and mockall doubles exist only for traits viola already defines at its seams · §8 Mocking & Stubbing Discipline
- S6 · Table cases are named rstest `#[case]` rows, or labelled case tables in `viola-e2e` and literal tables in `viola-core` · §4 Conventions
- S7 · Expected exit codes and refusal words are test literals, asserted as the integer and the serde kebab-case value, never read from the product or by `Debug` string · §11 Unit
- S8 · Each OS job holds lines 85, functions 95 and regions 80, and the coverage ignore regex is never widened to pass · §10 Coverage thresholds and §11 CI
- S9 · OS-branch code is proven only on its own CI runner, a Linux-only local run proving nothing for macOS, which is CI-only · §11 Test Strategy and §9 Matrix builds
- S10 · A case sits at the lowest layer that can hold it, §6 asserting only cross-process behaviour · §11 Test Strategy
- S11 · A new diagnostics line validates against obs-plan's `diag-line` or `diag-detail` schema · §6 Schema conformance
- S12 · A started wrapper sits in a `Drop` guard and is never run through `assert_cmd` `.assert()`, a test-started `viola` drops `VIOLA_NAME`, `VIOLA_DIR` and `VIOLA_BIN`, and root suites share `tests/support/` helpers without copying them · §11 E2E and §5 Cross-module patterns

## Not needed
- Windows pipe DACL and SDDL descriptor stubs, client SQOS, the Windows strict-modes reading and the ConPTY sideload and its companions: no Windows behaviour moves in this chunk.
- Confirmed `send` with CL-1 events and paste validation (Path 2): the chunk changes no typing, confirmation or `validate_paste_text` behaviour.
- Dialog decision bodies, the insta snapshots and the unverified-CLI version gate (Paths 4 and 7): the chunk changes no dialog or stamp rule.
- The wheel and `release` (Path 5) and the budget governor's thresholds, gate events and override (Path 6): the chunk moves none of them.
- The R8 `CLAUDE*` strip and identity floor (E2), `link` / `unlink` (E3) and snapshot replay (E5): none of them is in this chunk's scope.
- Recorded hook-payload fixtures, the fake agent's byte comparison against them and the fixture refresh: the chunk records no new CLI behaviour.
- `list` row integrity and the board's caption layout: the board's change is its home check, not its rows.
- Parser properties and cargo-fuzz targets: the chunk adds no parser surface.
- Torn lines, a no-EOF child, the forced vt100 panic and heartbeat staleness in the chaos suite: the chunk changes none of these faults.
- `run --mutants`, its gate and `windows-mutants.yml`: mutation runs at the epoch-boundary audit, with no chunk gate.
- The frontend and scenario E4: deferred to 0.2.0.
