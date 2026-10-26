# Cascade compare — test-plan · wrap · 2026-10-26-server-verification-before-any-frame
E items 22 · held 12 · lost at classify 3 · lost at strong 1 · beside a placed fact 0 · cascade only 3 · listed - · body 0 · unplaced 0

## Lost — the full read carried these and the cascade did not; the wrap validates each, marked «(sampler)»

### mocking-stubbing-discipline#2.4 · lost at classify · .andromeda/test-plan-detail/mocking-stubbing-discipline.md:19
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/mocking-stubbing-discipline.md
    near: 19
    section: '## 8. Mocking & Stubbing Discipline'
    change: '| Fake servers | interprocess 2.4.4 server API (squatter listener; impostor listener); `viola-fake-agent` as the fake CLI; windows-sys 0.61.2 `CreateNamedPipeW` (Windows only: test-owned control pipes) | The squatter binds the computed endpoint before `run`; in the listener-admission negatives the test-owned control pipes (accept-remote, reject-remote, the default descriptor) stand beside the real `viola_channel::Server::bind` listener, and `ImpersonateAnonymousToken` stands in for a foreign principal; in the server-verification negatives (as landed at chunk 2026-10-26-server-verification-before-any-frame, Linux and Windows only, compiled out on macOS) an impostor, a test-owned `viola_channel::Server::bind` listener beside a snapshot that records another live process (root tests: the fake agent holder mode `--hold-stdout-internal`; `crates/viola-mcp/tests/handlers.rs`: its own `Held` child), stands in for an endpoint served by a process that is not the recorded wrapper, each held process killed and reaped by a `Drop` guard (`tests/security_negatives_server_identity.rs`, the `Impostor` of `tests/hook_fail_open.rs`, `handlers_frame_tools_write_no_frame_to_a_server_that_is_not_the_recorded_wrapper`); the hook unit impostor varies the recorded start time instead (a second early, or an unparseable `started_at`), a root-bin unit test having no portable second process; no HTTP fake servers |'
    sidecar: 'Fake servers row: the server-verification impostor (a test-owned listener beside a snapshot recording another held live process, reaped by a Drop guard; the hook unit case varying the recorded start time) added as landed at chunk 2026-10-26-server-verification-before-any-frame.'
    rationale: 'The report brings a new fake-server pattern to this category: Deviations 3 and 4 (the held process is the fake agent holder mode `--hold-stdout-internal`, killed and reaped by a `Drop` guard; the hook unit impostor varies the start time), New text by line for `tests/security_negatives_server_identity.rs` (`Held`, `Served`, `Server::bind`, `InstanceSnapshot`), `tests/hook_fail_open.rs` (`Held`, `Impostor`, `serve`) and `crates/viola-mcp/tests/handlers.rs` (`handlers_held_child_entry`, `Held`, 359-403); Harness / gate surface (the new binary is `#![cfg(not(target_os = "macos"))]`). The row names only the squatter and the admission control pipes.'
    basis: .andromeda/test-plan-detail/mocking-stubbing-discipline.md:19

### test-data-fixtures#1.5 · lost at classify · .andromeda/test-plan-detail/test-data-fixtures.md:14
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-data-fixtures.md
    near: 14
    section: "## 7. Test Data & Fixtures"
    change: "In the `claude agents --json` output row, add: a test that plants more than one fake-agent copy plants every copy (each recorded set's bin) one after another before any thread starts a child, because the tmpfs `target/e2e-home` defeats the hard link, so each copy is written, and a child forked by one thread inherits another thread's open write handle (ETXTBSY). In `fake_agent_agents_replay_matches_every_recorded_agents_set` (`tests/contract_fake_agent_drift.rs`) this was 6 of 40 standalone runs red before and 0 of 40 after (chunk 2026-10-26-server-verification-before-any-frame, evidence/etxtbsy-drift-test.md)."
    sidecar: "test-data-fixtures agents row: fake-agent copies are planted one after another before any thread spawns (the ETXTBSY fold, 6/40 red to 0/40), chunk 2026-10-26-server-verification-before-any-frame."
    rationale: "Report Deviations 1 and the Scope record (in-intent `tests/contract_fake_agent_drift.rs`, inputs#I5): two threads each copied the fake agent and one forked child inherited another's write handle; planting every set's bin one after another before the threads start brought it from 6/40 red to 0/40. New text adds `let planted: Vec<(String, TestHome, PathBuf, Vec<u8>)> = recorded_versions(&root)` (571-583)."
    basis: tests/contract_fake_agent_drift.rs:571

### test-scope-summary#1.1 · lost at strong · .andromeda/test-plan-detail/test-scope-summary.md:11
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 11
    section: "## 1. Test Scope Summary"
    change: "add one sub-item after line 11 (its own record; line 11 stays as is): `  - **As landed (server verification, chunk 2026-10-26-server-verification-before-any-frame):** the pid/start-time server verification line 11 names as testable is asserted on Linux and Windows: the six CLI frame verbs by `tests/security_negatives_server_identity.rs` (an impostor server gets no frame, exit 21, beside a control that reaches the recorded wrapper), the hook by `tests/hook_fail_open.rs`'s impostor rows (exit 0, no body, no frame), the MCP frame tools by `crates/viola-mcp/tests/handlers.rs` (`instance-unreachable`, no frame), and the decision table by the `viola-state` unit `server_verdict_rows`. This chunk's negatives compile out on macOS, where the client reads no peer pid (the recorded gap); the macOS euid and socket-directory cases are owed to \"Unix endpoint and home hardening\".`"
    sidecar: "test-scope-summary: viola-channel entity gains an as-landed record of the server verification's tests (Linux and Windows) and its macOS compile-out, owed to \"Unix endpoint and home hardening\" (chunk 2026-10-26-server-verification-before-any-frame)"
    rationale: "report Changes › Coverage of new surfaces (viola-0.1.0/chunks/2026-10-26-server-verification-before-any-frame/report.md:70-73) and Outcome (:104-110) name the landed tests; Retired (:23) and P5 card row 3 (:45-47) put macOS on the recorded gap with its cases owed to \"Unix endpoint and home hardening\". The sweep reads line 11 as still true (gaps, no-change, report.md:66), so this adds as-landed detail in the form line 11 already uses for the Windows listener (`tests/security_negatives_channel.rs`); it does not correct line 11."
    basis: .andromeda/test-plan-detail/test-scope-summary.md:11

### test-strategy#3.1 · lost at classify · .andromeda/test-plan-detail/test-strategy.md:58
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-strategy.md
    near: 58
    section: "## 2. Test Strategy"
    change: "The IPC vector (V1) row's Plan location names where the server pid / start-time mismatch trigger now lands: `| IPC vector (V1) | integration + E2E | §5 Module ↔ IPC and MCP (`crates/viola-mcp/tests/handlers.rs`'s impostor case, `instance-unreachable`, no frame); §6 Scenario 1 + Security control negatives (root `security_negatives_channel.rs`: the Windows listener's remote-client rejection and DACL denial; root `security_negatives_server_identity.rs`, compiled out on macOS: the six CLI frame verbs against a server that is not the recorded wrapper, exit 21 and no frame, beside its control reaching the recorded wrapper) + Security sweep (the hook fail-open matrices' impostor rows, off macOS) |`"
    sidecar: "test-strategy: the IPC vector (V1) trigger row names the server-verification negatives as landed — root `security_negatives_server_identity.rs` (CLI), the `hook_fail_open` impostor rows (hook), the `handlers.rs` impostor case (MCP); chunk 2026-10-26-server-verification-before-any-frame."
    rationale: "The report's Changes add the new test binary `tests/security_negatives_server_identity.rs` (Files; Harness / gate surface: root, `#![cfg(not(target_os = \"macos\"))]`) and its Coverage of new surfaces maps the V1 trigger's three outcomes (test-plan.md:184 — CLI exit 21 / MCP `instance-unreachable` / hook exit 0 no body) to `security_negatives_server_identity` (12 mismatch rows + 6 controls), `hook_fail_open`'s three impostor rows and `handlers_frame_tools_write_no_frame_to_a_server_that_is_not_the_recorded_wrapper`; the traceability row for V1 names only `security_negatives_channel.rs`, so the trigger's newly built cases have no plan location in this category."
    basis: viola-0.1.0/chunks/2026-10-26-server-verification-before-any-frame/report.md:70

## Held

- ci-integration#1.1
- e2e-test-strategy#11.1
- e2e-test-strategy#12.1
- e2e-test-strategy#14.1
- e2e-test-strategy#15.1
- integration-test-strategy#3.1
- mocking-stubbing-discipline#2.3
- quality-gates-coverage-targets#3.1
- test-data-fixtures#1.1
- unit-test-strategy#1.1
- unit-test-strategy#1.2
- unit-test-strategy#2.1

## Cascade only

- test-data-fixtures#4.1
- test-strategy#1.1
- test-strategy#2.1
