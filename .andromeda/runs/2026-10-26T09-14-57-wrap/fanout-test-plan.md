proposals:
  - detector: D-tests-coverage
    severity: warning
    section: '§6 E2E Test Strategy › Scenario: E5 — snapshot corruption → replay recovery (As landed)'
    change: 'The As-landed bullet keeps steps 1–3 unbuilt while the collision check of `run` calls `read_snapshot`, but names their owner as the route entry "Home and code-bearing file integrity" (re-pinned there by chunk 2026-10-26-server-verification-before-any-frame, held for the founder''s own word: fork 1 answered PROVISIONALLY, option 1, every reader kept on `read_snapshot`), no longer "Server verification before any frame", which closed without the reader move.'
    sidecar: 'test-plan §6 E5 As landed: the owner of steps 1–3 (the readers onto the replay) moved from "Server verification before any frame" to "Home and code-bearing file integrity", held for the founder''s word (fork 1 PROVISIONAL).'
    rationale: 'Report, Spec claims disproved by measurement (pattern `owner`): test-plan.md:590 «owed to the route entry "Server verification before any frame"» — this entry closes without it; the move is re-pinned to "Home and code-bearing file integrity" (P5 card row 1, fork 1 PROVISIONAL, inputs#I3 / inputs#I4); Expected amendments lists test-plan.md:590 as changed under `gaps`.'
    basis: '.andromeda/test-plan.md:590'
  - detector: D-tests-coverage
    severity: warning
    section: '§6 E2E Test Strategy › Non-path suites › Exit-cause matrix'
    change: 'The Exit-cause matrix bullet states, as landed, that the server-verification cause of exit 21 (a process on the endpoint that is not the recorded wrapper, Linux and Windows) is held by `tests/security_negatives_server_identity.rs` — `security_negative_cli_verb_writes_no_frame_to_a_server_that_is_not_the_recorded_wrapper` (six verbs × `--json` and human mode: exit 21, the unchanged `instance-unreachable` document, the server-verify hint last on stderr, no frame) with its control `security_negative_cli_verb_reaches_the_recorded_wrapper` — not a `cross_exit_causes.rs` row (that file is not built); the macOS case (the per-user socket directory loosened to 0755 → exit 21) is owed to the route entry "Unix endpoint and home hardening".'
    sidecar: 'test-plan §6 Exit-cause matrix: the server-verification exit-21 case recorded as landed in tests/security_negatives_server_identity.rs (no cross_exit_causes.rs); the macOS 0755 case owed to "Unix endpoint and home hardening".'
    rationale: 'Report, Spec claims disproved by measurement (pattern `tests`): test-plan.md:604 places the server-verification exit-cause case in the `cross_exit_causes.rs` matrix; it landed in `tests/security_negatives_server_identity.rs` (175-263, 265-289), six verbs × two modes, and `cross_exit_causes.rs` does not exist; the macOS 0755 case is not built (P5 card row 3 → "Unix endpoint and home hardening"). Expected amendments marks test-plan.md:604 changed.'
    basis: '.andromeda/test-plan.md:604; tests/security_negatives_server_identity.rs:175-263, :265-289'
  - detector: D-tests-coverage
    severity: warning
    section: '§6 E2E Test Strategy › Non-path suites › Security sweep'
    change: 'The hook fail-open matrix list gains, off macOS, a server that is not the recorded wrapper (the `stop` event and the `pre-tool-use` / `permission-request` dialog tiers: exit 0, empty streams, under 1.0 s, no frame, `hook-decision` `server-verify-failed`) beside oversize stdin, malformed JSON, clap error, channel failure, a forced panic and a missing `VIOLA_NAME`.'
    sidecar: 'test-plan §6 Security sweep: the hook fail-open matrix lists the server-verification mismatch (impostor rows, off macOS).'
    rationale: 'Report, Counts / qualifiers moved (pattern `matrix11`): `hook_fails_open_silently_within_the_spine_bound` gains `case::impostor` and `hook_dialog_tier_fails_open_silently` gains `question_impostor` / `permission_impostor`, gated off macOS; test-plan.md:598 lists the matrix causes without the server-verification mismatch (changed). Outcome: the hook impostor criterion MET by those rows. No count is written (Tier 1 rule: name the functions).'
    basis: '.andromeda/test-plan.md:598; tests/hook_fail_open.rs:182-190'
  - detector: D-tests-coverage
    severity: warning
    section: '§6 E2E Test Strategy › Non-path suites › Security control negatives'
    change: 'The `security_negatives_*.rs` control list gains the client-side server verification on Linux and Windows (`tests/security_negatives_server_identity.rs`: a CLI frame verb against a process that is not the recorded wrapper writes no frame, beside its control reaching the recorded wrapper); the macOS case is owed to "Unix endpoint and home hardening".'
    sidecar: 'test-plan §6 Security control negatives: the client-side server-verification control and its new binary security_negatives_server_identity named.'
    rationale: 'Report, Harness / gate surface: new test binary `security_negatives_server_identity` (root, `#![cfg(not(target_os = "macos"))]`); Retired: the server-identity half of the dated gaps closed on Linux and Windows, each verb now verifying the serving process before its first frame; the suite bullet enumerates one negative per security-plan control and does not name this one.'
    basis: 'tests/security_negatives_server_identity.rs:175-263'
note: 'No body count literal moved: the matrix11 count («the 11-case matrix») and the five-reader list sit only in detail files (e2e-test-strategy.md:235, :242, :244, :261, :264; unit-test-strategy.md:18; integration-test-strategy.md:47) — the cascade''s. Key files hold no touched claim; harness, status shape and log format unchanged (D-tests-obs-harness, D-tests-framework clean). test-plan.md:46, :184, :250, :435, :449, :767 read still true.'
