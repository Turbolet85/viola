## 2026-10-26-server-verification-before-any-frame — the macOS-target check-only clippy, a dev-host recipe
**Section:** §9 CI Integration (test-plan.md: Pipeline structure, the Lint line; ci-integration.md: the Lint stage)
**Change:**
- The Lint stage gains, beside the Windows-target recipe, a check-only macOS-target clippy on the Linux dev host: `CARGO_TARGET_DIR=target/maccheck cargo clippy … --target aarch64-apple-darwin`, nothing linked, the target std installed as host preparation by `rustup target add aarch64-apple-darwin`; it reproduced CI's `lint (macos-latest)` error line for line, as measured at the chunk's operator pass; neither a CI step, nor a `pre-push` stage, nor a plan gate entry.
- The master's Lint line: the `--acyclic` review and the dev-host Windows- and macOS-target check-only clippy runs are not gates (was "a dev-host Windows-target clippy").
**Why:** the chunk's pre-CI run was red on `lint (macos-latest)` alone: an unused `super::*` import in the `last.rs` and `pause.rs` test modules once their cases were gated off macOS, which no local lint could see; the macOS-target check reproduced the line exactly, and the fix gated the two modules whole.
**Kept:** the Windows-target recipe's own clause that a chunk plan may list it as a gate entry; the macOS recipe takes no such clause.
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/

## 2026-10-26-server-verification-before-any-frame — E5 steps 1–3 and the three other snapshot readers, held on "Home and code-bearing file integrity"
**Section:** §6 E2E Test Strategy (test-plan.md: Scenario: E5, As landed; e2e-test-strategy.md: the E5 procedure, the `list --json` leg) · §4 Unit Test Strategy (unit-test-strategy.md: viola-state, the Snapshot envelope item)
**Change:**
- The other snapshot readers are three (`run`'s collision check, `src/cmd/hook.rs` once, its event and dialog paths sharing the one read, and `src/cmd/hook/statusline.rs`; the rule: `read_snapshot(` under `src/`, outside test modules), each still on `read_snapshot`; was five (`src/cmd/client.rs`, the collision check, `src/cmd/hook.rs` twice, `statusline.rs`), per "2026-10-11-the-board-viola-list — the board's readings landed: E5, Path 6, Chaos; the literal pin and the kill line". The CLI frame verbs and the MCP frame tools read the snapshot through `viola_state::liveness::live_endpoint`, whose one `read_snapshot` call feeds the server check.
- E5 steps 1–3 stay unbuilt while the collision check calls `read_snapshot`: every snapshot reader stays on `read_snapshot` (option 1), and their move onto `read_snapshot_or_replay`, with what a client does with an unreadable or newer snapshot, is one CARRY on the route entry "Home and code-bearing file integrity"; was owed to "Server verification before any frame" (the e2e line's "the review card the founder approved" dropped).
**Why:** measured at this tree: `src/cmd/client.rs` holds no `read_snapshot(` call (its read moved into `live_endpoint` at chunk 2026-10-17-mcp-server-for-drivers), and this chunk joined the hook's two reads in one `connect`; the entry closes without moving any reader. Option 1 of fork CARRY 7/8 is the founder's own ruling, given by dialog at his wake on 2026-10-26 and relayed by the overseer; it supersedes the overseer's provisional delegate answer and the wrap directive's hold, so it is recorded as his word, not PROVISIONAL.
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/

## 2026-10-26-server-verification-before-any-frame — the hook fail-open matrices' impostor rows
**Section:** §6 E2E Test Strategy (test-plan.md: Non-path suites, Security sweep; e2e-test-strategy.md: the Security sweep procedure)
**Change:**
- The hook fail-open matrix is named by its functions, `hook_fails_open_silently_within_the_spine_bound` and `hook_dialog_tier_fails_open_silently` (was "the 11-case matrix").
- Its causes gain, on Linux and Windows, a server that is not the recorded wrapper (`server-verify-failed`), each exit 0 with no body, at both sites.
- The impostor rows, each under `cfg_attr(not(target_os = "macos"), …)`: `case::impostor` in the first, `question_impostor` and `permission_impostor` in the second, driving `stop`, `pre-tool-use` and `permission-request` against a same-user server on the recorded endpoint while the snapshot records another live process (the fake agent's `--hold-stdout-internal` holder): each exits 0 with empty streams under the 1.0 s `Instant` bound, no frame reaches the server, and one `hook-decision` line carries `detail:"server-verify-failed"` (on the dialog path after `hook-invoked{corr:null}`, with `decision_emitted:false`).
- Not a refusal case: a hook reaching an exiting wrapper's own Windows pipe passes the server check by design, the exiting wrapper being the recorded process; whether sysinfo still lists a terminated Windows process whose pipe handle is open, and SessionEnd's outcome in that window, are one CARRY on the route entry "Windows-only live measurements".
**Why:** the chunk added the three impostor rows to the two existing matrices; a master states no test-case count, so the count gives way to the function names. The exiting-wrapper window's re-pin is a Beyond-the-plan row of the P5 review card, on the overseer's founder-delegated word.
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/

## 2026-10-26-server-verification-before-any-frame — the exit-cause matrix as landed: no `cross_exit_causes.rs`, the server-verification cause
**Section:** §6 E2E Test Strategy (test-plan.md: Non-path suites, Exit-cause matrix; e2e-test-strategy.md: the Exit-cause matrix procedure)
**Change:**
- `cross_exit_causes.rs` is marked not built in the master and recorded as absent, with no rstest table joining the causes: each built cause is a case of the binary its row names.
- The server-verification exit-21 cause, on Linux and Windows, is the six-verb server-identity negative and its control in `tests/security_negatives_server_identity.rs`, not a `cross_exit_causes.rs` row.
- macOS: the client reads no peer pid and compares no server (the recorded gap); the client euid check and the `#[cfg(target_os = "macos")]` case loosening the per-user socket dir to 0755 → exit 21 are not built, one CARRY on the route entry "Unix endpoint and home hardening" (was the client checking euid and the socket dir, the 0755 case written as asserted).
- The cause's hint is its own, cause `server-verify` in `src/human/hints.rs` (the design-system surface cli-send string, byte for byte), apart from a dead instance's not-running hint; its line is `process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}` at WARN in `cli-<name>.ndjson`; its `--json` document is every exit-21 cause's `{"v":1,"error":"instance-unreachable","detail":null}`.
**Why:** the report disproved the case's home: it landed in the new security-negative binary, six verbs in two modes, and `cross_exit_causes.rs` does not exist. The macOS re-pin is a Beyond-the-plan row of the P5 review card, on the overseer's founder-delegated word; its clause, read at first as contradicting the line it rewrites, was applied with its companion on the overseer's founder-delegated answer at the wrap's halt.
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/

## 2026-10-26-server-verification-before-any-frame — the server-identity security negative and the `security_negatives` suite prefix
**Section:** §6 E2E Test Strategy (test-plan.md: Non-path suites, Security control negatives; e2e-test-strategy.md: the Security control negatives procedure) · §2 Test Strategy (test-strategy.md: the naming pattern, the sync E2E suites)
**Change:**
- Security control negatives gain, on Linux and Windows, a CLI frame verb writing no frame to a server that is not the recorded wrapper (`tests/security_negatives_server_identity.rs`, beside its control reaching the recorded wrapper).
- Its procedure, **Server identity** (`#![cfg(not(target_os = "macos"))]`, IPC client-side server verification): `security_negative_cli_verb_writes_no_frame_to_a_server_that_is_not_the_recorded_wrapper` runs `send`, `wait`, `last`, `answer`, `pause` and `release`, each under `--json` and in human mode, against a same-user server bound on the recorded endpoint while `snapshot.json` records another live process (the fake agent's `--hold-stdout-internal` holder): exit 21 with the cause's `--json` document (stderr empty), hint and `process-exit` line, no `channel-request` line, no frame reaching the server, and no pid, start time, path or endpoint in any output or line (`digit_runs`). Its control `security_negative_cli_verb_reaches_the_recorded_wrapper` runs each verb in both modes against the recorded wrapper itself, which receives one frame each.
- The root sync E2E suites read `tests/{cli,hook,tui,channel,chaos,contract,security_negatives}_<topic>.rs` (was without `security_negatives`).
**Why:** the chunk's new test binary, green on the host and on CI's ubuntu-latest and windows-2025 legs; it retires on Linux and Windows the server-identity half of the dated liveness-only gaps for the six verbs.
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/

## 2026-10-26-server-verification-before-any-frame — the Wrapper channel's server-mismatch cases and the MCP impostor case
**Section:** §5 Integration Test Strategy (integration-test-strategy.md: the Wrapper channel cases)
**Change:**
- A server pid / start-time mismatch versus `snapshot.json` gives CLI exit 21 and the hook exits 0 with no body, on Linux and Windows: the CLI half in `tests/security_negatives_server_identity.rs`, the hook half the impostor rows of `tests/hook_fail_open.rs` (§6 Security sweep). macOS reports no peer pid, so there the check passes with no comparison, the recorded gap; its euid and directory check is owed to the route entry "Unix endpoint and home hardening". Was "macOS verifies euid + dir only", and the MCP half "the same impostor case in `cross_exit_causes.rs`, the crate with the rmcp client", `call_tool("send")`.
- The MCP half: `handlers_frame_tools_write_no_frame_to_a_server_that_is_not_the_recorded_wrapper` (`crates/viola-mcp/tests/handlers.rs`, a held process as the recorded pid) drives the transport-free `Tools` handle, no rmcp client: MCP `send`, `wait`, `last` and `answer` against a server that is not the recorded wrapper each get `isError` with `{"error":"instance-unreachable","detail":null}` and text `refused: instance-unreachable`; no frame reaches the server and no log line is written.
**Why:** the report disproved the MCP case's home: it landed in the crate's handler tests over the `Tools` handle, and `cross_exit_causes.rs` does not exist; the macOS client reads no peer pid, and its euid check belongs to that route entry.
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/

## 2026-10-26-server-verification-before-any-frame — the server-identity impostor: its held process, its listener, its test-built snapshot
**Section:** §8 Mocking & Stubbing Discipline (mocking-stubbing-discipline.md: the Process mocking and Fake servers rows) · §7 Test Data & Fixtures (test-plan.md: Seed strategies, On-disk product state; test-data-fixtures.md: the On-disk product state row, a new data item, the fake agent's modes)
**Change:**
- Process mocking: a held live process whose pid an impostor's snapshot records is a real process, never a double: in root tests the fake agent's `--hold-stdout-internal` holder, in `crates/viola-mcp/tests/handlers.rs` its own `Held` child, entered at `handlers_held_child_entry` only when its `HELD` variable is set, each killed and reaped by a `Drop` guard.
- Fake servers gain the impostor listener: in the server-verification negatives (Linux and Windows) a server on the recorded endpoint while the snapshot records another held live process stands in for a server that is not the recorded wrapper (root tests bind a test-owned `viola_channel::Server::bind` listener in `tests/security_negatives_server_identity.rs` and `tests/hook_fail_open.rs`'s `Impostor`; the MCP case is the handlers impostor case); the hook unit impostor varies the recorded start time instead (a second early, or an unparseable `started_at`), a root-bin unit test having no portable second process.
- On-disk product state: three exceptions to "never hand-written" (was two), the impostor cases added: `served` and `Impostor::serve` write an `InstanceSnapshot` themselves, recording a held live process's pid beside an endpoint the test binds. The master's seed line names the impostors' test-built `snapshot.json`.
- The `--exit-no-eof` holder's re-exec mode `--hold-stdout-internal` (sleeps 10 s, writes nothing) is also spawned directly by the two root impostor files, killed and reaped by each file's `Held` `Drop` guard.
**Why:** an impostor needs a snapshot recording a live pid that is not the server's, which no product run writes; the chunk's held process is the holder mode, not a bare fake-agent spawn, and its hook unit impostor varies the start time.
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/

## 2026-10-26-server-verification-before-any-frame — the hook perf rows with the server check in the timed path
**Section:** §10 Quality Gates & Coverage Targets (quality-gates-coverage-targets.md: the Perf session)
**Change:**
- Every timed hook also makes its server check (the peer pid and its OS start time against the one snapshot read) before its first frame; with the check in the timed path the hook rows read `max` 2.2–2.4 ms over 30 runs each on the Linux dev host, as measured at the chunk's `run --perf`.
**Why:** the chunk put the check ahead of every hook frame; `run --perf` stayed green, every hook row far under the 1 s `SPINE_DEADLINE` with zero panics, against 1.23–1.6 ms read before the check landed.
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/

## 2026-10-26-server-verification-before-any-frame — fake-agent copies planted one after another (ETXTBSY)
**Section:** §7 Test Data & Fixtures (test-data-fixtures.md: a new data item)
**Change:**
- A test that plants more than one fake-agent copy plants every copy (each recorded set's bin) one after another before any thread starts a child: on the tmpfs `target/e2e-home` the hard link fails, so each copy is written, and a child one thread forks inherits another thread's open write handle (ETXTBSY). In `fake_agent_agents_replay_matches_every_recorded_agents_set` (`tests/contract_fake_agent_drift.rs`) it reads 0 red of 40 standalone runs, as measured at the chunk's ETXTBSY evidence.
**Why:** the chunk's pre-push red was ETXTBSY in that test, 6 of 40 standalone runs red before the sequential planting; the test-only fix was folded into this chunk by the standing rule that a red met during a chunk folds into it, on the overseer's founder-delegated word, recorded in-intent.
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/

## 2026-10-26-server-verification-before-any-frame — where the server verification is asserted: the viola-channel entity and the V1 row
**Section:** §1 Test Scope Summary (test-scope-summary.md: the `viola-channel` entity, a new As landed item) · §2 Test Strategy (test-strategy.md: the trigger table, the IPC vector (V1) row)
**Change:**
- As landed (server verification): the pid/start-time server verification is asserted on Linux and Windows — the six CLI frame verbs by `tests/security_negatives_server_identity.rs` (an impostor gets no frame and the verb exits 21, beside a control reaching the recorded wrapper), the hook by `tests/hook_fail_open.rs`'s impostor rows (exit 0, no body, no frame), the MCP frame tools by `crates/viola-mcp/tests/handlers.rs` (`instance-unreachable`, no frame), and the decision table by the `viola-state` unit `server_verdict_rows`. These negatives compile out on macOS, where the client reads no peer pid; the macOS cases are owed (§6 Exit-cause matrix).
- The V1 row's plan location gains §5 Wrapper channel (the MCP impostor case), root `security_negatives_server_identity.rs` (off macOS) under Security control negatives, and the hook fail-open impostor rows (off macOS) under Security sweep.
**Why:** the chunk built the verification's negatives on all three frame-writing surfaces, each compiled out on macOS, where there is no peer pid to compare.
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/

## 2026-10-26-server-verification-before-any-frame — the server check's captured-span cases compiled out on macOS
**Section:** §2 Test Strategy (test-plan.md: Test pyramid, Unit; test-strategy.md: the Unit level, Tier-specific depth)
**Change:**
- Unit runs on all three OSes except the two real-cargo-mutants harness tests and the server check's captured-span cases, compiled out on macOS (was the mutants tests alone), where the server check reads no peer pid: each frame verb's `request_refuses_an_unverified_server_inside_its_dispatch_span`, `hook_refuses_an_unverified_server_inside_hook_handle`, the test-only `cmd::client::refused_inside` helper they share (under `#[cfg(test)]` + `#[cfg(not(target_os = "macos"))]`) and the `last.rs` and `pause.rs` test modules, gated whole; the decision table `server_verdict_rows` and the macOS `server_pid` case still run there.
**Why:** macOS reports no peer pid, so the check passes there with no comparison and these cases have nothing to refuse. The helper's gate is two attributes because a first `#[cfg(all(test, not(target_os = "macos")))]` gate put its `cfg!(windows)` ahead of the file's first `#[cfg(test)]` and the Windows mutation-scope contract read the file as Windows-gated product code; the two modules are gated whole because a per-case gate left an import unused on macOS.
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/

## 2026-10-26-server-verification-before-any-frame — the server pid and the pure server verdict, unit cases
**Section:** §4 Unit Test Strategy (test-plan.md: the viola-channel and viola-state lines; unit-test-strategy.md: viola-channel, viola-state)
**Change:**
- viola-channel: the connected stream's server pid, `Client::server_pid` and `TokioClient::server_pid` (`peer_creds().pid()`: Linux `SO_PEERCRED`, Windows `GetNamedPipeServerProcessId` on the SQOS-adopted client end, macOS none), asserted on each OS by `client_server_pid_is_the_serving_process` and `client_server_pid_is_none_where_peer_credentials_hold_no_pid` (`crates/viola-channel/src/client.rs`) and their Tokio twins `tokio_client_server_pid_is_the_serving_process` and `tokio_client_server_pid_is_none_where_peer_credentials_hold_no_pid` (`tokio_client.rs`, behind the `tokio` feature); `SERVER_PID_REPORTED` is false on macOS.
- viola-state: `ServerVerdict { Matches, Mismatch, Unverifiable }` from the pure `server_verdict(recorded_pid, started_at, observed_pid, observed_start, peer_pid_reported)`, pinned by the decision table `server_verdict_rows`; with no peer pid reported (`peer_pid_reported` false, macOS) the verdict is `Matches` with no comparison, the recorded gap. `verify_server_reads_the_observed_start_time_from_the_os` covers the joined `verify_server`'s OS read (`process_start_time`, none for a zombie). `live_endpoint` returns `LiveEndpoint { endpoint, pid, started_at }`, all from its one `read_snapshot` call.
- The master's lines gain the connected stream's server pid on each OS (viola-channel) and the pure server verdict's decision table (viola-state).
**Why:** the chunk's new channel and state surface; the decision table is named without its row count, since a master states no test-case count.
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/

## 2026-10-26-server-verification-before-any-frame — the root bin's server-check unit cases
**Section:** §4 Unit Test Strategy (test-plan.md: the root bin line; unit-test-strategy.md: root bin, two new items)
**Change:**
- The server check before any frame: `request_refuses_an_unverified_server_inside_its_dispatch_span` in each of `src/cmd/{send,wait,last,answer,pause,release}.rs` and `hook_refuses_an_unverified_server_inside_hook_handle` (`src/cmd/hook.rs`) assert the refusal inside the verb's or the hook's span with no frame written, each through the shared test-only `cmd::client::refused_inside` helper (`assert_refused_inside`; compiled out on macOS); `unreachable_logs_its_cause` (`src/cmd/client.rs`) logs each closed `Unreachable` cause (`instance-dead` · `server-verify-failed`, the server cause `during` `connect`).
- The hook's server check: `ask_writes_no_frame_to_a_server_that_is_not_the_recorded_wrapper` (the dialog path writes no frame and emits no decision), and a SessionEnd whose check fails takes the direct append (`append_session_end`), held by a mismatch row of the hook's unit cases; the unit impostor varies the recorded start time (a second early; an unparseable `started_at`) rather than recording another live pid, the integration rows' shape.
- The master's root bin line gains the server check refused inside each frame verb's and the hook's span, with each closed unreachable cause logged.
**Why:** the chunk's root-bin surface; the seven captured-span cases and their helper were added beyond the plan on the operator's check, each red with the check removed, and a root-bin unit test has no portable second process to record.
**Ref:** .andromeda/runs/2026-10-26T09-14-57-wrap/
