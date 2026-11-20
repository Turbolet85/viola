@task_prose part=session
The session made a question first raised by a PermissionRequest hook (with no armed PreToolUse) an answerable dialog: it is held like any dialog, and the driver's answer goes back as a PermissionRequest decision body with behavior allow and updatedInput (the tool input plus the answers and annotations).
It retired the rule that left such a question to the human; a human key, an expired hold or an unverified CLI still leave it to the human, and a late answer is refused unknown-dialog, exit 13.
It added a 27th relied ledger row, permission-request-question, with its own verify probe (the sixth closed probe id, permission-question-first-option) through a fourth compiled prompt pasted last in verify Run C, so verify step lines read [NN/27] and the all-pass summary 27 pass.
It re-recorded the 2.1.328 fixture set live (24 files replaced, 3 new question-permission-1 payloads), moved the one default and recorded CLI version from 2.1.287 to 2.1.328, made 2.1.328 the only stamped set and the older sets drift-only (still replayed).
Measured: unit, integration, E2E and pre-push green, the live rig start green at 27/27; CI attempt 1 was red on one wheel test (a send into a turn left running accepted, not refused turn-running), green on the one allowed re-run, its cause left open.

@task_file n=1 path=crates/viola-agent-claude/src/dialog.rs
what: the dialog decision body mapping — the hook decision body for each dialog kind and event, now the PermissionRequest question body (allow with updatedInput)
@task_file n=2 path=crates/viola-agent-claude/src/ledger.rs
what: the ledger of relied rows and their verify probe checks, the compiled probe prompts, the closed probe ids and the stamped and drift-only fixture set lists
@task_file n=3 path=crates/viola-agent-claude/src/ledger
what: the ledger unit cases — row checks, probe framing, stamp cases — gaining the permission-request-question pass case and its five fail cases
@task_file n=4 path=crates/viola-agent-claude/src/snapshots/viola_agent_claude__dialog__tests__question_answered_on_permission_request.snap
what: a new insta snapshot of the decision body for a question answered on PermissionRequest
@task_file n=5 path=src/run/dialog.rs
what: the wrapper's dialog hold — which dialogs are held for the driver and which are left to the human
@task_file n=6 path=src/cmd/verify.rs
what: viola verify — its step lines and summary line over every relied ledger row
@task_file n=7 path=src/cmd/verify/typed/runs.rs
what: viola verify's typed runs — Run C pastes the compiled dialog prompts, now a fourth, with a settle after its Stop
@task_file n=8 path=src/cmd/hook.rs
what: the viola hook exec-form entry points (its unit module only)
@task_file n=9 path=src/run/version_gate.rs
what: the unverified CLI version gate (its unit module only, over the new relied row)
@task_file n=10 path=crates/viola-e2e/src/rig/mod.rs
what: the standing live rig — its start labels, the kept stamp's row lines and its floor rows (every relied row but four)
@task_file n=11 path=crates/viola-e2e/src/harness/boot.rs
what: the harness boot command — its one default CLI version it stamps at
@task_file n=12 path=crates/viola-e2e/src/harness/run.rs
what: the harness run command — the local-live suite's expected pass output
@task_file n=13 path=src/bin/viola-fake-agent.rs
what: the fake agent — its default --cli-version
@task_file n=14 path=src/bin/viola-fake-agent/test_cases_opts.rs
what: the fake agent's command-line option unit cases (the default CLI version)
@task_file n=15 path=fixtures/claude/2.1.328
what: the recorded claude fixture set for CLI version 2.1.328, hook payloads named by hook event, now with the question-permission-1 payloads
@task_file n=16 path=fixtures/fake-scripts/path4.json
what: the fake agent turn script for critical path 4 (dialogs)
@task_file n=17 path=fixtures/fake-scripts/path4-permission.json
what: the fake agent turn script for critical path 4's permission-raised question
@task_file n=18 path=tests/cli_answer.rs
what: the viola answer integration suite — Path 4, a question raised on PermissionRequest logged once, woken and answered by id, a second answer refused unknown-dialog
@task_file n=19 path=tests/cli_verify.rs
what: the viola verify integration suite — step and summary line literals over the relied rows
@task_file n=20 path=tests/cli_verify/runs.rs
what: the viola verify Run C cases — the compiled prompts pasted and the rows they pass
@task_file n=21 path=tests/cli_verify/stamps.rs
what: the viola verify stamp and fixture-recording cases — the recorded names oracle
@task_file n=22 path=tests/contract_ledger_probes.rs
what: the ledger probe contract suite — every relied row passes over every stamped fixture set
@task_file n=23 path=tests/contract_fake_agent_drift.rs
what: the fake agent drift contract — the fake agent held to the recorded fixtures, each set replayed in order
@task_file n=24 path=tests/contract_fixture_hygiene.rs
what: the fixture hygiene walk over the recorded fixture sets
@task_file n=25 path=tests/support/verify.rs
what: the shared verify test helper — recorded dialog shapes fed to stamping verifies
@task_file n=26 path=tests/support/home.rs
what: the shared per-test home helper — the stamp it writes and its dialog rows
@task_file n=27 path=tests/support/fake.rs
what: the shared fake agent helper — the default CLI version it starts at
@task_file n=28 path=tests/support/start.rs
what: the shared wrapper start helper — the recorded fixture session id and CLI version
@task_file n=29 path=tests/chaos_revive.rs
what: a chaos test's fake agent boot at the one default CLI version (its version literal only)
@task_file n=30 path=tests/cli_send.rs
what: an integration suite's fake agent boot at the one default CLI version (its version literal only)
@task_file n=31 path=tests/cli_list.rs
what: an integration suite's fake agent boot at the one default CLI version (its version literals only)
@task_file n=32 path=tests/cross_list_rows.rs
what: a cross-surface test's fake agent boot at the one default CLI version (its version literal only)
@task_file n=33 path=tests/tui_passthrough.rs
what: a passthrough test's fake agent boot at the one default CLI version (its version literal only)
@task_file n=34 path=tests/run_cli.rs
what: a run integration test's fake agent boot at the one default CLI version (its version literal only)
@task_file n=35 path=tests/hook_events.rs
what: a hook integration test's fake agent boot at the one default CLI version (its version literals only)
@task_file n=36 path=tests/cli_wheel.rs
what: an integration suite's fake agent boot at the one default CLI version (its version literal only)
@task_file n=37 path=crates/viola-e2e/tests/mcp_tools.rs
what: an E2E suite's fake agent boot at the one default CLI version (its version literals only)
@task_file n=38 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the live rig rehearsals over the fake agent — their screen-fact literals for the default set's screens and the live pass count

@task_item n=1 part=touches
decision/dialog-answer-mapping: the question answer now maps on PermissionRequest too — allow with updatedInput carrying the tool input, answers and annotations — where before a PermissionRequest-raised question had no body.
@task_item n=2 part=touches
contract/hook-decision-body/question and contract/hook-decision/question: the question decision body gains its PermissionRequest form beside the PreToolUse one.
@task_item n=3 part=touches
decision/dialog-bodies-pinned-by-insta, decision/insta-snapshots-for-decision-bodies, decision/decision-bodies-snapshotted, decision/decision-bodies-as-insta-snapshots: a new decision body snapshot was added for the PermissionRequest question.
@task_item n=4 part=touches
decision/no-decision-body-without-driver-and-stamp, decision/decision-body-gate, decision/no-test-passing-unstamped-decision-body: a new decision body kind now passes through the gate (still only for a driver on a stamped CLI).
@task_item n=5 part=touches
contract/hook-dialog/unverified-null, decision/unverified-cli-gates-answer-only, decision/unverified-cli-send-only: a PermissionRequest-raised question is now held; on an unverified CLI it is still left to the human with no body.
@task_item n=6 part=touches
contract/dialog/event-logged-once, contract/dialog-events/logged-once, contract/dialog-event/logged-once, decision/pending-dialog-before-its-line: a question first raised by PermissionRequest is now raised as a pending dialog and logged once (Path 4).
@task_item n=7 part=touches
contract/viola-answer/unknown-dialog and decision/dialog-hold-expiry: a second or late answer to the PermissionRequest-raised question, or one after its hold expired, is refused unknown-dialog, exit 13.
@task_item n=8 part=touches
decision/human-never-blocked, decision/human-input-takes-the-wheel: the retired leave-it-to-the-human rule for a PermissionRequest question — a human key still leaves the question to the human.
@task_item n=9 part=touches
decision/ledger-row-verify-probe and decision/version-gate-needs-all-relied-rows: a new relied ledger row permission-request-question with its own probe; the gate now needs it too.
@task_item n=10 part=touches
decision/ledger-probe-contract-suite: the stamped set list now holds 2.1.328 alone and the drift-only list five sets; where the row states those lists it is stale.
@task_item n=11 part=touches
contract/verify-output/step-line, contract/verify-output/summary-line, contract/verify-output/last-line, contract/viola-verify/stamped-line: the relied row count moved 26 to 27 — step lines [NN/27], all-pass summary 27 pass 0 fail.
@task_item n=12 part=touches
contract/local-live-suite/pass: the live pass output now lists 27 rows.
@task_item n=13 part=touches
decision/harness-cli-version-matches-fake, contract/fake-agent-cli/cli-version, contract/test-harness/boot, contract/harness-boot/default-instances: the one default CLI version moved 2.1.287 to 2.1.328; boot stamps there.
@task_item n=14 part=touches
decision/fixtures-per-cli-version, decision/replay-every-cli-version-fixture, decision/relayed-fixtures-kept-superseded, decision/compat-tests-for-cli-versions, decision/multi-version-trigger-layers: the 2.1.328 set re-recorded and the only stamped one, the older sets kept drift-only and still replayed.
@task_item n=15 part=touches
decision/fixtures-recorded-by-live-verify, decision/hook-payloads-recorded-by-verify, decision/hook-payloads-recorded-locally, decision/fixture-named-by-hook-event, contract/claude-fixtures/payload-path: three new payloads PreToolUse, PermissionRequest and PostToolUse question-permission-1, recorded live.
@task_item n=16 part=touches
decision/fake-agent-matches-verify-fixtures, decision/fake-agent-contract-suite, decision/fake-agent-matches-recordings, decision/fixture-drift-byte-compare, decision/fake-agent-replays-recorded-bytes: the drift contract's oracle gains the three new names and a framed-exit check for a set holding them.
@task_item n=17 part=touches
decision/fixture-hygiene-walk, decision/fixtures-walked-for-hygiene, decision/recorded-hook-payloads-walked-at-run-time: the walked fixture set grew 28 to 31 files.
@task_item n=18 part=touches
decision/fake-agent-replay-for-stamps, decision/stamps-from-fake-agent-verify, decision/stamping-verify-passes-transcript, decision/guarded-paths-start-stamped: stamps now come from the 2.1.328 replay with the new row; a stamp without dialog replay reads 22 pass 5 fail.
@task_item n=19 part=touches
decision/standing-live-rig: the rig gains start labels PQ and PQ-2, its kept stamp prints the new row, its floor rows 22 to 23 (every relied row but four).
@task_item n=20 part=touches
decision/e2e-for-critical-paths: critical path 4 (dialogs) now runs on the PermissionRequest-raised question.
@task_item n=21 part=touches
contract/event-data/question and contract/question-event/option-details: a question raised on PermissionRequest now yields the question event; its data keys are unchanged.
@task_item n=22 part=touches
decision/zero-flakiness, decision/no-flaky-test-retries, decision/no-test-retries-or-sleeps, decision/build-fails-on-test-failure: a CI red on a wheel test (a send into a turn left running accepted instead of refused turn-running) went green on one re-run, its cause left open.
@task_item n=23 part=touches
contract/viola-send/turn-running and contract/viola-release/running-turn: the CI attempt-1 red sits on the path where a turn left running is cleared by pause then release.

@task_fact n=1 slug=unit-test-strategy
fact: A question first raised by PermissionRequest with no armed PreToolUse is held like any dialog and answered with a PermissionRequest decision body behavior allow and updatedInput (tool input plus answers and annotations); a human key, an expired hold or an unverified CLI still leave it to the human.
@task_fact n=2 slug=integration-test-strategy
fact: The relied ledger row permission-request-question is probed by its own closed probe id, permission-question-first-option, through a compiled prompt verify Run C pastes last, with a settle after its Stop.

@task_item n=1 part=untouched
CI workflow, supply-chain, cargo-deny, zizmor, SHA pinning and toolchain pin rows: no workflow, dependency or tool version moved.
@task_item n=2 part=untouched
Mutation gate, coverage floor and performance budget rows: no gate, threshold or budget moved.
@task_item n=3 part=untouched
Socket directory, strict modes, server identity and peer euid checks: no security control changed.
@task_item n=4 part=untouched
MCP tool schema, invalid-params fields and tools/list rows: no MCP tool, field or reply changed.
@task_item n=5 part=untouched
Process log line format, corr fields and log streams: no log event or field added.
@task_item n=6 part=untouched
Wrapper channel JSON-RPC errors, link and unlink methods and channel endpoint rows: no IPC method, endpoint or error changed.
@task_item n=7 part=untouched
Fuzz, proptest and parser property rows: no parser surface changed.
@task_item n=8 part=untouched
Harness pre-push, gate breach, secret scan, cleanup and status document rows: no harness command, status or verdict shape changed.
@task_item n=9 part=untouched
Permission and plan decision bodies and the PreToolUse question body: their shapes are unchanged.
