# tests · Test Strategy — category read

## Carried
- (.andromeda/test-plan-detail/test-strategy.md:10) Each new refusal's unit cases must sit in rstest tables over the typed exit set, which stays closed with no new exit code.
  > rstest 0.27.0 `#[case]` tables cover the refusal-order matrices, the exit-code table (0/1/2/10–14/20/21) and the per-`<cli-version>` fixture replay.
- (.andromeda/test-plan-detail/test-strategy.md:10) The macOS pid gap must be honoured in the suite: the server check's captured-span negatives stay compiled out on macOS, gated by the named rule, the decision table and the `server_pid` case keep running there, and no test reads the gap as a comparison.
  > the server check's captured-span cases, compiled out on macOS, where the server check reads no peer pid: each frame verb's `request_refuses_an_unverified_server_inside_its_dispatch_span`, `hook_refuses_an_unverified_server_inside_hook_handle`, the test-only `cmd::client::refused_inside` helper they share (under `#[cfg(test)]` + `#[cfg(not(target_os = "macos"))]`) and the `last.rs`, `pause.rs` and `unlink.rs` test modules, gated whole (a module is gated whole only when its every case is gated: `src/cmd/link.rs` gates its server-check case alone, and its reply-shape and line cases run on macOS); the decision table `server_verdict_rows` and the macOS `server_pid` case still run there.
- (.andromeda/test-plan-detail/test-strategy.md:11) The chunk's tests must run under nextest; a plain `cargo test` reading is no evidence.
  > nextest is the runner; a plain `cargo test` run (`cargo test --bin viola`) is not supported
- (.andromeda/test-plan-detail/test-strategy.md:29) Mutation is no gate of this chunk, so its guards' strength must be shown by the remove-the-guard rule below, not by a chunk mutation run.
  > No per-chunk, pre-push or CI mutation gate.
- (.andromeda/test-plan-detail/test-strategy.md:29) Each new guard (directory verification, peer euid on both sides, the board's home rule) must pass remove-the-guard, and a neutralised guard that lets a wrapper start needs a witness bounded through `Running` and `WITHIN`.
  > Beside it stands the per-chunk remove-the-guard rule (`.claude/rules/testing.md`): a control whose neutralised guard lets a product process start needs a bounded witness; the G4 / G10 witnesses spawn through `tests/support/cli.rs` `Running`, bounded by `WITHIN`, since nextest's default profile has no kill line (an unbounded witness hung 27 minutes, as measured at `viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md`).
- (.andromeda/test-plan-detail/test-strategy.md:42) A test that loosens the socket directory or a home path must use the shared helpers and hold no copy of its own outside `tests/support/`.
  > `home.rs` holds the home-widening helpers `widen` · `grant_read` · `unwiden` · `still_widened` (Unix modes; on Windows `icacls`, its exit asserted), through which a root test widens or read-grants a home path, restores it and reads whether it is still widened. No root test file outside `tests/support/` defines its own copy of the reader, the wait, the guard or the result struct.
- (.andromeda/test-plan-detail/test-strategy.md:43) The killed-wrapper and leftover-socket takeover cases must start from the shared start helpers, `kill_wrapper` among them.
  > `tests/support/start.rs` shares the start setups across root test files: `first_life`, `wait_session_start_hooks`, `made_dir`, `stopped_life`, `remove_dir_once_free`, `plant_agents`, `listed`, `revive` and `kill_wrapper`.
- (.andromeda/test-plan-detail/test-strategy.md:44) The new refusal rows land in this `cross_` binary, which must stay fake-agent gated and selected by the E2E layer's regex.
  > a `cross_` binary (`tests/cross_exit_causes.rs`, sync, `[[test]]` `required-features = ["fake-agent"]`); both homes report in the E2E layer, whose regex is `^(path[0-9]*|tui|mcp|cross|chaos|contract)_`
- (.andromeda/test-plan-detail/test-strategy.md:59) The IPC trigger row must be traced to the chunk's new socket-directory and euid negatives, and its off-macOS legs must stay off macOS.
  > §6 Scenario 1 + Security control negatives (root `security_negatives_channel.rs`: the Windows listener's remote-client rejection and DACL denial; root `security_negatives_server_identity.rs`, off macOS: a CLI frame verb against a server that is not the recorded wrapper; root `channel_links.rs`: raw `link` / `unlink` frames with a bad name or pair, each `-32602` with no line written) + Security sweep (the hook fail-open impostor rows, off macOS)
- (.andromeda/test-plan-detail/test-strategy.md:64) The filesystem trigger's strict-modes negatives must keep holding on the paths this chunk changes: refused before any log, exit 21 with its detail, no frame and no line, and a start under umask 000 owner-only.
  > Security control negatives (root `security_negatives_strict_modes.rs`: each of the six frame verbs `send` · `wait` · `last` · `answer` · `pause` · `release` refusing a widened instance before any log, exit 21 with the instance pair or the `instance-unreachable` document, its `detail` `strict-modes-failed`, no frame and no line; a home another user may read refusing `run`; a start under umask 000 making every home path owner-only (Unix);
- (.andromeda/test-plan.md:46) A peer of another euid must be asserted at the peer-credential check boundary, since hosted runners cannot drive it.
  > only the cross-user part (a peer of another euid) cannot be driven on hosted runners and is asserted at the peer-credential check boundary.
- (.andromeda/test-plan.md:118) P3's measurement of `cleanup` after a force-killed Unix wrapper must be read against this requirement.
  > `cleanup` stops every wrapper and deletes the home, idempotently
- (.andromeda/test-plan.md:135) The takeover of a leftover socket file must not weaken this: a squatted endpoint or a refused home tree still exits `run` 1.
  > a second `run` of the same name, a squatted endpoint, a tampered pinned exe or a refused home tree exits 1.
- (.andromeda/test-plan.md:186) The socket directory's and the socket file's modes must be asserted as this trigger requires.
  > Unix socket dir 0700 and file 0600 asserted
- (.andromeda/test-plan.md:201) Every refusal this chunk adds to a home path must keep this exit mapping per surface.
  > `--home` / `VIOLA_DIR` failing strict-modes → exit 1 / 21 / hook 0
- (.andromeda/test-plan.md:210) The board's one home rule must satisfy this trigger: a home another user may read is refused whether or not it holds an instance.
  > a home, instance directory or `diagnostics/` another user may read → refused (Unix a group or other bit; Windows a read grant to another SID)
- (.andromeda/test-plan.md:254) The new OS-branch legs must be covered on their runners, the macOS server verification being euid and directory only.
  > OS-branch tests cover named pipe vs Unix socket, ConPTY vs openpty, Windows DACL / strict-modes vs Unix modes, and macOS euid + dir-only server verification.
- (.andromeda/test-plan.md:380) The euid check's decision must have unit cases with injected euids on both sides, and the server-pid reading stays per OS.
  > the peer-credential decision with injected euids, the connected stream's server pid on each OS
- (.andromeda/test-plan.md:382) The directory verification and the board's home rule must be unit-covered among the `viola-state` checks, write rule and read rule alike.
  > Unix strict-modes and Windows strict-modes (the write and the read rule) over synthetic SDDL descriptors and, on `windows-2025`, real grants, the instance check, the home-tree and diagnostics checks
- (.andromeda/test-plan.md:393) A client-side euid refusal must be held as this server check is: refused inside the span, its closed cause logged, on each frame verb and the hook.
  > the server check refused inside each frame verb's and the hook's span with each closed unreachable cause logged
- (.andromeda/test-plan.md:428) The socket's modes must be read back from a real endpoint, and a squatter must be created through the server API, never by overwrite.
  > a real endpoint bound in a temp home with frames validated by jsonschema, the squatter created first through the server API (never `try_overwrite`), and the Unix socket modes and the Windows pipe DACL read back
- (.andromeda/test-plan.md:444) The chunk's strict-modes changes must keep this per-surface contract, including that a refused check writes no line; P3 reads where the directory and euid refusals sit against it.
  > strict-modes refusals give exit 1 for `run` / `mcp` / `viola revive` / `viola verify`, hook 0 with no output, `instance-unreachable` from the MCP frame tools, and CLI 21 for every other verb (the six frame verbs printing the instance pair); a refused check at `run`, `viola revive`, `viola verify`, the frame verbs, the hook and the MCP frame tools writes no line.
- (.andromeda/test-plan.md:484) The cross-user euid leg must be held at the unit boundary, with no E2E pretending to a second account.
  > Cross-OS-user peers. Covered by the peer-credential boundary unit test (§4) and same-user SDDL read-back (§5).
- (.andromeda/test-plan.md:486) Every macOS leg of this chunk must be proved on its CI runner, never locally.
  > macOS local runs, which are CI-only.
- (.andromeda/test-plan.md:604) A directory or euid refusal reaching `hook` must take a row of this fail-open matrix, exit 0 with no body.
  > the hook fail-open matrix — oversize stdin, malformed JSON, clap error, channel failure, a forced panic, a missing `VIOLA_NAME`, an instance the instance check refuses or a `VIOLA_DIR` naming no instance directory and, on Linux and Windows, a server that is not the recorded wrapper, each exit 0 with no body
- (.andromeda/test-plan.md:606) The new refusals' documents and hints must name no socket path or upstream text, under this rule.
  > no absolute path, serde path or `Caused by` in any error payload
- (.andromeda/test-plan.md:610) This matrix is the chunk's owed item: each new cause takes a row asserting exit code, `--json` document and a hint of its own, none naming `release`, and the macOS case owed to this route entry must be discharged, without reading as a comparison.
  > **Exit-cause matrix** (`tests/cross_exit_causes.rs`, a root `cross_` binary): one negative test per cause of exit 21 and of exit 1 it holds, each asserting the exit code, the `--json` document and a last `hint:` line that names its cause — a hint shared across causes fails the table (`no_two_codes_share_a_hint`, its control `shared_hints_reads_a_planted_duplicate`); the causes outside it, named in its top comment, are server verification, stale heartbeat, the `.cmd` human pair, `verify`'s version, recording, pinned-copy, interrupted and stamps refusals, and `viola mcp`'s two silent exits; beside the table, the server-verification cause is a case of `tests/security_negatives_server_identity.rs` on Linux and Windows (its macOS case owed to the route entry "Unix endpoint and home hardening"), the strict-modes cause of exit 21 for the six frame verbs is a case of `tests/security_negatives_strict_modes.rs` on three OSes, the exit-1 home-tree refusals of `run` and `verify` are cases of `tests/run_cli.rs` and `tests/cli_verify.rs`, and `revive`'s refusals are cases of `tests/cli_revive.rs`; driver-facing hints never contain `release` (`no_hint_names_release`).
- (.andromeda/test-plan.md:612) The verified directory's refusal must have its negative test among the security control negatives.
  > one negative test per security-plan control that had none — the Unix socket directory refusal,
- (.andromeda/test-plan.md:612) The board's one home rule must keep these witnesses green: exit 21 opening no log, and the pre-existing readable home's refusal of `run`.
  > `viola list`'s whole-board refusal exit 21 opening no log and `viola plugin install`'s exit 21 creating nothing (`tests/cross_exit_causes.rs` `strict_modes_on_the_board_is_exit_21_and_opens_no_log`, `strict_modes_on_plugin_install_is_exit_21_and_creates_nothing`)
- (.andromeda/test-plan.md:618) Any new cause of exit 21 must hold this parity across CLI and MCP.
  > for each cause of exit 21, the MCP `isError` body `{error, detail}` equals the CLI `--json` document less `v`, its text `refused: instance-unreachable`
- (.andromeda/test-plan.md:639) Test homes must be made as the product makes them, the board's rule cases included, and only the server-identity impostors may build a `snapshot.json`.
  > except the chaos tests' corruption of files the product already wrote, the Windows x64 `seed_conpty` companions and the server-identity impostors' test-built `snapshot.json`; a case that needs a home viola created starts from `stamped_home`, and a test that makes a home itself makes it through `viola_state::fs::create_private_dir`, never `create_dir_all`: a plain home is one the product refuses (on Unix by the read rule;
- (.andromeda/test-plan.md:775) The macOS and Ubuntu runners are where this chunk's Unix legs must be proved; the "euid + dir-only" wording names macOS's verification.
  > `macos-latest` (macOS 26 arm64: openpty, Unix socket, euid + dir-only server verification; pin `macos-26` if the label moves), `ubuntu-latest` (openpty, Unix socket, modes;
- (.andromeda/test-plan.md:821) New `#[cfg(unix)]` and macOS branches must meet the coverage gates on their own OS job.
  > Gates are enforced per OS job with `cargo llvm-cov nextest … --fail-under-lines 85 --fail-under-functions 95 --fail-under-regions 80`. Per-OS gating is deliberate: `#[cfg(windows)]` / `#[cfg(unix)]` code compiles only on its own runner.
- (.andromeda/test-plan.md:859) A race around a leftover socket or a takeover must be fixed at its root inside this chunk, with no retry and no `#[ignore]`.
  > flaky tests are NOT tolerated. The nextest `retries = 0`. If a test flakes once, it gets quarantined immediately and fixed at the root cause, never with a retry-once budget. For viola, quarantine means the chunk stays red until the root cause is fixed within that chunk. `#[ignore]` is not allowed as a parking place.
- (.andromeda/test-plan.md:876) The refusal order and euid decision belong to the lower layers; E2E asserts only cross-process behaviour.
  > NEVER assert at E2E a case that a lower layer can hold. Parser, refusal-order, threshold and clock cases belong in §4, contract shapes in §5, and §6 asserts only cross-process behaviour (§2 pyramid).
- (.andromeda/test-plan.md:881) A leg of the Linux and macOS parity must not be claimed from the live Linux run.
  > NEVER treat a Linux-only local run as proof for Windows or macOS. OS-branch code is verified only on its own runner.
- (.andromeda/test-plan.md:889) The refusal cases' oracles must be literals, the new code words and exit codes included.
  > NEVER import the product's own `IDENTITY_FLOOR`, ledger rows or refusal ordering as the test oracle. The expected values (the 11 identity-floor names, the refusal order, exit codes) are written as literals in the test.
- (.andromeda/test-plan.md:903) Waits on a socket's appearance or a killed wrapper's end must key on an explicit signal, as the connect probe `holder_gone` does, never on a sleep.
  > NEVER use `sleep(N)` for synchronization. Wait for an explicit signal: an `events.ndjson` offset, `viola wait --after` or a status field.

## Bindings
- **Exit-cause matrix ↔ the body's §6 suites and the route entry**: the new closed codes and hints land as rows of `tests/cross_exit_causes.rs` and the owed macOS case is this entry's, (.andromeda/test-plan.md:610), with the strategy file naming that binary's layer (.andromeda/test-plan-detail/test-strategy.md:44).
- **Trigger → layer map ↔ the body's §6 Security control negatives**: the IPC and filesystem rows name the same suites as the negatives list, so a negative this chunk adds moves both, (.andromeda/test-plan-detail/test-strategy.md:59), (.andromeda/test-plan-detail/test-strategy.md:64) and (.andromeda/test-plan.md:612).
- **Unit tier's macOS gating ↔ the body's §2 broad line**: the compile-out detail serves the pyramid's unit line, and the two must agree on what macOS runs, (.andromeda/test-plan-detail/test-strategy.md:10) and (.andromeda/test-plan.md:289).
- **Peer euid ↔ security plan's Vector 1**: the cross-user leg stays at the unit boundary the security excerpt's Vector names as undrivable on hosted runners, (.andromeda/test-plan.md:46), (.andromeda/test-plan.md:88) and (.andromeda/test-plan.md:380).
- **multi-os-compat ↔ §9 Matrix builds**: the trigger's map row points at the matrix, which carries the macOS and Ubuntu legs, (.andromeda/test-plan-detail/test-strategy.md:71) and (.andromeda/test-plan.md:775).
- **Refusal documents ↔ obs plan's schema and the security plan's error sanitization**: the new codes' payloads answer the sanitization expectations the body holds, (.andromeda/test-plan.md:269) and (.andromeda/test-plan.md:606).
