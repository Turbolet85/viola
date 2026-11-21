@task_prose part=session
The session added the 29th relied capability-ledger row, cross-session-prefix, reading pass or fail only, and stamped it live on claude 2.1.328 (29/29).
viola verify now spawns a print-mode sender child, verify-sender (claude -p with --model haiku, --no-session-persistence, --allowedTools SendMessage and a sender plugin), whose prompt carries the CLI's inbox socket address for Run B's child; Run B now runs with a compiled settings literal that accepts cross-session inbound messages.
verify reads XDG_RUNTIME_DIR once, in memory only, to compose that uds address; the address reaches the sender's argv (and through it the model provider) alone, never a log line, stamp, fixture, error or step line of verify's own.
The hook capture arm gained a hidden names-only mode that files only closed-word Tool.k.json files (0600), and the stamp gained a cross_session_probe block of closed words and one latency number.
The 2.1.328 fixture set was re-recorded whole and grew from 32 to 33 files; the measured new cross-session fixture holds the recording host's runtime-dir path (with the uid), the sender pid and a seven-digit from-name number, which the scrub does not normalise.
Every test and harness verify now gets a short owner-only test-owned runtime dir, since the pre-push empty environment leaves XDG_RUNTIME_DIR unset; the bare-session-id sender form was retired in favour of the address form.
@task_file n=1 path=crates/viola-agent-claude/src/ledger.rs
what: edited; the capability ledger and its relied rows, the stamp merge, the fixture scrub and hygiene walk, the probe plugins and the verify probe prompts
@task_file n=2 path=crates/viola-agent-claude/src/lib.rs
what: edited; the Claude adapter's compiled literals (settings flag, uds scheme, CLI socket dir, suffix and path bound, allowed-tools flag)
@task_file n=3 path=crates/viola-agent-claude/src/hook.rs
what: edited; the Claude hook payload reader and the harness prefix literals that classify a prompt's origin
@task_file n=4 path=crates/viola-agent-claude/src/ledger/test_cases.rs
what: edited; unit tests of the ledger's capture documents, tool words and probe plugin hooks
@task_file n=5 path=crates/viola-agent-claude/src/ledger/test_cases_checks.rs
what: edited; unit tests of the ledger row checks, the uds address composition and the sender prompt
@task_file n=6 path=crates/viola-agent-claude/src/ledger/test_cases_framing.rs
what: edited; unit tests of the verify probe framing turns
@task_file n=7 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: edited; unit tests of the stamp merge and its measured blocks
@task_file n=8 path=src/cmd/hook.rs
what: edited; the viola hook command and its hidden capture arm, now with the names-only mode
@task_file n=9 path=src/cmd/verify.rs
what: edited; the viola verify command, its child set, its environment read and its probe directories
@task_file n=10 path=src/cmd/verify/typed.rs
what: edited; verify's typed run inputs, now holding the runtime dir in memory
@task_file n=11 path=src/cmd/verify/typed/runs.rs
what: edited; verify's typed runs and how Run B ends
@task_file n=12 path=src/cmd/verify/typed/turns.rs
what: edited; verify's probe turns, Run B's cross-session turn and the sender child spawn
@task_file n=13 path=src/run/version_gate.rs
what: edited; tests of viola run's version gate over the stamps
@task_file n=14 path=src/bin/viola-fake-agent.rs
what: edited; the test-only fake agent, its cross-session sender and receiver modes and its receipt
@task_file n=15 path=src/bin/viola-fake-agent/agents.rs
what: edited; the fake agent's sibling lookup, now by uds address
@task_file n=16 path=src/bin/viola-fake-agent/test_cases_modes.rs
what: edited; tests of the fake agent's sender and receiver modes
@task_file n=17 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: edited; tests of the fake agent's options over the recorded set
@task_file n=18 path=schemas/diag-line.v1.json
what: edited; the diagnostics line schema and its subject list, now with verify-sender
@task_file n=19 path=crates/viola-e2e/src/harness/boot.rs
what: edited; the harness boot stamp, now giving verify an owner-only runtime dir
@task_file n=20 path=crates/viola-e2e/src/harness/run.rs
what: edited; the harness run and its expected 29-row step lines
@task_file n=21 path=crates/viola-e2e/src/rig/mod.rs
what: edited; the standing live rig, its floor rows, labels and kept-stamp summary lines
@task_file n=22 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: edited; the live rig rehearsal cases and the env they carry
@task_file n=23 path=tests/cli_verify.rs
what: edited; integration tests of viola verify, including the cross-session row cases
@task_file n=24 path=tests/cli_verify/runs.rs
what: edited; integration tests of verify's runs
@task_file n=25 path=tests/cli_verify/stamps.rs
what: edited; integration tests of verify's stamps
@task_file n=26 path=tests/cli_fake_agent.rs
what: edited; tests of the fake agent's receipt keys
@task_file n=27 path=tests/cli_send.rs
what: edited; integration tests of viola send over recorded session ids
@task_file n=28 path=tests/chaos_revive.rs
what: edited; revive chaos tests over recorded session ids
@task_file n=29 path=tests/contract_diag_schema.rs
what: edited; the diagnostics schema contract test
@task_file n=30 path=tests/contract_fake_agent_drift.rs
what: edited; the fake agent drift contract against the recorded dialogs
@task_file n=31 path=tests/contract_fixture_hygiene.rs
what: edited; the recorded fixture hygiene walk contract test
@task_file n=32 path=tests/contract_ledger_probes.rs
what: edited; the ledger probes contract over every stamped set
@task_file n=33 path=tests/security_negatives_env_capture.rs
what: edited; the security negative tests of the env and hook capture arms, now with a names-only canary case
@task_file n=34 path=tests/support/home.rs
what: edited; the test home support, starting wrappers with a test-owned socket root
@task_file n=35 path=tests/support/start.rs
what: edited; the test start support over recorded session ids
@task_file n=36 path=tests/support/verify.rs
what: edited; the test verify helpers, now giving verify a test-owned runtime dir or none
@task_file n=37 path=fixtures/claude/2.1.328/UserPromptSubmit.cross-session-1.json
what: added; a recorded claude hook fixture of a cross-session arrival, holding the CLI's own cross-session prefix
@task_file n=38 path=fixtures/claude/2.1.328
what: replaced; the recorded claude 2.1.328 hook fixture set, re-recorded whole and scrubbed
@task_item n=1 part=touches
decision/verify-children-direct-spawn: verify spawns a new direct child, the print-mode sender verify-sender, whose -p prompt carries the composed uds address, and Run B's args gain --settings with the crossSessionInbound accept literal.
@task_item n=2 part=touches
decision/verify-trusted-runs-risk: Run B now runs under an accept-inbound settings literal, opening a cross-session inbound window of 30 s while the sender runs under a 120 s deadline.
@task_item n=3 part=touches
decision/hook-capture-arm and decision/capture-arms-isolated: the hidden capture arm gained --names-only, valid only with --capture, PreToolUse and PostToolUse only, filing only closed-word Tool.k.json files at 0600 and no payload byte.
@task_item n=4 part=touches
decision/verify-probe-captures-transient, decision/probe-artifacts-cleanup and decision/verify-probe-dirs-cleanup: a SIGKILL can leave the sender's closed-word Tool.k.json files and the sender plugin's two files behind.
@task_item n=5 part=touches
decision/verify-echo-probe-residuals and decision/verify-real-cli-transcripts-accepted: a new cross-session residual row belongs beside the echo residuals, and the transcripts row takes a pointer to it.
@task_item n=6 part=touches
decision/live-rig-kept-stamp-owner-only: its labels list gains XS and XS-2 with kept stamps cross-stamp.json and cross-stamp-2.json; the rig's summary lines gain cross-session-prefix, cross-session, sender-tools and send-result.
@task_item n=7 part=touches
decision/probe-prompts-synthetic: the sender's prompt texts are compiled, and the composed uds address is their one non-literal part.
@task_item n=8 part=touches
decision/new-env-read-needs-log-entry, decision/no-stored-secrets and decision/env-not-config-channel: verify now reads XDG_RUNTIME_DIR once, in memory, only to compose the address; its Decisions Log entry is the operator's answer X1 (a).
@task_item n=9 part=touches
The row stating XDG_RUNTIME_DIR is read for the socket directory (decision/socket-in-per-user-dir, decision/per-user-socket-directory and kin) is stale by omission: verify now reads it too, for the CLI's inbox socket, not viola's own.
@task_item n=10 part=touches
decision/messaging-credentials-are-secret and decision/no-messaging-secrets-in-outputs: the claude CLI's inbox socket address is now composed by verify and placed in the sender's prompt, reaching the model provider, and it appears in the committed cross-session fixture via the CLI's own prefix.
@task_item n=11 part=touches
decision/fixture-recording-scrubbed, decision/fixture-scrub-and-refusal, decision/fixture-hygiene and decision/fixtures-reviewed-before-commit: the scrub rewrites only home and user, so the runtime-dir path with the uid, the sender pid and a seven-digit from-name number pass into the committed fixture.
@task_item n=12 part=touches
contract/fixture-refusal-codes/absolute-path: the hygiene walk's absolute-path check reads only drive-letter, /home/, /Users/ and Users paths, so a runtime-dir path is not refused.
@task_item n=13 part=touches
decision/harness-prefix-literals: the four raw-start harness prefix literals now have ids and a lookup that names which matched; the prompt origin is rewritten over it, unchanged in result, and the cross-session row reads the arrival's prompt against them.
@task_item n=14 part=touches
decision/session-id-argv-closed-shape and decision/session-id-argv-no-shell: the sender prompt no longer takes a bare session id; it takes a uds address of a checked shape, one argv element.
@task_item n=15 part=touches
decision/verify-run-e-resume-target: Run B's cross-session turn no longer uses the resume target and is gated on the child pid and the composed address; Run E keeps the resume target.
@task_item n=16 part=touches
decision/bounded-readers-census, decision/max-frame-shared-cap and decision/reads-capped-at-max-frame: new readers, the names-only stdin read bounded at MAX_FRAME plus one and the sender's Tool.k.json reads bounded at MAX_FRAME.
@task_item n=17 part=touches
decision/env-probe-names-and-counts-only and decision/verify-stamp-first-screen-closed-words: the stamp gains a cross_session_probe block of closed words (outcome, prefix, sender_tools, send_result) and one number, latency_ms.
@task_item n=18 part=touches
decision/hermetic-child-environment and decision/pre-push-empty-environment-launch: the empty environment leaves XDG_RUNTIME_DIR unset, so every test and harness verify is given a short owner-only test-owned runtime dir under /tmp; the rig sets its own only when the host sets none.
@task_item n=19 part=touches
decision/tests-use-own-home and decision/test-homes-gitignored: tests and the harness now own a runtime dir beside their home, removed with it.
@task_item n=20 part=touches
decision/test-seams-fake-agent-only and decision/test-seams-compiled-out: the fake agent gained test-only cross-session sender and receiver modes, a sibling lookup by uds address, and pid and prompt keys in its start receipt.
@task_item n=21 part=touches
decision/live-rig-prompt-capture-closed and decision/live-rig-isolated-child: the XS and XS-2 rounds ran under the rig's private config and scratch home, reading sender tools and send result as closed words.
@task_item n=22 part=touches
decision/refusals-hold-no-paths-pids-ids and decision/no-paths-in-error-output: the composed address holds a path and a pid and is held to the sender's argv, entering no refusal, error or step line.
@task_item n=23 part=touches
decision/from-field-authenticates-nothing: under the accept literal, Run B's child may take an inbound cross-session frame; the peer key's no-token frame and what Run B does with an unvouched frame stay unmeasured.
@task_fact n=1 slug=data-protection
fact: Verify's cross-session probe residuals are accepted as measured: the address closes the target save the pid-reuse window (W3), W4 and T5, the SIGKILL closed-word files and sender plugin files, the peer key's unmeasured no-token frame, and the address reaching the model provider in the sender's prompt.
@task_fact n=2 slug=data-protection
fact: The committed cross-session fixture keeps, inside the CLI's own prefix, the recording host's runtime-dir path with the uid, the sender pid and a seven-digit from-name number; the runtime dir reaches no log, stamp or fixture verify writes of its own, only this recorded arrival through the scrub.
@task_fact n=3 slug=secret-management
fact: viola verify reads XDG_RUNTIME_DIR once, holds it in memory and uses it only to compose the claude CLI's inbox socket address for the sender's prompt; it enters no log line, stamp, fixture, error or step line.
@task_fact n=4 slug=input-validation
fact: The composed uds address is built only from an absolute runtime dir of bytes A-Z a-z 0-9 / . _ - with no dot or dot-dot component and a composed path of at most 103 bytes, else the row reads not-run; the sender prompt re-checks the shape and verify lists, reads and stats no directory to compose it.
@task_fact n=5 slug=input-validation
fact: The hook capture arm's names-only mode files only closed words, a tool class and a send result read from the success boolean alone, never a payload byte; an unknown shape reads unread.
@task_fact n=6 slug=secret-management
fact: Every test and harness verify is given a short owner-only test-owned runtime dir for the stamp's life, because the pre-push empty environment leaves XDG_RUNTIME_DIR unset.
@task_item n=1 part=untouched
The wrapper channel methods, channel frames and MCP tools and their contracts: no method, frame, param or tool changed.
@task_item n=2 part=untouched
Peer euid checks, server verification before a frame and viola's own socket directory checks, bind and chmod: viola's own channel socket is unchanged; the composed address is the claude CLI's inbox socket.
@task_item n=3 part=untouched
Dependency security, toolchains, CI workflows and release rows: no dependency, pin, workflow or release step changed.
@task_item n=4 part=untouched
Error-handling contracts, CLI exit codes, refusal codes and error documents: none added or moved.
@task_item n=5 part=untouched
Paste, send and answer text validation, the dialog free text set and the stdin non-editing list: untouched.
@task_item n=6 part=untouched
Revive, list, link and unlink rows and their contracts: untouched.
@task_item n=7 part=untouched
Strict-modes checks on the home tree, snapshot and stamps, and the version gate's strict stamps read: unchanged in rule.
@task_item n=8 part=untouched
Absences of a public listener, outbound network calls by viola itself, stored secrets and model API calls of its own: the sender is the unmodified claude CLI making its own call; viola adds none.
