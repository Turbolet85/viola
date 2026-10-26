proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-strategy.md
    near: 10
    section: '## 2. Test Strategy'
    change: 'Tier-specific depth: ... Runs on all 3 OSes, except the two real-cargo-mutants harness tests (§10 Mutation gate) and the server-verification unit cases, which compile out on macOS, where the server check has no peer pid (the test-only `cmd::client::refused_inside` helper under `#[cfg(test)]` + `#[cfg(not(target_os = "macos"))]`, the verbs'' captured-span cases through it, and the `last.rs` and `pause.rs` test modules gated whole; chunk 2026-10-26-server-verification-before-any-frame).'
    sidecar: 'test-strategy Unit depth: the macOS compile-out names the server-verification unit cases beside the two cargo-mutants tests.'
    rationale: 'Report Reverted bullet: `refused_inside` restacked as `#[cfg(test)]` + `#[cfg(not(target_os = "macos"))]`, and the `last.rs` / `pause.rs` test modules'' per-case macOS gate replaced by a whole-module gate (`775641fa`); Deviations 8; Outcome: macos-latest reads 10 new-case PASS lines against 39 on the other legs. «except the two real-cargo-mutants harness tests» no longer names every unit case compiled out on macOS.'
    basis: .andromeda/test-plan-detail/test-strategy.md:10
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan.md
    near: 285
    section: '## 2. Test Strategy'
    change: '**Unit:** ... on all three OSes except the two real-cargo-mutants harness tests and the server-verification unit cases, compiled out on macOS (no peer pid there).'
    sidecar: 'test-plan §2 Unit pyramid: the macOS compile-out names the server-verification unit cases.'
    rationale: 'The broad line the test-strategy Unit depth details; same cause (report Reverted bullet, Deviations 8).'
    dependent-of: D-cascade
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-strategy.md
    near: 42
    section: '## 2. Test Strategy'
    change: '**Sync (tokio-free) E2E suites:** root `tests/{cli,hook,tui,channel,chaos,contract,security_negatives}_<topic>.rs`, sharing helpers in `tests/support/...` (rest unchanged).'
    sidecar: 'test-strategy naming: the root sync-suite prefix list gains `security_negatives`.'
    rationale: 'Report Changes › Files: new root test binary `tests/security_negatives_server_identity.rs` (Harness / gate surface: «New test binary `security_negatives_server_identity` (root, `#![cfg(not(target_os = "macos"))]`)»); its prefix is not in the listed root prefixes.'
    basis: .andromeda/test-plan-detail/test-strategy.md:42
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 13
    section: '## 4. Unit Test Strategy'
    change: 'Add under viola-channel: - The connected stream''s peer pid (chunk 2026-10-26-server-verification-before-any-frame): `client_server_pid_is_the_serving_process` and `client_server_pid_is_none_where_peer_credentials_hold_no_pid` (`crates/viola-channel/src/client.rs`), with their Tokio twins `tokio_client_server_pid_is_the_serving_process` and `tokio_client_server_pid_is_none_where_peer_credentials_hold_no_pid` (`tokio_client.rs`, behind the `tokio` feature), assert `Client::server_pid` / `TokioClient::server_pid` (`peer_creds().pid()`: Linux `SO_PEERCRED`, Windows `GetNamedPipeServerProcessId` on the SQOS-adopted client end, macOS none) on each OS; `SERVER_PID_REPORTED` is false on macOS.'
    sidecar: 'unit-test-strategy viola-channel: the peer-pid cases and SERVER_PID_REPORTED added.'
    rationale: 'Report Symbols / APIs (client.rs 57-61, 132-139; tokio_client.rs 44-47) and New text, by line (client.rs 562-569, 571-577; tokio_client.rs 178-193, 195-210); Coverage of new surfaces: «tests unit (each OS: the serving process''s pid; macOS none)».'
    fact: F9
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan.md
    near: 376
    section: '## 4. Unit Test Strategy'
    change: '**viola-channel:** the ndjson frame codec at the `MAX_FRAME` boundary, the JSON-RPC error mapping, the FNV-1a endpoint golden vectors, the peer-credential decision with injected euids, the connected stream''s peer pid on each OS, and Windows handle non-inheritance.'
    sidecar: 'test-plan §4 viola-channel broad line: the peer pid added.'
    rationale: 'The broad line the viola-channel unit group details; report Symbols / APIs (`Client::server_pid`, `TokioClient::server_pid`).'
    dependent-of: D-new-detail
  - detector: D-tests-count-literals
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 18
    section: '## 4. Unit Test Strategy'
    change: 'Replace the last sentence: The three other snapshot readers (`run`''s collision check, `src/cmd/hook.rs` once — its event and dialog paths share one read since chunk 2026-10-26-server-verification-before-any-frame — and `src/cmd/hook/statusline.rs`; the rule: `read_snapshot(` under the root `src/`, outside test modules) still call `read_snapshot`; the CLI frame verbs and the MCP frame tools read the snapshot through `viola_state::liveness::live_endpoint` (since chunk 2026-10-17-mcp-server-for-drivers), which holds the one `read_snapshot` call that feeds the server check.'
    sidecar: 'unit-test-strategy viola-state: the other snapshot readers are three (run''s collision check, hook.rs once, statusline.rs), not five; src/cmd/client.rs holds no read.'
    rationale: 'Report Counts / qualifiers moved (pattern `readers`): `read_snapshot(` under `src/` outside test modules reads 3 product callers — `src/cmd/hook.rs:204`, `src/cmd/hook/statusline.rs:142`, `src/cmd/run.rs:379`; `src/cmd/client.rs` gone since chunk 2026-10-17-mcp-server-for-drivers; hook.rs''s two moved to one; site listed: test-plan-detail/unit-test-strategy.md:18. Symbols: `live_endpoint` reads from the one `read_snapshot` call.'
    basis: .andromeda/test-plan-detail/unit-test-strategy.md:18
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 20
    section: '## 4. Unit Test Strategy'
    change: 'Add after the liveness classifier item: - The server check (chunk 2026-10-26-server-verification-before-any-frame): `pub enum ServerVerdict { Matches, Mismatch, Unverifiable }` and the pure `server_verdict(recorded_pid, started_at, observed_pid, observed_start, peer_pid_reported)`, pinned by the decision table `server_verdict_rows` (seven rows); where no peer pid is reported (`peer_pid_reported == false`, macOS) the verdict is `Matches` with no comparison, the recorded gap; `verify_server_reads_the_observed_start_time_from_the_os` covers the joined `verify_server`''s OS start-time read (`process_start_time`, a zombie none). `live_endpoint` returns `LiveEndpoint { endpoint, pid, started_at }`, all from its one `read_snapshot` call.'
    sidecar: 'unit-test-strategy viola-state: the ServerVerdict decision table and verify_server added; live_endpoint returns LiveEndpoint.'
    rationale: 'Report Symbols / APIs (liveness.rs 102-111, 113-140, `verify_server`, `LiveEndpoint`), New text, by line (liveness.rs 310-328, 330-348) and Outcome: «The decision table''s seven rows — MET (`server_verdict_rows`, under `run --unit`)».'
    fact: F8
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan.md
    near: 378
    section: '## 4. Unit Test Strategy'
    change: '**viola-state:** ... the pure liveness classifier fed ages directly (no clock), the pure server verdict decision table, Unix strict-modes and Windows strict-modes over synthetic SDDL descriptors, and the instance check.'
    sidecar: 'test-plan §4 viola-state broad line: the server verdict decision table added.'
    rationale: 'The broad line the viola-state unit group details; report Symbols / APIs (`ServerVerdict`, `server_verdict`) and Outcome (`server_verdict_rows`).'
    dependent-of: D-new-detail
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 65
    section: '## 4. Unit Test Strategy'
    change: 'Add under root bin: - The server check before any frame (chunk 2026-10-26-server-verification-before-any-frame): `request_refuses_an_unverified_server_inside_its_dispatch_span` in each of `src/cmd/{send,wait,last,answer,pause,release}.rs` and `hook_refuses_an_unverified_server_inside_hook_handle` (`src/cmd/hook.rs`) assert the refusal inside the verb''s dispatch span with no frame, all through the shared test-only `cmd::client::refused_inside` helper (`assert_refused_inside`, compiled out on macOS); `unreachable_logs_its_cause` (`src/cmd/client.rs`) logs each closed `Unreachable` cause (`instance-dead` · `server-verify-failed`, `during` `connect` for the server cause).'
    sidecar: 'unit-test-strategy root bin: the per-verb captured-span cases, the refused_inside helper and unreachable_logs_its_cause added.'
    rationale: 'Report Symbols / APIs (`Unreachable`, `connect`, `refused_inside` 227-324), New text, by line (each verb''s `request_refuses_an_unverified_server_inside_its_dispatch_span`, hook.rs 951-998, client.rs 434-473) and Deviations 6 (seven captured-span cases, each red with the check removed).'
    fact: F10
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 65
    section: '## 4. Unit Test Strategy'
    change: 'Add under root bin: - The hook''s server check (`src/cmd/hook.rs`): `ask_writes_no_frame_to_a_server_that_is_not_the_recorded_wrapper` (the dialog path writes no frame and emits no decision), and a SessionEnd whose server check fails takes the direct append (`append_session_end`) in the hook unit case''s mismatch row. The unit impostor varies the recorded start time (a second early; an unparseable `started_at`) rather than recording another live pid; the other-live-pid shape is the integration rows''.'
    sidecar: 'unit-test-strategy root bin: the hook unit impostor and the SessionEnd fallback on a failed check added.'
    rationale: 'Report Symbols / APIs (`src/cmd/hook.rs`: «a SessionEnd whose check fails takes `append_session_end`»), New text, by line (hook.rs 919-949), Deviations 3 and Outcome (arch): «A SessionEnd whose check fails takes the direct append — MET (the hook unit case''s mismatch row)».'
    fact: F11
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan.md
    near: 386
    section: '## 4. Unit Test Strategy'
    change: '**root bin:** ... the `send` and `answer` refusal ordering, the closed exit-21 cause and the server check refused inside each verb''s and the hook''s span, human-mode C0/C1 escaping, and the clock-driven dialog hold.'
    sidecar: 'test-plan §4 root bin broad line: the server check''s unit cases added.'
    rationale: 'The broad line the root-bin unit group details; report Symbols / APIs (`Unreachable`, `connect`, `refused_inside`).'
    dependent-of: D-new-detail
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 47
    section: '## 5. Integration Test Strategy'
    change: '- a server pid / start-time mismatch versus `snapshot.json` gives CLI exit 21 and the hook exits 0 with no body, on Linux and Windows (as landed, chunk 2026-10-26-server-verification-before-any-frame: the CLI half in `tests/security_negatives_server_identity.rs`, the hook half the impostor rows of `tests/hook_fail_open.rs`; macOS reports no peer pid, so its server check stays the recorded gap and the euid + directory check is owed to "Unix endpoint and home hardening"). The MCP half is `crates/viola-mcp/tests/handlers.rs`, driving the transport-free `Tools` handle, no rmcp client; `cross_exit_causes.rs` does not exist.'
    sidecar: 'integration-test-strategy Wrapper channel: the server-mismatch cases'' homes as landed; the MCP half in handlers.rs, not cross_exit_causes.rs.'
    rationale: 'Report Spec claims disproved: «integration-test-strategy.md:47 ... the case landed as `handlers_frame_tools_write_no_frame_to_a_server_that_is_not_the_recorded_wrapper` in `crates/viola-mcp/tests/handlers.rs` (359-403), driving the transport-free `Tools` handle, no rmcp client; `cross_exit_causes.rs` does not exist» (pattern `tests`); Retired: macOS stays on the recorded gap; P5 card row 3.'
    basis: .andromeda/test-plan-detail/integration-test-strategy.md:47
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 47
    section: '## 5. Integration Test Strategy'
    change: 'Add after it: - the MCP impostor case `handlers_frame_tools_write_no_frame_to_a_server_that_is_not_the_recorded_wrapper` (`crates/viola-mcp/tests/handlers.rs`, a held fake-agent process as the recorded pid): MCP `send`, `wait`, `last` and `answer` against a server that is not the recorded wrapper each get `isError` with `{"error":"instance-unreachable","detail":null}` and text `refused: instance-unreachable`; no frame reaches the server and no log line is written.'
    sidecar: 'integration-test-strategy Wrapper channel: the MCP impostor handler case and its signals added.'
    rationale: 'Report Outcome (security): «MCP `send` / `wait` / `last` / `answer` against the impostor: `isError`, `{"error":"instance-unreachable","detail":null}`, text `refused: instance-unreachable`, no frame — MET (`binary(handlers)`)»; Schema / config: «MCP writes no line for the mismatch»; New text, by line (handlers.rs 359-403, the `Held` helper 142-163).'
    fact: F7
  - detector: D-tests-count-literals
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 235
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: 'Replace «Steps 1–3 stay unbuilt ...; the five other snapshot readers taking the replay are owed to "Server verification before any frame" (the review card the founder approved).» with: Steps 1–3 stay unbuilt while `run`''s collision check calls `read_snapshot`. The three other snapshot readers (`run`''s collision check, `src/cmd/hook.rs` once, `src/cmd/hook/statusline.rs`; the rule: `read_snapshot(` under the root `src/`, outside test modules) taking the replay, and what a verified client does with an unreadable or newer snapshot, are re-pinned as one CARRY to the route entry "Home and code-bearing file integrity", held for the founder''s own word: "Server verification before any frame" closed without them (P5 review card 1 of chunk 2026-10-26-server-verification-before-any-frame; fork 1 answered option 1, keep every reader on `read_snapshot`, PROVISIONAL by the founder-delegated overseer).'
    sidecar: 'e2e E5 As landed: the other snapshot readers are three, not five, and their replay move is re-pinned to "Home and code-bearing file integrity", held for the founder''s word (fork 1 PROVISIONAL).'
    rationale: 'Report Counts / qualifiers moved (pattern `readers`, site e2e-test-strategy.md:235: measured three, `src/cmd/hook.rs:204`, `src/cmd/hook/statusline.rs:142`, `src/cmd/run.rs:379`) and Spec claims disproved (pattern `owner`): «this entry closes without it; the move is re-pinned to "Home and code-bearing file integrity", held for the founder''s word (P5 card row 1, fork 1 PROVISIONAL)»; Decisions & corrections (inputs#I3, inputs#I4).'
    basis: .andromeda/test-plan-detail/e2e-test-strategy.md:235
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan.md
    near: 590
    section: '## 6. E2E Test Strategy'
    change: '**As landed: in part.** The replay''s product readers (`viola revive`, `viola list`), its budget and link legs and its `list --json` leg are covered; steps 1–3 stay unbuilt while `run`''s collision check calls `read_snapshot`, re-pinned to the route entry "Home and code-bearing file integrity" and held for the founder''s own word (fork 1 PROVISIONAL); "Server verification before any frame" closed without them.'
    sidecar: 'test-plan §6 E5 As landed: the reader move''s owner re-pinned to "Home and code-bearing file integrity".'
    rationale: 'Report Spec claims disproved: «The reader move''s owner: test-plan.md:590 «owed to the route entry "Server verification before any frame"» ... re-pinned to "Home and code-bearing file integrity", held for the founder''s word (P5 card row 1, fork 1 PROVISIONAL)».'
    dependent-of: D-tests-count-literals
  - detector: D-tests-count-literals
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 242
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: 'Replace «As landed it holds the 11-case matrix,» with «As landed it holds the fail-open matrices `hook_fails_open_silently_within_the_spine_bound` and `hook_dialog_tier_fails_open_silently` (their `#[case]` rows the count, which no master states),» — the rest of the sentence unchanged.'
    sidecar: 'e2e security sweep: the hook fail-open matrix named by its functions, the 11-case count dropped.'
    rationale: 'Report Counts / qualifiers moved (pattern `matrix11`): 11 `#[case]` rows at the base, 12 off macOS now; «:242 states «the 11-case matrix», a count the masters do not state per the Tier 1 rule — name the functions».'
    basis: .andromeda/test-plan-detail/e2e-test-strategy.md:242
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 242
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: 'Add after the matrix sentence: The impostor rows (chunk 2026-10-26-server-verification-before-any-frame), each under `cfg_attr(not(target_os = "macos"), …)`: `case::impostor` (`stop`) in `hook_fails_open_silently_within_the_spine_bound`, and `question_impostor` and `permission_impostor` in `hook_dialog_tier_fails_open_silently`, against a same-user server on the recorded endpoint whose pid is not the recorded wrapper''s: each exit 0, empty streams, under the 1.0 s `Instant` bound, no frame reaching the server, and one `hook-decision` line with `detail:"server-verify-failed"` (the dialog path after `hook-invoked{corr:null}`, with `decision_emitted:false`).'
    sidecar: 'e2e security sweep: the hook fail-open matrices'' three impostor rows added.'
    rationale: 'Report Counts / qualifiers moved (`case::impostor`, `question_impostor`, `permission_impostor`, the macOS gate), Schema / config (the dialog path''s `hook-invoked` then `hook-decision`) and Outcome: «The hook''s `stop`, `pre-tool-use`, `permission-request` against the impostor: exit 0, empty streams, `< 1.0 s`, no frame, `hook-decision` `server-verify-failed` — MET».'
    fact: F3
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 244
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: 'The hook fail-open matrix cases are oversize stdin, malformed JSON, clap error, channel failure, forced panic, missing `VIOLA_NAME` and, on Linux and Windows, a server that is not the recorded wrapper (`server-verify-failed`). The sweep also covers the `--home` strict-modes refusals.'
    sidecar: 'e2e security sweep: the fail-open cause list gains the server-verification mismatch.'
    rationale: 'Report Counts / qualifiers moved (pattern `matrix11`): «:244 list the matrix''s causes without the server-verification mismatch»; the impostor rows are gated off macOS.'
    basis: .andromeda/test-plan-detail/e2e-test-strategy.md:244
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 244
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: 'Add after the cause list: Not a refusal case: a hook reaching an exiting wrapper''s own Windows pipe (ci#36532038635) passes the server check by design, since the exiting wrapper is the recorded process; measuring whether sysinfo still lists a terminated Windows process whose pipe handle is open, and SessionEnd''s outcome in that window, is owed as one CARRY to the route entry "Windows-only live measurements".'
    sidecar: 'e2e security sweep: the exiting-wrapper Windows pipe window recorded as passing by design, its measurement owed to "Windows-only live measurements".'
    rationale: 'Report Cross-project claims, P5 review card 1 Beyond-the-plan row 2 (inputs#I4): «CARRY 2 re-pinned, unclosed ... the check passes there by design (the exiting wrapper is the recorded process). The measurement ... moves as one CARRY to "Windows-only live measurements"».'
    fact: F6
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan.md
    near: 598
    section: '## 6. E2E Test Strategy'
    change: '**Security sweep:** nextest runs the MCP schema-violation cases (each `invalid-params` with its field, no frame written) and the hook fail-open matrix — oversize stdin, malformed JSON, clap error, channel failure, a forced panic, a missing `VIOLA_NAME` and, on Linux and Windows, a server that is not the recorded wrapper, each exit 0 with no body — plus the `--home` strict-modes refusals; the panic seam fires only under the `fake-agent` feature and never reaches the release build.'
    sidecar: 'test-plan §6 Security sweep: the fail-open cause list gains the server-verification mismatch.'
    rationale: 'Report Counts / qualifiers moved (pattern `matrix11`, site test-plan.md:598).'
    dependent-of: D-cascade
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 261
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: 'Add after the matrix head: As landed (chunk 2026-10-26-server-verification-before-any-frame): `cross_exit_causes.rs` does not exist and no rstest table joins the causes; each built cause is a case of its own binary — the server-verification cause in `tests/security_negatives_server_identity.rs` (see Security control negatives), E1''s in `tests/cli_list.rs`, `revive`''s in `tests/cli_revive.rs`.'
    sidecar: 'e2e exit-cause matrix: cross_exit_causes.rs recorded as not existing; the built causes'' homes named.'
    rationale: 'Report Spec claims disproved: «test-plan.md:604 and test-plan-detail/e2e-test-strategy.md:261, :264 place it in the `cross_exit_causes.rs` matrix (rstest table, a last `hint:` line) — it landed as ... in `tests/security_negatives_server_identity.rs`»; «`cross_exit_causes.rs` does not exist (`ls tests/cross_exit_causes.rs` absent)» (pattern `tests`).'
    basis: .andromeda/test-plan-detail/e2e-test-strategy.md:261
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 264
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: '- server verification fails. On Linux and Windows: `snapshot.json` pid or start time does not match the listening process (a same-user impostor listener on the recorded endpoint); as landed, `security_negative_cli_verb_writes_no_frame_to_a_server_that_is_not_the_recorded_wrapper` and its control `security_negative_cli_verb_reaches_the_recorded_wrapper` in `tests/security_negatives_server_identity.rs` (`#![cfg(not(target_os = "macos"))]`), six verbs × two modes, not a `cross_exit_causes.rs` row. On macOS the client reads no peer pid (the recorded gap); its case is the next item''s.'
    sidecar: 'e2e exit-cause matrix: the server-verification cause''s Linux/Windows case as landed in security_negatives_server_identity.rs.'
    rationale: 'Report Spec claims disproved (pattern `tests`): the case landed as those two functions in `tests/security_negatives_server_identity.rs` (175-263, 265-289), six verbs × two modes; Retired: macOS stays on the recorded gap (no peer pid).'
    basis: .andromeda/test-plan-detail/e2e-test-strategy.md:264
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 264
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: 'Add after the server-verification cause: the macOS server-verification cases are not built, owed as one CARRY to the route entry "Unix endpoint and home hardening": the `#[cfg(target_os = "macos")]` case that loosens the verified per-user socket directory to 0755 and asserts exit 21, and the macOS client euid check; the Linux and Windows negatives compile out on macOS.'
    sidecar: 'e2e exit-cause matrix: the macOS 0755-directory case and the client euid check owed to "Unix endpoint and home hardening".'
    rationale: 'Report Spec claims disproved: «the macOS 0755-directory case is not built (P5 card row 3 → "Unix endpoint and home hardening")»; P5 review card 1 Beyond-the-plan row 3 (inputs#I4).'
    fact: F5
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 264
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: 'Add to the server-verification cause: its hint is its own, hint cause `server-verify`, printed exactly `the process on <name>''s endpoint is not its recorded wrapper; it was not contacted` (`src/human/hints.rs`, the design-system cli-send string byte for byte), distinct from the not-running hint of a dead instance; its log detail is its own, `process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}` at WARN in `cli-<name>.ndjson`; its `--json` document is the unchanged `{"v":1,"error":"instance-unreachable","detail":null}`.'
    sidecar: 'e2e exit-cause matrix: the server-verification cause''s own hint and log detail, its unchanged --json document.'
    rationale: 'Report Symbols / APIs (`src/human/hints.rs` 50-52: the `server-verify` cause''s exact string; `Unreachable::hint_cause` `not-running` | `server-verify`), the six verbs bullet (the unchanged document, the hint last on stderr) and Schema / config (the `process-exit` line).'
    fact: F4
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan.md
    near: 604
    section: '## 6. E2E Test Strategy'
    change: '**Exit-cause matrix:** one negative test per cause of exit 21 and of exit 1, each asserting the exit code, the `--json` document and a last `hint:` line that names its cause — a hint shared across causes fails the table; planned as the `cross_exit_causes.rs` table, which does not exist: the server-verification cause is a case of `tests/security_negatives_server_identity.rs` (Linux and Windows; the macOS case owed to "Unix endpoint and home hardening"), `revive`''s refusals are cases of `tests/cli_revive.rs`; driver-facing hints never contain `release`.'
    sidecar: 'test-plan §6 Exit-cause matrix: cross_exit_causes.rs recorded as not existing; the server-verification case''s home and the macOS owner named.'
    rationale: 'Report Spec claims disproved (pattern `tests`, site test-plan.md:604) and P5 card row 3.'
    dependent-of: D-cascade
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 297
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: 'Add: - **Server identity** (`#![cfg(not(target_os = "macos"))]`, IPC client-side server verification; chunk 2026-10-26-server-verification-before-any-frame). `tests/security_negatives_server_identity.rs`, a root test binary: `security_negative_cli_verb_writes_no_frame_to_a_server_that_is_not_the_recorded_wrapper` runs `send`, `wait`, `last`, `answer`, `pause` and `release`, each under `--json` and in human mode, against a same-user server on the recorded endpoint while the snapshot records a held process''s pid: exit 21, under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}` with stderr empty, in human mode the server-verify hint last on stderr, one `process-exit` line with `detail:"server-verify-failed"` and `during:"connect"` and no `channel-request`, no frame reaching the server, and no pid, start time, path or endpoint in any output or line (`digit_runs`).'
    sidecar: 'e2e security control negatives: the server-identity negative added.'
    rationale: 'Report Outcome (security, obs): the six verbs write no frame, exit 21, the exact document under `--json` (stderr empty), the hint last in human mode — MET (12 rows); one `process-exit{exit_code:21, detail:"server-verify-failed", during:"connect"}` at WARN, no `channel-request`, no pid / start time / path / endpoint — MET (`digit_runs`); New text, by line (security_negatives_server_identity.rs 175-263); Deviations 4 (the held process).'
    fact: F1
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 297
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: 'Add to the Server identity item: its control `security_negative_cli_verb_reaches_the_recorded_wrapper` runs each of the six verbs, in both modes, against the recorded wrapper itself, which receives one frame each.'
    sidecar: 'e2e security control negatives: the server-identity control added.'
    rationale: 'Report Outcome (security): «The control reaches the recorded wrapper, one frame each — MET: `security_negative_cli_verb_reaches_the_recorded_wrapper` (6 rows, both modes each)»; New text, by line (265-289).'
    fact: F2
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan.md
    near: 606
    section: '## 6. E2E Test Strategy'
    change: '**Security control negatives** (`security_negatives_*.rs`): one negative test per security-plan control that had none — ... umask-independent modes, single writers of `stamps.json` and `snapshot.json`, and, on Linux and Windows, a CLI frame verb writing no frame to a server that is not the recorded wrapper (beside its control).'
    sidecar: 'test-plan §6 Security control negatives: the server-identity negative added to the list.'
    rationale: 'The broad line the security control negatives group details; report Changes › Files (new `tests/security_negatives_server_identity.rs`) and Outcome (security).'
    dependent-of: D-new-detail
note: F12 (quality-gates), F13 (ci-integration), the fake agent's `--hold-stdout-internal` holder mode (test-data-fixtures) and the ETXTBSY planting fix in `tests/contract_fake_agent_drift.rs` are group 2's or another category's; body test-plan.md:435 and test-scope-summary.md:11 read no-change per the report's `gaps` sweep.
