@task_prose part=session
A question first raised by PermissionRequest is no longer left unanswerable: it is held like any dialog and the driver's answer goes back as decision allow with updatedInput (the tool input plus answers and annotations); the unanswerable rule is retired.
A human key, an expired hold or an unverified CLI still leave such a question to the human; a late answer, or one after an expired hold, is refused unknown-dialog, exit 13.
The capability ledger gains a 27th relied row, permission-request-question, checked by viola verify's Run C through a fourth compiled prompt and a sixth closed capture probe id, permission-question-first-option; verify step lines read [NN/27] and the all-pass summary 27 pass 0 fail.
The recorded and default claude CLI version moved 2.1.287 to 2.1.328: the fixture set re-recorded (31 files, three new question-permission captures), the stamped set narrowed to 2.1.328 and the older sets kept as drift-only; version literals swept across the test suites, including the cross-list test.
Measured: a live verify round green at 27/27 with the kept stamp unmoved; pre-push green; CI attempt 1 red on one wheel test (a send into a running turn accepted where turn-running exit 13 was expected), the one allowed re-run green, the red left open with its cause unread.
No IPC method, socket, env var, log event, log field, config key or crate added.
@task_file n=1 path=crates/viola-agent-claude/src/dialog.rs
what: the claude adapter's dialog mapping: the hook decision body each dialog kind and hook event answers with, edited
@task_file n=2 path=crates/viola-agent-claude/src/ledger.rs
what: the capability ledger: the relied rows, their checks over recorded hook captures, the closed probe ids and probe bodies of the hidden capture arms, edited
@task_file n=3 path=src/run/dialog.rs
what: the run wrapper's dialog handling: the hold, the expiry, the answer route and which dialogs fall back to the human, edited
@task_file n=4 path=src/cmd/verify.rs
what: viola verify: the step lines and summary line over the relied ledger rows, edited
@task_file n=5 path=src/cmd/verify/typed/runs.rs
what: viola verify's typed PTY probe runs, Run C pasting the compiled dialog prompts, edited
@task_file n=6 path=crates/viola-e2e/src/rig/mod.rs
what: the live rig (test-only): its arms and labels, the kept stamp read and the floor of relied rows, edited
@task_file n=7 path=crates/viola-e2e/src/harness/boot.rs
what: the agent-run harness boot, its default recorded CLI version, edited
@task_file n=8 path=crates/viola-e2e/src/harness/run.rs
what: the agent-run harness run, its expected live verify pass lines, edited
@task_file n=9 path=src/bin/viola-fake-agent.rs
what: the fake agent (test-only), its default CLI version, edited
@task_file n=10 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: the fake agent's option unit cases, edited
@task_file n=11 path=crates/viola-agent-claude/src/ledger/test_cases.rs
what: capability ledger unit cases, edited
@task_file n=12 path=crates/viola-agent-claude/src/ledger/test_cases_checks.rs
what: capability ledger row-check unit cases, the new row's pass and fail cases, edited
@task_file n=13 path=crates/viola-agent-claude/src/ledger/test_cases_framing.rs
what: capability ledger framing and probe-body unit cases, edited
@task_file n=14 path=crates/viola-agent-claude/src/ledger/test_cases_stamps.rs
what: capability ledger stamp unit cases, edited
@task_file n=15 path=src/cmd/hook.rs
what: viola hook, its test module only, edited
@task_file n=16 path=src/run/version_gate.rs
what: the run version gate over the ledger stamps, its test module only, edited
@task_file n=17 path=crates/viola-agent-claude/src/snapshots/viola_agent_claude__dialog__tests__question_answered_on_permission_request.snap
what: the snapshot of the PermissionRequest question decision body, added
@task_file n=18 path=fixtures/claude/2.1.328
what: the recorded claude fixture set (scrub-required hook captures), 24 files replaced and three added
@task_file n=19 path=fixtures/fake-scripts/path4.json
what: the fake agent's script for the dialog path, edited
@task_file n=20 path=fixtures/fake-scripts/path4-permission.json
what: the fake agent's script for the PermissionRequest dialog path, edited
@task_file n=21 path=tests/cli_answer.rs
what: integration tests of viola answer over the dialog path, edited
@task_file n=22 path=tests/cli_verify.rs
what: integration tests of viola verify's step and summary lines, edited
@task_file n=23 path=tests/cli_verify/runs.rs
what: integration tests of viola verify's typed runs, edited
@task_file n=24 path=tests/cli_verify/stamps.rs
what: integration tests of viola verify's stamp records, edited
@task_file n=25 path=tests/contract_ledger_probes.rs
what: the ledger probe contract suite over every stamped fixture set, edited
@task_file n=26 path=tests/contract_fake_agent_drift.rs
what: the fake agent drift contract suite replaying each fixture set, edited
@task_file n=27 path=tests/contract_fixture_hygiene.rs
what: the fixture hygiene contract suite over the recorded claude fixtures, edited
@task_file n=28 path=tests/support/verify.rs
what: test support for verify's recorded captures, edited
@task_file n=29 path=tests/support/home.rs
what: test support for test homes and their stamps, edited
@task_file n=30 path=tests/support/fake.rs
what: test support for the fake agent, its recorded CLI version, edited
@task_file n=31 path=tests/support/start.rs
what: test support for instance starts, its recorded CLI version, edited
@task_file n=32 path=tests/chaos_revive.rs
what: chaos tests booting the fake agent at the recorded CLI version, edited
@task_file n=33 path=tests/cli_send.rs
what: integration tests booting the fake agent at the recorded CLI version, edited
@task_file n=34 path=tests/cli_list.rs
what: integration tests of list output at the recorded CLI version, edited
@task_file n=35 path=tests/tui_passthrough.rs
what: integration tests booting the fake agent at the recorded CLI version, edited
@task_file n=36 path=tests/run_cli.rs
what: integration tests of viola run at the recorded CLI version, edited
@task_file n=37 path=tests/hook_events.rs
what: integration tests of hook events at the recorded CLI version, edited
@task_file n=38 path=tests/cli_wheel.rs
what: integration tests of the wheel at the recorded CLI version, edited
@task_file n=39 path=crates/viola-e2e/tests/mcp_tools.rs
what: E2E tests booting the fake agent at the recorded CLI version, edited
@task_file n=40 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the live rig rehearsals, their screen-fact literals and pass lines, edited
@task_item n=1 part=touches
A row holding that a question first raised by PermissionRequest is left to the human, gets null or is unanswerable is now stale: it is held and answered allow with updatedInput.
@task_item n=2 part=touches
The hook stdout contract per hook event (decision body only) gains the PermissionRequest question form: decision behavior allow with updatedInput carrying the answers.
@task_item n=3 part=touches
The dialog path log events and their order (hook-invoked, dialog-raised, hook-decision, dialog-answered) and their corr now also cover a question first raised by PermissionRequest.
@task_item n=4 part=touches
The dialog log fields for dialog-raised, dialog-answered and the hook lines: a new dialog kind and event pair flows through them, with no field added.
@task_item n=5 part=touches
Dialog content never logged: the driver's answers now travel inside the hook's stdout decision body as updatedInput; rows on dialog text and answers never logged rule over this.
@task_item n=6 part=touches
The unknown-dialog refusal on the dialog log line: a late answer, or one after an expired hold, on a PermissionRequest question is refused unknown-dialog, exit 13.
@task_item n=7 part=touches
Expired dialog returns to the human, and the unverified-cli answer refusal: both now also govern a PermissionRequest-raised question.
@task_item n=8 part=touches
The verify stdout step line and summary line: the relied row count moved 26 to 27 ([NN/27], 27 pass 0 fail); any row cell holding 26 is stale.
@task_item n=9 part=touches
Verify lines carry no content, and the ledger's path and stamps never logged: a new relied row and its step line, and the stamp gains the permission-request-question key.
@task_item n=10 part=touches
The hidden hook capture arms and their probe bodies (uninstrumented, writing no lines): a sixth closed probe id answers the permission question with the allow body.
@task_item n=11 part=touches
The verify PTY probe spawn subject and Run C: a fourth compiled prompt is pasted, with a settle after its Stop.
@task_item n=12 part=touches
Recorded claude fixtures, scrub-required and no telemetry source: the set re-recorded at 2.1.328 with three new captures under the hygiene suite.
@task_item n=13 part=touches
Real-CLI verify never runs in CI and CI verifies against the fake agent only: the live round ran on the host, CI ran the fake agent at the new default version.
@task_item n=14 part=touches
Re-run is no substitute for diagnostics, and no retry-once policy: a CI red was re-run once and went green; the red stays open with its cause unread, the job log holding the panic line alone.
@task_item n=15 part=touches
The turn-running send refusal: the CI red shows a send into a running turn accepted with exit 0 where turn-running exit 13 is expected; measured, not mended.
@task_item n=16 part=touches
The run log cli_version and cli_verified fields and the version gate over ledger stamps: the recorded and stamped CLI version is now 2.1.328 alone.
@task_fact n=1 slug=span-trace-coverage
fact: A question first raised by PermissionRequest with no armed PreToolUse is held and logged like any dialog (dialog-raised, hook-decision, dialog-answered); a human key, an expired hold or an unverified CLI leave it to the human.
@task_fact n=2 slug=pii-scrubbing-compliance
fact: The driver's answers to a question reach claude only inside the hook's stdout decision body as updatedInput; no log line carries them.
@task_item n=1 part=untouched
OTel, exporters, trace propagation, metric derivation and telemetry budgets: nothing in the telemetry pipeline changed.
@task_item n=2 part=untouched
Panic capture and error reporting rows: no panic hook, catch site or error chain handling changed.
@task_item n=3 part=untouched
Secret scan, artifact upload, retention and the schema and panic gates: no CI gate or upload rule changed.
@task_item n=4 part=untouched
Link and unlink, state recovery and snapshot replay, heartbeat liveness: none reached.
@task_item n=5 part=untouched
Socket-directory, server-verify and strict-modes refusals, and exit 21 cause words: no refusal code added or moved.
@task_item n=6 part=untouched
Log line format, levels, sinks, field names, the event enum and diagnostics file modes: no event or field added.
