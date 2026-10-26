# tests · Integration Test Strategy — category read

## Carried
- (.andromeda/test-plan-detail/integration-test-strategy.md:47) The chunk's own integration case: a server pid / start-time mismatch against `snapshot.json` must give CLI exit 21 and a hook exit 0 with no body on Linux and Windows (macOS euid + dir only), and the MCP half must be the same impostor case in `cross_exit_causes.rs` with the coded `instance-unreachable` result.
  > a server pid / start-time mismatch versus `snapshot.json` gives CLI exit 21, and the hook exits 0 with no body (Linux and Windows; macOS verifies euid + dir only, see the §6 exit-cause matrix). The MCP half is the same impostor case in `cross_exit_causes.rs`, the crate with the rmcp client: `call_tool("send")` gets `is_error:true` with `{"error":"instance-unreachable","detail":null}`.
- (.andromeda/test-plan-detail/integration-test-strategy.md:42) The verification must be exercised over every client method the wrapper channel serves, through the crate and root channel suites.
  > `send`/`wait`/`last`/`answer`/`pause`/`release`/`link`/`unlink`/`hook.dialog`/`hook.event` via `crates/viola-channel/tests/` and root `tests/channel_*.rs`
- (.andromeda/test-plan-detail/integration-test-strategy.md:43) Any result, refusal or error shape the verification adds or touches on the channel must stay schema-validated.
  > result, refusal and error shapes schema-validated
- (.andromeda/test-plan-detail/integration-test-strategy.md:18) An impostor or squatting server in a test must be a real endpoint created through the interprocess server API in a temp home, never via `try_overwrite`.
  > Wrapper channel: real endpoint bound by `viola-channel` server in a temp home, with frames validated by jsonschema. The squatter listener is created first via the interprocess server API, never `try_overwrite`.
- (.andromeda/test-plan-detail/integration-test-strategy.md:25) The hook client's verification must be driven as `viola hook <event>` on fixture stdin with `VIOLA_NAME`/`VIOLA_DIR` against a live test wrapper, its event line and hook trace asserted.
  > `viola hook <event>` fed fixture stdin with `VIOLA_NAME`/`VIOLA_DIR` against a live test wrapper. The event line, the `diagnostics/hook-<name>.ndjson` trace and `budget.json` are asserted.
- (.andromeda/test-plan-detail/integration-test-strategy.md:32) Each verification case must run in a fresh tempfile home of its own.
  > there is a fresh tempfile home per test (the rstest `home` fixture).
- (.andromeda/test-plan-detail/integration-test-strategy.md:33) If the client verification takes a strict-modes check with it, a case that drives that check-guarded path on every OS must start from `stamped_home`, because a seeded Windows home fails the instance check; started wrappers stay in a kill-then-wait `Drop` guard.
  > Such a home fails `hook statusline`'s instance check on Windows (as measured at chunk 2026-10-10-statusline-pass-through, CI run 38080061631, three cases refused; the home's DACL not read), so a case that drives a check-guarded path on every OS starts from `stamped_home` (§7 Seed strategies).
- (.andromeda/test-plan-detail/integration-test-strategy.md:33) Test wrappers must be held in the kill-then-wait guard, with nothing shared between tests.
  > Wrappers started in a test are held in a `Drop` guard that calls `Child::kill()` then `wait()`. Nothing is shared between tests.
- (.andromeda/test-plan-detail/integration-test-strategy.md:56) If the strict-modes half lands here, its refusals must keep the standing exit split: hook 0 with no output, CLI 21 for every verb other than `run` / `mcp` / `revive`.
  > hook 0 with no output (the statusline arm reads and opens nothing under the home after a refused check and writes no line, however often it is invoked), CLI 21 for every other verb
- (.andromeda/test-plan-detail/integration-test-strategy.md:80) The machine-contract tables that pin each driver verb's document and exit over a live wrapper, a home with no instance and a home another user could write must keep passing, or be restated, once the clients verify the server.
  > `tests/cli_machine_contract.rs` reads each driver-facing verb's `--json` document and literal exit over one live wrapper's life (`every_verb_prints_one_document_and_its_literal_exit_over_a_wrapper_s_life`, a labelled case table per world), over a home with no instance (`verbs_over_a_home_with_no_instance_print_their_documents`) and over a home another user could write (`verbs_over_a_home_another_user_could_write_print_their_documents`)
- (.andromeda/test-plan-detail/integration-test-strategy.md:80) A test-started client must drop the inherited `VIOLA_*` names and compare paths by `resolved(home)`.
  > A test-started `viola` drops `VIOLA_NAME`, `VIOLA_DIR` and `VIOLA_BIN` (`tests/support/home.rs` `UNWRAPPED`) and a path oracle compares `resolved(home)`, never `TestHome::path()`.
- (.andromeda/test-plan.md:46) Server verification must be tested on the same-user path, with the cross-user part asserted at the peer-credential check boundary.
  > same-user framing, dispatch, refusal order, server verification and the squatted-name refusal are testable, and the Windows listener rejecting remote pipe clients and its pipe DACL denying an anonymous open are asserted on `test (windows-2025)`; only the cross-user part (a peer of another euid) cannot be driven on hosted runners and is asserted at the peer-credential check boundary.
- (.andromeda/test-plan.md:128) The values the client compares are the wrapper `pid` and `started_at` that `snapshot.json` records, and a pid + start-time that no longer names a running process reads `gone`.
  > There is no standalone pid file. The wrapper `pid`, `started_at` and `child_pid` live in `<home>/instances/<name>/snapshot.json`, with liveness in `<home>/instances/<name>/heartbeat` (touched every 1 s): when pid + start time no longer name a running process the instance is `gone` whatever the beat's age; otherwise a beat at most 5 s old is `live`, and an older or absent one `stale`.
- (.andromeda/test-plan.md:145) A killed wrapper must still read CLI exit 21 and MCP `instance-unreachable` after the verification is added.
  > Killing the wrapper mid-wait gives exit 21 / MCP `instance-unreachable`.
- (.andromeda/test-plan.md:184) The IPC coverage trigger names this chunk's negative across all three clients.
  > server pid / start-time mismatch vs `snapshot.json` → CLI exit 21 / MCP `instance-unreachable` / hook exit 0 with no body
- (.andromeda/test-plan.md:193) The hook's fail-open matrix must hold with the verification in its path: exit 0, empty stderr, no body.
  > fail-open tests on oversize stdin, malformed JSON, clap error, channel failure, a forced panic (on the real binary, through the `fake-agent`-only `FAKE_AGENT_HOOK_PANIC` seam; see §6 Security sweep), and a missing `VIOLA_NAME`. Each must give exit 0 with empty stderr and no body.
- (.andromeda/test-plan.md:201) A strict-modes failure reached through `--home` / `VIOLA_DIR` must keep its exit split if the client check reads strict-modes.
  > `--home` / `VIOLA_DIR` failing strict-modes → exit 1 / 21 / hook 0
- (.andromeda/test-plan.md:237) The chaos trigger's vanishing endpoint and stale-heartbeat / live-pid distinction must stay as they are under the new check.
  > an endpoint vanishing during `wait` read `instance-unreachable`, and the stale-heartbeat / live-pid distinction.
- (.andromeda/test-plan.md:250) macOS's euid + dir-only verification is an OS-branch test that must run on its own runner, beside the named-pipe and Unix-socket branches.
  > OS-branch tests cover named pipe vs Unix socket, ConPTY vs openpty, Windows DACL / strict-modes vs Unix modes, and macOS euid + dir-only server verification.
- (.andromeda/test-plan.md:320) New integration binaries must take the layer prefixes so the filtersets select them.
  > Rust integration binaries use a `<surface>_<topic>.rs` prefix so nextest filtersets can select layers: `cli_`, `hook_`, `tui_`, `channel_`, `chaos_`, `contract_` in root; `path_`, `mcp_`, `cross_` in `viola-e2e`.
- (.andromeda/test-plan.md:323) New test functions must follow the naming pattern.
  > General pattern: snake_case `<subject>_<condition>_<expected>`, for example `send_with_esc_refuses_control_character` or `hook_oversize_stdin_exits_zero_silent`.
- (.andromeda/test-plan.md:376) The verification's peer decision must be unit-tested with injected euids in `viola-channel`, below the integration case.
  > **viola-channel:** the ndjson frame codec at the `MAX_FRAME` boundary, the JSON-RPC error mapping, the FNV-1a endpoint golden vectors, the peer-credential decision with injected euids, and Windows handle non-inheritance.
- (.andromeda/test-plan.md:378) Any liveness or instance-check logic the client reuses is unit-tested in `viola-state` without a clock.
  > the pure liveness classifier fed ages directly (no clock), Unix strict-modes and Windows strict-modes over synthetic SDDL descriptors, and the instance check.
- (.andromeda/test-plan.md:435) Server verification must be one of the wrapper channel's cross-module patterns, covered through the crate and root channel suites.
  > **Wrapper channel:** every method through the crate and root channel suites, with schema-validated result, refusal and error shapes, the `cursor` equal to the log length at acceptance, `-32602` on a higher `v`, `-32600` and a close on an oversize frame, server verification, and wrapper-side paste validation in which `control-character` wins over every other refusal.
- (.andromeda/test-plan.md:449) The MCP handler tests run the Tokio client against a `TestWrapper` whose snapshot comes from `write_snapshot`, so that snapshot must still pass the new pid + start-time check (or the harness must be restated).
  > **MCP:** adapter-level handler tests in `crates/viola-mcp/tests/handlers.rs` drive the transport-free `viola_mcp::Tools` handle (the same `Server::call` `serve` answers with) against a `viola_channel::Server` test wrapper (`TestWrapper`, its snapshot written through `viola_state::snapshot::write_snapshot`); the handlers under test use the Tokio client (`viola_channel::tokio_client::TokioClient`, `viola-channel`'s `tokio` feature), never the sync client (chunk 2026-10-17-mcp-server-for-drivers).
- (.andromeda/test-plan.md:468) A channel test client must take the endpoint only from `snapshot.json`.
  > the sync client, a raw interprocess stream and jsonschema; the endpoint is read only from `snapshot.json`.
- (.andromeda/test-plan.md:590) E5 steps 1–3 are owed to this entry, so the plan must either build them or name them unclosed, under the scope's held CARRY on the replay.
  > **As landed: in part.** The replay's product readers (`viola revive`, `viola list`), its budget and link legs and its `list --json` leg are covered; steps 1–3 stay unbuilt while `run`'s collision check calls `read_snapshot`, owed to the route entry "Server verification before any frame".
- (.andromeda/test-plan.md:592) Whatever builds E5 must restate the two signals that do not match what landed.
  > Two of the signals above are set against what landed, for the entry that builds it to restate: every start appends `wheel{holder:"driver", cause:"start"}`, so `wheel:"human"` does not hold after step 3, and the replay writes no file, so no snapshot is rebuilt from it.
- (.andromeda/test-plan.md:598) The security sweep's hook fail-open matrix must keep passing with the verification on the hook path.
  > the hook fail-open matrix — oversize stdin, malformed JSON, clap error, channel failure, a forced panic and a missing `VIOLA_NAME`, each exit 0 with no body
- (.andromeda/test-plan.md:604) The impostor cause of exit 21 must be its own row of the exit-cause matrix, with a `hint:` line that names that cause alone.
  > **Exit-cause matrix** (`cross_exit_causes.rs`): one negative test per cause of exit 21 and of exit 1, each asserting the exit code, the `--json` document and a last `hint:` line that names its cause — a hint shared across causes fails the table; `revive`'s refusals are cases of `tests/cli_revive.rs`; driver-facing hints never contain `release`.
- (.andromeda/test-plan.md:606) A security-plan control that lands with no negative test needs one in `security_negatives_*.rs`.
  > **Security control negatives** (`security_negatives_*.rs`): one negative test per security-plan control that had none
- (.andromeda/test-plan.md:631) The test home and its product state must come from real `viola` runs, starting from `stamped_home` where a case needs a home viola created.
  > **On-disk product state:** produced only by real `viola` runs against the fake agent in a fresh home, never hand-written — except the chaos tests' corruption of files the product already wrote and the Windows x64 `seed_conpty` companions; a case that needs a home viola created starts from `stamped_home`.
- (.andromeda/test-plan.md:905) The pid / start-time mismatch must be produced without hand-writing `snapshot.json` in any booted-wrapper test; only a reader's own unit cases may alter the writer's bytes.
  > NEVER hand-write `ledger/stamps.json`, `snapshot.json` or `budget.json`. Stamps come from `viola verify`, snapshots from the wrapper and budget from `viola hook statusline`. The ban binds every booted-wrapper test. A reader's own unit cases may plant a malformed state file only by altering the writer's own bytes: write through the writer, then pad, replace or re-version (the operator's answer at the chunk 2026-10-10-budget-governor wrap, where `budget_read_*` landed in that form).
- (.andromeda/test-plan.md:889) Impostor tests must not reuse an endpoint name across tests and must not use `try_overwrite`.
  > NEVER use interprocess `try_overwrite`, or reuse an endpoint name across tests. Take the endpoint only from `snapshot.json`.
- (.andromeda/test-plan.md:891) Hook and CLI cases that clear the environment must re-add `LLVM_PROFILE_FILE`.
  > NEVER call `.env_clear()` without re-adding `LLVM_PROFILE_FILE`, because that run silently drops out of coverage.
- (.andromeda/test-plan.md:868) Decision-level cases (the match verdict) belong in §4 units, and contract shapes in §5.
  > NEVER assert at E2E a case that a lower layer can hold. Parser, refusal-order, threshold and clock cases belong in §4, contract shapes in §5, and §6 asserts only cross-process behaviour (§2 pyramid).
- (.andromeda/test-plan.md:873) The Windows pipe-server-pid and macOS branches are verified only on their own CI runners, never by a Linux local run.
  > NEVER treat a Linux-only local run as proof for Windows or macOS. OS-branch code is verified only on its own runner.
- (.andromeda/test-plan.md:834) The verification added to the hook's connect path must stay inside the hook's connect deadline, below the spine deadline.
  > The hook's own connect deadline is the crate-private, provisional `CONNECT_DEADLINE` = 750 ms in the root bin's `src/cmd/hook.rs`, below the gate and asserted below it by `connect_deadline_is_below_the_spine_deadline`.
- (.andromeda/test-plan.md:840) The spine hook perf rows must keep their `max` under `SPINE_DEADLINE` with the verification added.
  > | Spine hooks `session-start`, `user-prompt-submit`, `stop` (same method, `VIOLA_NAME`/`VIOLA_DIR` exported to hyperfine) | mean, median, stddev | `max <` `viola_core::SPINE_DEADLINE` (1.0 s) |
- (.andromeda/test-plan.md:813) New code must hold the per-OS coverage gates.
  > Gates are enforced per OS job with `cargo llvm-cov nextest … --fail-under-lines 85 --fail-under-functions 95 --fail-under-regions 80`.

## Bindings
- **Impostor case ↔ §6 exit-cause matrix (body)**: the MCP half of the mismatch case lives in `cross_exit_causes.rs` (.andromeda/test-plan-detail/integration-test-strategy.md:47), and that matrix requires one row per exit-21 cause with a hint of its own (.andromeda/test-plan.md:604).
- **Impostor case ↔ security-plan §Local trust boundary**: the case is the test for the clause "Before the FIRST frame of any method … verify the server" and for closing the dated exceptions; the integration case is at (.andromeda/test-plan-detail/integration-test-strategy.md:47) and the coverage trigger at (.andromeda/test-plan.md:184).
- **Strict-modes refusals ↔ route entry "Home and code-bearing file integrity"**: whichever side the strict-modes half lands on, the exit split stays as (.andromeda/test-plan-detail/integration-test-strategy.md:56) and (.andromeda/test-plan.md:201) state it, and a check-guarded case on every OS starts from `stamped_home` (.andromeda/test-plan-detail/integration-test-strategy.md:33) ↔ §7 Seed strategies (.andromeda/test-plan.md:631).
- **MCP handler harness ↔ the MCP frame tools (eighth exception)**: the `TestWrapper` snapshot written through `write_snapshot` must satisfy the Tokio client's new check (.andromeda/test-plan.md:449).
- **E5 steps 1–3 ↔ scope CARRY 7 / 8 (held for the founder)**: E5's unbuilt steps are owed to this entry (.andromeda/test-plan.md:590), and building them means the client move to the replay, which the scope holds behind a stop rule; the signals to restate are at (.andromeda/test-plan.md:592).
- **Impostor construction ↔ §11 Test Data ban**: the mismatch must come from a real endpoint and a product-written snapshot, never a hand-written one (.andromeda/test-plan.md:905), (.andromeda/test-plan-detail/integration-test-strategy.md:18).
- **Hook verification ↔ §10 perf gate**: the added check sits inside `CONNECT_DEADLINE` and the spine rows' `max` (.andromeda/test-plan.md:834), (.andromeda/test-plan.md:840).
- **macOS euid + dir branch ↔ §9 CI matrix**: the macOS gap is an OS-branch test run on `macos-latest` only (.andromeda/test-plan.md:250), (.andromeda/test-plan.md:873).
