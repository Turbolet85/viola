proposals:
  - detector: D-tests-count-literals
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 235
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: 'Replace «the five other snapshot readers taking the replay» with «the three product readers still on `read_snapshot` taking the replay (`run`''s collision check `src/cmd/run.rs:379`; `src/cmd/hook.rs:204`, one read shared by the hook''s event and dialog paths since chunk 2026-10-26-server-verification-before-any-frame; `src/cmd/hook/statusline.rs:142` — counted by `grep -rn ''read_snapshot('' src/` outside `#[cfg(test)]` modules at `775641fa`; the CLI verbs and `viola mcp` read through `viola_state::liveness::live_endpoint` since chunk 2026-10-17-mcp-server-for-drivers, so `src/cmd/client.rs` holds no call)».'
    sidecar: 'e2e-test-strategy E5: the snapshot readers still on `read_snapshot` restated as the measured three with their counting rule (pattern `readers`), not five — `src/cmd/client.rs` holds none since 2026-10-17 and `src/cmd/hook.rs`''s two reads were joined in one.'
    rationale: 'The report''s Changes › Counts / qualifiers moved (pattern `readers`) names this line among its 5 changed sites: the masters state five product callers, the tree reads three (`src/cmd/hook.rs:204`, `src/cmd/hook/statusline.rs:142`, `src/cmd/run.rs:379`); Spec claims disproved, second bullet; Deviations from intent 7 (hook read sites joined).'
    basis: .andromeda/test-plan-detail/e2e-test-strategy.md:235
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 235
    section: '## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery'
    change: 'Replace «are owed to "Server verification before any frame" (the review card the founder approved)» with «are owed to "Home and code-bearing file integrity", one CARRY there taken only after its `check_instance` step: "Server verification before any frame" closed without the move, re-pinned by its P5 review card 1 (the Viola overseer, founder-delegated, 2026-10-26 08:19Z, inputs#I4) on fork 1''s PROVISIONAL answer (option 1, every reader kept on `read_snapshot`, inputs#I3), held for the founder''s own word».'
    sidecar: 'e2e-test-strategy E5: the readers'' move onto the replay re-pinned from "Server verification before any frame" (closed without it) to "Home and code-bearing file integrity", labelled PROVISIONAL on fork 1 and held for the founder''s word.'
    rationale: 'The report''s Spec claims disproved, third bullet (pattern `owner`) names this line: the entry closes without the reader move, re-pinned by P5 card row 1 (Changes › Cross-project / external claims, inputs#I4); Decisions & corrections: fork 1 stays PROVISIONAL (inputs#I3) until the founder''s own word, so the text carries that label rather than stating the move as settled.'
    basis: .andromeda/test-plan-detail/e2e-test-strategy.md:235
  - detector: D-tests-count-literals
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 242
    section: '## 6. E2E Test Strategy'
    change: 'Replace «As landed it holds the 11-case matrix,» with «As landed it holds the fail-open matrix `hook_fails_open_silently_within_the_spine_bound` (its `case::impostor` row — `stop` against a server that is not the recorded wrapper, `server-verify-failed` — under `cfg_attr(not(target_os = "macos"), …)` since chunk 2026-10-26-server-verification-before-any-frame) and the dialog tier `hook_dialog_tier_fails_open_silently` (its `question_impostor` and `permission_impostor` rows, the same gate),» — the functions named, no case count.'
    sidecar: 'e2e-test-strategy Security sweep: «the 11-case matrix» replaced by the named functions `hook_fails_open_silently_within_the_spine_bound` and `hook_dialog_tier_fails_open_silently` with their impostor rows (off macOS); the master states no case count.'
    rationale: 'The report''s Changes › Counts / qualifiers moved (pattern `matrix11`): 11 `#[case]` rows at the base, 12 off macOS now, «:242 states «the 11-case matrix», a count the masters do not state per the Tier 1 rule — name the functions»; the dialog tier gains `question_impostor` and `permission_impostor` under the same gate (Deviations from intent 5).'
    basis: .andromeda/test-plan-detail/e2e-test-strategy.md:242
  - detector: D-tests-count-literals
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 244
    section: '## 6. E2E Test Strategy'
    change: 'Extend the cause list to «…forced panic, missing `VIOLA_NAME` and, on Linux and Windows, a server that is not the recorded wrapper (an impostor holding the recorded endpoint: exit 0, empty streams, no frame, `< 1.0 s`, one `hook-decision{detail:"server-verify-failed"}` line at WARN), its rows compiled out on macOS».'
    sidecar: 'e2e-test-strategy Security sweep: the hook fail-open causes gain the server-verification mismatch (Linux and Windows; macOS compiled out).'
    rationale: 'The report''s Counts / qualifiers moved (pattern `matrix11`) names :244 as listing the matrix''s causes without the server-verification mismatch; Outcome: the hook''s `stop`, `pre-tool-use`, `permission-request` against the impostor exit 0, empty streams, `< 1.0 s`, no frame, `hook-decision` `server-verify-failed`; Schema / config: the hook-decision line written at WARN.'
    basis: .andromeda/test-plan-detail/e2e-test-strategy.md:244
    dependent-of: D-tests-count-literals
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 264
    section: '## 6. E2E Test Strategy'
    change: 'After «…not asserted as a refusal.» add «As landed (chunk 2026-10-26-server-verification-before-any-frame): the Linux and Windows case is `security_negative_cli_verb_writes_no_frame_to_a_server_that_is_not_the_recorded_wrapper` with its control `security_negative_cli_verb_reaches_the_recorded_wrapper` in `tests/security_negatives_server_identity.rs` (Security control negatives: the six frame verbs under `--json` and in human mode, exit 21, the unchanged `instance-unreachable` document or the last `hint: the process on <name>''s endpoint is not its recorded wrapper; it was not contacted`), not a `tests/cross_exit_causes.rs` row; the macOS 0755-directory case is not built, a CARRY on "Unix endpoint and home hardening" (P5 review card 1 row 3, inputs#I4).»'
    sidecar: 'e2e-test-strategy Exit-cause matrix: the server-verification cause restated as landed in `tests/security_negatives_server_identity.rs` (Linux and Windows) with its distinct hint; the macOS 0755 case unbuilt, carried to "Unix endpoint and home hardening".'
    rationale: 'The report''s Spec claims disproved, fifth bullet (pattern `tests`): this line places the case in the `cross_exit_causes.rs` matrix, but it landed in `tests/security_negatives_server_identity.rs` (175-263, 265-289), six verbs × two modes, and the macOS 0755-directory case is not built (P5 card row 3); Symbols / APIs: the `server-verify` hint string; Outcome security criteria MET.'
    basis: .andromeda/test-plan-detail/e2e-test-strategy.md:264
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 261
    section: '## 6. E2E Test Strategy'
    change: 'After the bullet''s first sentence add «As landed, no `tests/cross_exit_causes.rs` binary exists (as read at chunk 2026-10-26-server-verification-before-any-frame): the causes built so far are cases of their own files — E1''s in `tests/cli_list.rs`, `revive`''s in `tests/cli_revive.rs`, server verification''s in `tests/security_negatives_server_identity.rs`.»'
    sidecar: 'e2e-test-strategy Exit-cause matrix: records that `tests/cross_exit_causes.rs` does not exist and where the built causes live.'
    rationale: 'The report''s Spec claims disproved, fourth and fifth bullets: `cross_exit_causes.rs` does not exist (`ls tests/cross_exit_causes.rs` absent); Expected amendments (test-plan §6) list e2e-test-strategy.md:261 as changed under pattern `tests`; the E1 and revive homes are this file''s own lines 266 and 274.'
    basis: .andromeda/test-plan-detail/e2e-test-strategy.md:261
    dependent-of: D-tests-coverage
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 297
    section: '## 6. E2E Test Strategy'
    change: 'Add after the Windows listener DACL denial bullet: «- **Server identity before any frame** (`#![cfg(not(target_os = "macos"))]`, IPC client-side server verification; chunk 2026-10-26-server-verification-before-any-frame). `tests/security_negatives_server_identity.rs` `security_negative_cli_verb_writes_no_frame_to_a_server_that_is_not_the_recorded_wrapper` serves the recorded endpoint from a test-owned `viola_channel::Server` while `snapshot.json` records another live process (a held fake agent) and runs each of `send`, `wait`, `last`, `answer`, `pause`, `release` under `--json` and in human mode: no frame reaches the server, exit 21 with the unchanged `{"v":1,"error":"instance-unreachable","detail":null}` (stderr empty) or the `unable` line with the `server-verify` hint last, one `process-exit{exit_code:21, detail:"server-verify-failed", during:"connect"}` and no `channel-request` line, no pid, start time, path or endpoint in any output; its control `security_negative_cli_verb_reaches_the_recorded_wrapper` records the serving process and reads one frame per verb in both modes. The hook half is `tests/hook_fail_open.rs`'' impostor rows (Security sweep) with the units `ask_writes_no_frame_to_a_server_that_is_not_the_recorded_wrapper` and `hook_refuses_an_unverified_server_inside_hook_handle` (a SessionEnd whose check fails takes the direct append); the MCP half is `crates/viola-mcp/tests/handlers.rs` `handlers_frame_tools_write_no_frame_to_a_server_that_is_not_the_recorded_wrapper` (the transport-free `Tools` handle; `send`, `wait`, `last`, `answer` each `isError` `{"error":"instance-unreachable","detail":null}`, text `refused: instance-unreachable`, no frame); the decision table is the unit `server_verdict_rows`, the peer pid `client_server_pid_is_the_serving_process` / `tokio_client_server_pid_is_the_serving_process`. macOS compiles it out (no peer pid; its euid and directory checks a CARRY on "Unix endpoint and home hardening").»'
    sidecar: 'e2e-test-strategy Security control negatives: new bullet for the server-identity negatives (CLI, hook, MCP halves) landed by chunk 2026-10-26-server-verification-before-any-frame, Linux and Windows.'
    rationale: 'The report''s Changes › Files (new `tests/security_negatives_server_identity.rs`), Harness / gate surface (new test binary, `#![cfg(not(target_os = "macos"))]`), Coverage of new surfaces (CLI, hook, MCP and `server_pid` rows) and Outcome (each security criterion MET on host, ubuntu-latest and windows-2025) bring a security-plan control negative this suite list does not yet hold; P5 card row 3 carries the macOS checks.'
    basis: viola-0.1.0/chunks/2026-10-26-server-verification-before-any-frame/report.md:70
note: 'The body''s own sites are not this file''s to edit — .andromeda/test-plan.md:590 (owner), :598 (fail-open causes), :604 (exit-cause matrix home) and :606 (security negatives list without server identity) — and .andromeda/test-plan-detail/integration-test-strategy.md:47 (the MCP impostor case''s home) is another category''s.'
